use std::{error::Error, path::Path};
use walkdir::WalkDir;
use crate::ui::App;

pub fn auto_sync_repo(repo_path: &Path, app: &mut App) -> Result<(), Box<dyn Error>> {
    // Fetch
    app.update_progress("⏳ Haciendo fetch...");
    let fetch = std::process::Command::new("git")
        .arg("fetch")
        .current_dir(repo_path)
        .status()?;
    if !fetch.success() {
        return Err("Error al hacer fetch".into());
    }

    // Pull
    app.update_progress("⏳ Haciendo pull...");
    let pull = std::process::Command::new("git")
        .arg("pull")
        .current_dir(repo_path)
        .status()?;
    if !pull.success() {
        return Err("Error al hacer pull".into());
    }

    // Push
    app.update_progress("⏳ Haciendo push...");
    let push = std::process::Command::new("git")
        .arg("push")
        .current_dir(repo_path)
        .status()?;
    if !push.success() {
        return Err("Error al hacer push".into());
    }

    app.update_progress("✅ Sincronización completada");
    Ok(())
}

pub fn list_git_repos(start_path: &str) -> Result<Vec<std::path::PathBuf>, Box<dyn Error>> {
    let mut repos = Vec::new();
    let start_path = std::path::Path::new(start_path);

    for entry in std::fs::read_dir(start_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            if path.join(".git").exists() {
                repos.push(path);
            } else {
                repos.extend(list_git_repos(path.to_str().unwrap())?);
            }
        }
    }

    Ok(repos)
}
