use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;
use std::path::{Path, PathBuf};

// Busca hacia arriba hasta encontrar el .git
fn find_git_root(mut dir: PathBuf) -> Option<PathBuf> {
    loop {
        if dir.join(".git").is_dir() {
            return Some(dir);
        }
        if !dir.pop() {
            return None; // Llegamos a la raíz del sistema y no hay repo Git
        }
    }
}

fn get_github_token() -> Option<String> {
    env::var("GITHUB_TOKEN").ok()
}

fn run(cmd: &str, args: &[&str]) -> bool {
    let mut command = Command::new(cmd);
    
    // Si es un comando git y tenemos token, lo usamos
    if cmd == "git" && get_github_token().is_some() {
        let token = get_github_token().unwrap();
        command.env("GITHUB_TOKEN", token);
    }
    
    let status = command
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(s) if s.success() => true,
        _ => {
            eprintln!("❌ Error al ejecutar: {} {:?}", cmd, args);
            false
        }
    }
}

fn main() {
    let current = env::current_dir().expect("❌ No se pudo obtener el directorio actual");
    let git_root = find_git_root(current.clone());

    let repo_path = match git_root {
        Some(path) => path,
        None => {
            eprintln!("❌ No estás dentro de un repositorio Git");
            return;
        }
    };

    let repo_name = repo_path.file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("")).to_string_lossy();

    println!("📁 Repositorio raíz: {}", repo_name);
    println!("🗂️  Ruta: {}", repo_path.display());
    println!("----------------------------------");

    println!("🔍 Estado del repositorio:");
    if !run("git", &["status", "-sb"]) {
        return;
    }

    println!("⬇️  Haciendo pull...");
    if !run("git", &["pull"]) {
        return;
    }

    println!("📦 Verificando cambios locales...");
    let has_changes = !Command::new("git")
        .args(&["diff", "--quiet"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false) || !Command::new("git")
        .args(&["diff", "--cached", "--quiet"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false) || !Command::new("git")
        .args(&["ls-files", "--others", "--exclude-standard"])
        .output()
        .map(|output| output.stdout.is_empty())
        .unwrap_or(true);

    if has_changes {
        if run("git", &["add", "."]) {
            println!("✅ Cambios añadidos");
        } else {
            return;
        }
    } else {
        println!("🟢 No hay cambios que añadir");
        return;
    }

    print!("✏️  Escribe tu mensaje de commit: ");
    io::stdout().flush().unwrap();
    let mut mensaje = String::new();
    io::stdin().read_line(&mut mensaje).unwrap();
    let mensaje = mensaje.trim();

    if mensaje.is_empty() {
        eprintln!("⚠️  El mensaje no puede estar vacío");
        return;
    }

    if !run("git", &["commit", "-m", mensaje]) {
        return;
    }

    println!("⬆️  Haciendo push...");
    if let Some(token) = get_github_token() {
        let remote_url = Command::new("git")
            .args(&["config", "--get", "remote.origin.url"])
            .output()
            .ok()
            .and_then(|output| String::from_utf8(output.stdout).ok())
            .map(|url| url.trim().to_string());

        if let Some(url) = remote_url {
            let auth_url = url.replace("https://", &format!("https://{}@", token));
            if !run("git", &["remote", "set-url", "origin", &auth_url]) {
                return;
            }
        }
    }
    
    run("git", &["push"]);
}
