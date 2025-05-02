use crate::git_sync::list_git_repos;
use crate::ui::App;
use std::error::Error;

pub fn run_cli(app: &mut App) -> Result<(), Box<dyn Error>> {
    // Verificar conexión a internet
    if !crate::utils::check_internet() {
        app.update_status("❌ No hay conexión a internet");
        return Err("No hay conexión a internet".into());
    }

    app.update_status("🌐 Conexión a internet verificada");
    app.refresh_entries();
    Ok(())
}
