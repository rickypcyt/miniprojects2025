use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};
use chrono;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};

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
        .max_depth(4) // Aumentamos un poco la profundidad
        .into_iter()
        .filter_entry(|e| {
            // Ignorar directorios ocultos (excepto .git) y node_modules
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

fn main() -> Result<(), Box<dyn Error>> {
    // Encontrar y mostrar todos los repositorios en el directorio raíz
    let home_dir = dirs::home_dir().expect("No se pudo encontrar el directorio home");
    println!("🔍 Buscando repositorios Git en {}...", home_dir.display());
    
    let all_repos = find_all_git_repos(&home_dir);
    println!("\n📚 Repositorios encontrados: {}", all_repos.len());
    for (i, repo) in all_repos.iter().enumerate() {
        println!("  {}. {}", i + 1, repo.display());
    }
    
    // Proceder con la sincronización del repositorio actual
    let current_dir = std::env::current_dir()?;
    
    if let Some(repo_path) = find_git_repo(&current_dir) {
        println!("\n📂 Repositorio actual: {}", repo_path.display());
        
        // Fetch changes
        println!("⏳ Obteniendo cambios remotos...");
        run_git_command(&repo_path, &["fetch", "origin"])?;
        
        // Check for unstaged changes
        let status = run_git_command(&repo_path, &["status", "--porcelain"])?;
        if !status.is_empty() {
            println!("📝 Hay cambios sin commitear, haciendo commit automático...");
            run_git_command(&repo_path, &["add", "."])?;
            run_git_command(&repo_path, &["commit", "-m", &format!("Auto-sync: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"))])?;
        }
        
        // Pull with rebase
        println!("⏳ Actualizando cambios locales...");
        run_git_command(&repo_path, &["pull", "--rebase", "origin"])?;
        
        // Push changes
        println!("⏳ Subiendo cambios...");
        run_git_command(&repo_path, &["push", "origin"])?;
        
        println!("✅ Sincronización completada");
    } else {
        println!("❌ No se encontró ningún repositorio git en el directorio actual o superiores");
    }
    
    Ok(())
}
