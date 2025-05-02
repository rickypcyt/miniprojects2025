use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, ListState},
    Terminal,
};
use std::{
    error::Error,
    io::{stdout, Stdout},
    path::{Path, PathBuf},
    time::Duration,
    collections::HashMap,
};

pub struct App {
    pub current_dir: PathBuf,
    pub entries: Vec<PathBuf>,
    pub selected: usize,
    pub status: String,
    pub progress: String,
    pub list_state: ListState,
    pub popup_message: Option<String>,
    pub cache: HashMap<PathBuf, (String, bool)>, // Cache for git info and is_git_repo
}

impl App {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_default();
        let mut app = Self {
            current_dir: current_dir.clone(),
            entries: Vec::new(),
            selected: 0,
            status: "⏳ Analizando directorios...".to_string(),
            progress: String::new(),
            list_state: ListState::default(),
            popup_message: None,
            cache: HashMap::new(),
        };
        app.refresh_entries();
        app.status = "✅ Análisis completado".to_string();
        app
    }

    pub fn update_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    pub fn update_progress(&mut self, progress: &str) {
        self.progress = progress.to_string();
    }

    pub fn refresh_entries(&mut self) {
        self.entries.clear();
        self.cache.clear();
        
        // Añadir opción para subir un nivel si no estamos en la raíz del usuario
        if let Some(parent) = self.current_dir.parent() {
            if parent.starts_with("/home/") {
                self.entries.push(parent.to_path_buf());
                self.cache.insert(parent.to_path_buf(), ("↩️  Volver".to_string(), false));
            }
        }
        
        // Leer el directorio actual
        if let Ok(entries) = std::fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Verificar si es un repositorio git o contiene uno
                    if path.join(".git").exists() {
                        let path_clone = path.clone();
                        self.entries.push(path_clone.clone());
                        self.cache.insert(path_clone, (self.get_git_info(&path).unwrap_or_default(), true));
                    } else if let Ok(dir_entries) = std::fs::read_dir(&path) {
                        if dir_entries.flatten().any(|e| e.path().join(".git").exists()) {
                            let path_clone = path.clone();
                            self.entries.push(path_clone.clone());
                            self.cache.insert(path_clone, ("📁 Directorio con git".to_string(), false));
                        }
                    }
                }
            }
        }
        
        self.entries.sort();
        self.selected = 0;
        self.list_state.select(Some(0));
    }

    fn get_git_info(&self, path: &Path) -> Result<String, Box<dyn Error>> {
        // Branch actual
        let branch = std::process::Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .current_dir(path)
            .output()?;
        
        let branch_info = if !branch.stdout.is_empty() {
            format!("Branch: {}", String::from_utf8_lossy(&branch.stdout).trim())
        } else {
            "No branch".to_string()
        };

        // Estado del repositorio
        let status = std::process::Command::new("git")
            .arg("status")
            .arg("-s")
            .current_dir(path)
            .output()?;
        
        let changes = if !status.stdout.is_empty() {
            "Cambios pendientes: Sí"
        } else {
            "Cambios pendientes: No"
        };

        // Commits adelante/atrás
        let ahead = std::process::Command::new("git")
            .arg("rev-list")
            .arg("--count")
            .arg("@{u}..")
            .current_dir(path)
            .output()?;
        
        let behind = std::process::Command::new("git")
            .arg("rev-list")
            .arg("--count")
            .arg("..@{u}")
            .current_dir(path)
            .output()?;

        let ahead_count = String::from_utf8_lossy(&ahead.stdout).trim().parse::<i32>().unwrap_or(0);
        let behind_count = String::from_utf8_lossy(&behind.stdout).trim().parse::<i32>().unwrap_or(0);

        let ahead_info = if ahead_count > 0 {
            format!("↑{}", ahead_count)
        } else {
            String::new()
        };

        let behind_info = if behind_count > 0 {
            format!("↓{}", behind_count)
        } else {
            String::new()
        };

        Ok(format!("{} | {} | {} {}", branch_info, changes, ahead_info, behind_info))
    }

    pub fn get_entry_info(&self, path: &Path) -> Result<String, Box<dyn Error>> {
        if let Some((info, _)) = self.cache.get(path) {
            return Ok(info.clone());
        }
        Ok("📁 Directorio".to_string())
    }

    pub fn navigate_into(&mut self, path: &Path) {
        if path.is_dir() {
            self.current_dir = path.to_path_buf();
            self.refresh_entries();
        }
    }

    pub fn navigate_up(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            if parent.starts_with("/home/") {
                self.current_dir = parent.to_path_buf();
                self.refresh_entries();
            }
        }
    }

    pub fn show_popup(&mut self, message: String) {
        self.popup_message = Some(message);
    }

    pub fn clear_popup(&mut self) {
        self.popup_message = None;
    }

    pub fn show_changes(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        if !path.join(".git").exists() {
            self.show_popup("No es un repositorio git".to_string());
            return Ok(());
        }

        let output = std::process::Command::new("git")
            .arg("status")
            .arg("-s")
            .current_dir(path)
            .output()?;
        
        if !output.stdout.is_empty() {
            self.show_popup(String::from_utf8_lossy(&output.stdout).to_string());
        } else {
            self.show_popup("No hay cambios".to_string());
        }
        Ok(())
    }

    pub fn show_diff(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        if !path.join(".git").exists() {
            self.show_popup("No es un repositorio git".to_string());
            return Ok(());
        }

        let output = std::process::Command::new("git")
            .arg("diff")
            .current_dir(path)
            .output()?;
        
        if !output.stdout.is_empty() {
            self.show_popup(String::from_utf8_lossy(&output.stdout).to_string());
        } else {
            self.show_popup("No hay diferencias".to_string());
        }
        Ok(())
    }

    pub fn sync_repo(&mut self, path: &Path) -> Result<(), Box<dyn Error>> {
        if !path.join(".git").exists() {
            self.show_popup("No es un repositorio git".to_string());
            return Ok(());
        }

        // Fetch
        self.update_progress("⏳ Haciendo fetch...");
        let fetch = std::process::Command::new("git")
            .arg("fetch")
            .current_dir(path)
            .output()?;
        if !fetch.status.success() {
            self.show_popup(String::from_utf8_lossy(&fetch.stderr).to_string());
            return Ok(());
        }

        // Pull
        self.update_progress("⏳ Haciendo pull...");
        let pull = std::process::Command::new("git")
            .arg("pull")
            .current_dir(path)
            .output()?;
        if !pull.status.success() {
            self.show_popup(String::from_utf8_lossy(&pull.stderr).to_string());
            return Ok(());
        }

        // Push
        self.update_progress("⏳ Haciendo push...");
        let push = std::process::Command::new("git")
            .arg("push")
            .current_dir(path)
            .output()?;
        if !push.status.success() {
            self.show_popup(String::from_utf8_lossy(&push.stderr).to_string());
            return Ok(());
        }

        self.update_progress("✅ Sincronización completada");
        self.show_popup("✅ Repositorio sincronizado".to_string());
        Ok(())
    }
}

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn draw_ui(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<(), Box<dyn Error>> {
    terminal.draw(|f| {
        let size = f.size();
        let margin = if size.width > 80 { 2 } else { 1 };
        let title_height = if size.height > 30 { 3 } else { 2 };
        let shortcuts_height = if size.height > 30 { 3 } else { 2 };

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(margin)
            .constraints([
                Constraint::Length(title_height),
                Constraint::Min(0),
                Constraint::Length(title_height),
                Constraint::Length(shortcuts_height),
            ])
            .split(f.size());

        // Current directory
        let current_dir = Paragraph::new(format!("📂 Directorio actual: {}", app.current_dir.to_string_lossy()))
            .style(Style::default().fg(Color::Cyan))
            .block(Block::default().borders(Borders::ALL).title("Current Directory"));
        f.render_widget(current_dir, chunks[0]);

        // Entries list
        let mut items: Vec<ListItem> = Vec::new();
        for path in &app.entries {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            let info = app.get_entry_info(path).unwrap_or_default();
            
            // Ajustar el texto según el ancho de la ventana
            let display_text = if size.width > 80 {
                format!("{} - {}", name, info)
            } else {
                name.to_string()
            };

            // Determinar el estilo según el tipo de entrada
            let style = if path == app.current_dir.parent().unwrap_or(path) {
                Style::default().fg(Color::Magenta)
            } else if path.join(".git").exists() {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Blue)
            };

            items.push(ListItem::new(Line::from(vec![
                Span::styled(display_text, style),
            ])));
        }

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Directories"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");
        f.render_stateful_widget(list, chunks[1], &mut app.list_state);

        // Status and progress
        let status = Paragraph::new(format!("{}\n{}", app.status, app.progress))
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(status, chunks[2]);

        // Keyboard shortcuts
        let shortcuts_text = if size.width > 80 {
            "↑↓: Navegar directorios | Enter/→: Entrar directorio | ←: Subir directorio | c: Cambios | d: Diferencias | s: Sincronizar | r: Refrescar | q: Salir"
        } else if size.width > 60 {
            "↑↓: Navegar | Enter/→: Entrar | ←: Subir | c: Cambios | d: Diferencias | s: Sincronizar | r: Refrescar | q: Salir"
        } else {
            "↑↓: Navegar | Enter: Entrar | c: Cambios | d: Diferencias | s: Sincronizar | q: Salir"
        };

        let shortcuts = Paragraph::new(shortcuts_text)
            .style(Style::default().fg(Color::Magenta))
            .block(Block::default().borders(Borders::ALL).title("Atajos"));
        f.render_widget(shortcuts, chunks[3]);

        // Popup message
        if let Some(message) = &app.popup_message {
            let popup_width = (size.width as f32 * 0.8) as u16;
            let popup_height = (size.height as f32 * 0.3) as u16;
            let popup_x = (size.width - popup_width) / 2;
            let popup_y = (size.height - popup_height) / 2;

            let popup = Paragraph::new(message.clone())
                .style(Style::default().fg(Color::White))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title("Git Output")
                    .style(Style::default().fg(Color::Yellow)));
            
            f.render_widget(popup, Rect::new(popup_x, popup_y, popup_width, popup_height));
        }
    })?;
    Ok(())
}

pub fn handle_events(app: &mut App) -> Result<bool, Box<dyn Error>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Up => {
                    if app.selected > 0 {
                        app.selected -= 1;
                        app.list_state.select(Some(app.selected));
                    }
                }
                KeyCode::Down => {
                    if app.selected < app.entries.len().saturating_sub(1) {
                        app.selected += 1;
                        app.list_state.select(Some(app.selected));
                    }
                }
                KeyCode::Left => {
                    app.navigate_up();
                }
                KeyCode::Right | KeyCode::Enter => {
                    if let Some(path) = app.entries.get(app.selected) {
                        let path = path.clone();
                        app.navigate_into(&path);
                    }
                }
                KeyCode::Char('c') => {
                    if let Some(path) = app.entries.get(app.selected) {
                        let path = path.clone();
                        app.show_changes(&path)?;
                    }
                }
                KeyCode::Char('d') => {
                    if let Some(path) = app.entries.get(app.selected) {
                        let path = path.clone();
                        app.show_diff(&path)?;
                    }
                }
                KeyCode::Char('s') => {
                    if let Some(path) = app.entries.get(app.selected) {
                        let path = path.clone();
                        app.sync_repo(&path)?;
                    }
                }
                KeyCode::Char('r') => {
                    app.refresh_entries();
                }
                KeyCode::Esc => {
                    app.clear_popup();
                }
                _ => {}
            }
        }
    }
    Ok(false)
} 