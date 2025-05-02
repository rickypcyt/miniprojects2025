use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
    io::{self, Write},
};
use chrono;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use colored::*;

#[derive(Debug)]
struct RepoStatus {
    path: PathBuf,
    needs_push: bool,
    needs_pull: bool,
    has_changes: bool,
    branch: String,
}

fn find_git_repo(current_dir: &Path) -> Option<PathBuf> {
    let mut dir = current_dir.to_path_buf();
    while dir.parent().is_some() {
        if dir.join(".git").exists() {
            return Some(dir);
        }
        dir = dir.parent()?.to_path_buf();
    }
    None
}

fn find_all_git_repos(root_dir: &Path) -> Vec<PathBuf> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.blue} {msg}").unwrap()
    );
    pb.set_message("Buscando repositorios...");

    let mut repos = Vec::new();
    for entry in WalkDir::new(root_dir)
        .max_depth(4)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            (name != ".git" && !name.starts_with('.')) && name != "node_modules"
        })
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() && entry.path().join(".git").exists() {
            repos.push(entry.path().to_path_buf());
        }
        pb.tick();
    }
    
    pb.finish_with_message("Búsqueda completada");
    repos
}

fn run_git_command(repo_path: &Path, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = Command::new("git")
        .args(args)
        .current_dir(repo_path)
        .output()?;
    
    if !output.status.success() {
        return Err(format!("Error: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn check_repo_status(repo_path: &Path) -> Result<RepoStatus, Box<dyn Error>> {
    // Obtener la rama actual
    let branch = run_git_command(repo_path, &["branch", "--show-current"])?;
    let branch = branch.trim().to_string();

    // Verificar si hay cambios sin commitear
    let status = run_git_command(repo_path, &["status", "--porcelain"])?;
    let has_changes = !status.is_empty();

    // Verificar si hay commits para hacer push
    let needs_push = !run_git_command(repo_path, &["rev-list", "@{u}..HEAD"])?.is_empty();

    // Verificar si hay cambios para hacer pull
    let needs_pull = !run_git_command(repo_path, &["rev-list", "HEAD..@{u}"])?.is_empty();

    Ok(RepoStatus {
        path: repo_path.to_path_buf(),
        needs_push,
        needs_pull,
        has_changes,
        branch,
    })
}

fn display_dashboard(repos_status: &[RepoStatus], show_all: bool) {
    println!("\n{}", "📊 Dashboard de Repositorios".bold().blue());
    println!("{}", "=".repeat(80).blue());

    let mut needs_attention = 0;
    let mut displayed = 0;

    for (i, status) in repos_status.iter().enumerate() {
        let needs_attention = status.needs_push || status.needs_pull || status.has_changes;
        
        // Solo mostrar si necesita atención o si show_all es true
        if needs_attention || show_all {
            let status_icons = format!(
                "{}{}{}",
                if status.needs_push { "⬆️ " } else { "" },
                if status.needs_pull { "⬇️ " } else { "" },
                if status.has_changes { "📝" } else { "" }
            );

            let status_text = if needs_attention {
                format!("{} {}", status_icons, "Necesita atención".yellow().to_string())
            } else {
                "✅ Sincronizado".green().to_string()
            };

            println!(
                "{}. {} ({}) - {}",
                displayed + 1,
                status.path.display(),
                status.branch.cyan(),
                status_text
            );
            displayed += 1;
        }
    }

    println!("{}", "=".repeat(80).blue());
    println!(
        "📈 {} repositorios necesitan atención de {} totales",
        needs_attention.to_string().yellow(),
        repos_status.len()
    );
    println!("{}", "Presiona 'h' para mostrar/ocultar repositorios sincronizados".dimmed());
}

fn sync_repository(repo_path: &Path) -> Result<(), Box<dyn Error>> {
    println!("\n📂 Sincronizando: {}", repo_path.display());
    
    // Fetch de cambios
    println!("⏳ Obteniendo cambios remotos...");
    run_git_command(repo_path, &["fetch", "origin"])?;
    
    // Verificar si hay cambios sin commitear
    let status = run_git_command(repo_path, &["status", "--porcelain"])?;
    if !status.is_empty() {
        println!("📝 Hay cambios sin commitear");
        run_git_command(repo_path, &["add", "-A"])?;
        
        // Pedir mensaje de commit
        println!("\n💭 Ingresa el mensaje del commit (o presiona Enter para usar el mensaje por defecto):");
        let mut message = String::new();
        io::stdin().read_line(&mut message)?;
        let message = message.trim();
        
        let commit_message = if message.is_empty() {
            format!("Auto-sync: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))
        } else {
            message.to_string()
        };
        
        println!("⏳ Haciendo commit...");
        run_git_command(repo_path, &["commit", "-m", &commit_message])?;
    }
    
    // Pull con rebase después de manejar los cambios locales
    println!("⏳ Actualizando cambios locales...");
    run_git_command(repo_path, &["pull", "--rebase", "origin"])?;
    
    // Verificar si hay commits para hacer push
    let needs_push = !run_git_command(repo_path, &["rev-list", "@{u}..HEAD"])?.is_empty();
    if needs_push {
        println!("⏳ Subiendo cambios...");
        run_git_command(repo_path, &["push", "origin"])?;
    }
    
    println!("✅ Sincronización completada");
    Ok(())
}

fn show_menu() -> Result<(), Box<dyn Error>> {
    loop {
        println!("\n{}", "=== SyncGit Menu ===".bold().blue());
        println!("1) Local");
        println!("2) Global Dashboard");
        println!("3) Salir");
        print!("\nSelecciona una opción (1-3): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => {
                // Modo Local
                if let Some(repo_path) = find_git_repo(&std::env::current_dir()?) {
                    sync_repository(&repo_path)?;
                } else {
                    println!("❌ No se encontró ningún repositorio git en el directorio actual o superiores");
                }
            },
            "2" => {
                // Modo Global (Dashboard)
                let home_dir = dirs::home_dir().expect("No se pudo encontrar el directorio home");
                println!("🔍 Buscando repositorios Git en {}...", home_dir.display());
                
                let all_repos = find_all_git_repos(&home_dir);
                println!("\n📚 Repositorios encontrados: {}", all_repos.len());

                let mut repos_status = Vec::new();
                let pb = ProgressBar::new(all_repos.len() as u64);
                pb.set_style(
                    ProgressStyle::default_spinner()
                        .template("{spinner:.blue} {msg}: {pos}/{len}")
                        .unwrap()
                );
                pb.set_message("Analizando repositorios");

                for repo in all_repos {
                    if let Ok(status) = check_repo_status(&repo) {
                        repos_status.push(status);
                    }
                    pb.inc(1);
                }
                pb.finish_with_message("Análisis completado");

                let mut show_all = false;
                loop {
                    display_dashboard(&repos_status, show_all);
                    
                    println!("\nOpciones:");
                    println!("  - Ingresa el número del repositorio para sincronizarlo");
                    println!("  - Presiona 'h' para mostrar/ocultar repositorios sincronizados");
                    println!("  - Presiona 'q' para volver al menú principal");
                    print!("\nSelecciona una opción: ");
                    io::stdout().flush()?;
                    
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let input = input.trim();
                    
                    match input {
                        "h" => {
                            show_all = !show_all;
                            continue;
                        },
                        "q" => break,
                        _ => {
                            if let Ok(index) = input.parse::<usize>() {
                                let displayed_repos: Vec<_> = repos_status.iter()
                                    .filter(|s| show_all || s.needs_push || s.needs_pull || s.has_changes)
                                    .collect();
                                
                                if index > 0 && index <= displayed_repos.len() {
                                    sync_repository(&displayed_repos[index - 1].path)?;
                                } else {
                                    println!("❌ Número de repositorio inválido");
                                }
                            } else {
                                println!("❌ Entrada inválida");
                            }
                        }
                    }
                }
            },
            "3" => {
                println!("👋 ¡Hasta pronto!");
                break;
            },
            _ => println!("❌ Opción inválida"),
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    show_menu()
}
