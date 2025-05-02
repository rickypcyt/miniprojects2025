use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

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

fn main() -> Result<(), Box<dyn Error>> {
    let current_dir = std::env::current_dir()?;
    
    if let Some(repo_path) = find_git_repo(&current_dir) {
        println!("📂 Repositorio encontrado: {}", repo_path.display());
        
        // Mostrar estado
        let status = Command::new("git")
            .arg("status")
            .current_dir(&repo_path)
            .output()?;
        println!("{}", String::from_utf8_lossy(&status.stdout));
        
        // Verificar si hay cambios para commit
        let has_changes = !Command::new("git")
            .arg("diff")
            .arg("--quiet")
            .current_dir(&repo_path)
            .status()?
            .success();
        
        if has_changes {
            // Hacer commit con el mensaje proporcionado
            let args: Vec<String> = std::env::args().collect();
            if args.len() < 2 {
                println!("❌ Error: Debes proporcionar un mensaje de commit");
                return Ok(());
            }
            
            let message = &args[1];
            println!("⏳ Haciendo commit...");
            let commit = Command::new("git")
                .arg("commit")
                .arg("-am")
                .arg(message)
                .current_dir(&repo_path)
                .output()?;
            
            if !commit.status.success() {
                println!("❌ Error al hacer commit: {}", String::from_utf8_lossy(&commit.stderr));
                return Ok(());
            }
            println!("✅ Commit realizado");
        } else {
            println!("✅ No hay cambios para commit");
        }
        
        // Sincronizar
        println!("⏳ Sincronizando...");
        let fetch = Command::new("git")
            .arg("fetch")
            .current_dir(&repo_path)
            .output()?;
        
        if !fetch.status.success() {
            println!("❌ Error al hacer fetch: {}", String::from_utf8_lossy(&fetch.stderr));
            return Ok(());
        }
        
        let pull = Command::new("git")
            .arg("pull")
            .current_dir(&repo_path)
            .output()?;
        
        if !pull.status.success() {
            println!("❌ Error al hacer pull: {}", String::from_utf8_lossy(&pull.stderr));
            return Ok(());
        }
        
        let push = Command::new("git")
            .arg("push")
            .current_dir(&repo_path)
            .output()?;
        
        if !push.status.success() {
            println!("❌ Error al hacer push: {}", String::from_utf8_lossy(&push.stderr));
            return Ok(());
        }
        
        println!("✅ Sincronización completada");
    } else {
        println!("❌ No se encontró ningún repositorio git en el directorio actual o superiores");
    }
    
    Ok(())
}
