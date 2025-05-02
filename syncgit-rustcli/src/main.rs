use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};
use chrono;

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
    let current_dir = std::env::current_dir()?;
    
    if let Some(repo_path) = find_git_repo(&current_dir) {
        println!("📂 Repositorio encontrado: {}", repo_path.display());
        
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
