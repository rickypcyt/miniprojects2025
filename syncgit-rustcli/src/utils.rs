use std::error::Error;
use std::time::Duration;
use std::path::PathBuf;
use std::process::Command;
use colored::*;

pub fn check_internet() -> bool {
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .unwrap_or_default();
    
    client.get("https://github.com").send().is_ok()
}

pub fn get_current_dir_name() -> Result<String, Box<dyn Error>> {
    let path = std::env::current_dir()?;
    Ok(path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string())
}

pub fn get_git_status(path: &str) -> Result<String, Box<dyn Error>> {
    let repo = git2::Repository::open(path)?;
    let statuses = repo.statuses(None)?;
    
    let mut status_info = String::new();
    
    // Contar archivos modificados, nuevos, etc.
    let mut modified = 0;
    let mut new = 0;
    let mut deleted = 0;
    
    for entry in statuses.iter() {
        let status = entry.status();
        
        if status.is_index_modified() || status.is_wt_modified() {
            modified += 1;
        } else if status.is_index_new() || status.is_wt_new() {
            new += 1;
        } else if status.is_index_deleted() || status.is_wt_deleted() {
            deleted += 1;
        }
    }
    
    if modified > 0 {
        status_info.push_str(&format!("📝 {} modificados, ", modified));
    }
    if new > 0 {
        status_info.push_str(&format!("➕ {} nuevos, ", new));
    }
    if deleted > 0 {
        status_info.push_str(&format!("🗑️ {} eliminados, ", deleted));
    }
    
    // Eliminar la última coma y espacio si hay información
    if !status_info.is_empty() {
        status_info.truncate(status_info.len() - 2);
    } else {
        status_info = "✅ Sin cambios".to_string();
    }
    
    Ok(status_info)
}

pub fn check_git_parent_and_changes() -> Result<Option<PathBuf>, Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    let mut current = current_dir.as_path();
    
    // Buscar el directorio padre que contiene .git
    while current.parent().is_some() {
        if current.join(".git").exists() {
            // Si estamos en un subdirectorio del repo
            if current != current_dir {
                println!("{} Estás en un subdirectorio del repositorio Git:", "ℹ️".blue());
                println!("  - Directorio actual: {}", current_dir.display());
                println!("  - Repositorio padre: {}", current.display());
                
                // Mostrar cambios usando git status
                let output = Command::new("git")
                    .arg("status")
                    .arg("-s")
                    .current_dir(current)
                    .output()?;
                
                if !output.stdout.is_empty() {
                    println!("\n{} Cambios pendientes:", "📝".yellow());
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                } else {
                    println!("\n{} No hay cambios pendientes", "✅".green());
                }
                
                return Ok(Some(current.to_path_buf()));
            }
            return Ok(None);
        }
        current = current.parent().unwrap();
    }
    
    Ok(None)
}
