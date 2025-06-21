mod app;
mod ui;

use app::{App, View};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io;

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if let Event::Key(key) = crossterm::event::read()? {
            match app.current_view {
                View::DeviceList => handle_device_list_input(&mut app, key.code),
                View::DeviceDetails => handle_device_details_input(&mut app, key.code),
                View::Settings => handle_settings_input(&mut app, key.code),
            }

            // Global quit
            if key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn handle_device_list_input(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Up | KeyCode::Char('k') => {
            app.select_previous();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.select_next();
        }
        KeyCode::Char('s') => {
            app.toggle_scanning();
        }
        KeyCode::Char('t') => {
            app.toggle_bluetooth();
        }
        KeyCode::Enter => {
            if app.get_selected_device().is_some() {
                app.change_view(View::DeviceDetails);
            }
        }
        KeyCode::Char('?') => {
            app.change_view(View::Settings);
        }
        _ => {}
    }
}

fn handle_device_details_input(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Char('c') => {
            app.connect_device();
        }
        KeyCode::Char('p') => {
            app.pair_device();
        }
        KeyCode::Char('r') => {
            app.remove_device();
        }
        KeyCode::Esc => {
            app.change_view(View::DeviceList);
        }
        _ => {}
    }
}

fn handle_settings_input(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Char('t') => {
            app.toggle_bluetooth();
        }
        KeyCode::Char('s') => {
            app.toggle_scanning();
        }
        KeyCode::Esc => {
            app.change_view(View::DeviceList);
        }
        _ => {}
    }
}
