use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::{io, time::{Duration, Instant}};
use std::sync::mpsc;
use std::thread;

mod modules;
use modules::*;

enum InputEvent {
    Input(KeyCode),
    Tick,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Terminal setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Event handling setup
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(30)).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(InputEvent::Input(key.code)).unwrap();
                }
            }
            tx.send(InputEvent::Tick).unwrap();
            thread::sleep(Duration::from_millis(5000)); // Tick every 5s
        }
    });

    // Application state
    let wifi_scanner = WifiScanner::new().await;
    let mut wifi_enabled = WifiScanner::is_wifi_enabled();
    let mut networks = if wifi_enabled { 
        wifi_scanner.scan_networks().await.unwrap_or_default() 
    } else { 
        vec![] 
    };
    let mut ui = WifiUI::new();
    let mut running = true;
    let mut last_scan = Instant::now();
    let mut last_status_clear = Instant::now();

    // Main application loop
    while running {
        terminal.draw(|f| {
            render_main_ui(f, wifi_enabled, &networks, &mut ui);
        })?;

        match rx.recv()? {
            InputEvent::Input(key) => {
                if ui.show_password_prompt {
                    handle_password_input(key, &mut ui, &mut running);
                } else {
                    handle_main_input(
                        key, 
                        &mut ui, 
                        &mut wifi_enabled, 
                        &mut networks, 
                        &mut running,
                        &mut last_status_clear,
                        &wifi_scanner
                    ).await;
                }
            }
            InputEvent::Tick => {
                handle_tick(
                    &mut wifi_enabled,
                    &mut networks,
                    &mut ui,
                    &mut last_scan,
                    &mut last_status_clear,
                    &wifi_scanner
                ).await;
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

async fn handle_main_input(
    key: KeyCode,
    ui: &mut WifiUI,
    wifi_enabled: &mut bool,
    networks: &mut Vec<WifiNetwork>,
    running: &mut bool,
    last_status_clear: &mut Instant,
    wifi_scanner: &WifiScanner,
) {
    match key {
        KeyCode::Up => {
            if *wifi_enabled && ui.selected > 0 {
                ui.update_selection(ui.selected - 1);
            }
        }
        KeyCode::Down => {
            if *wifi_enabled && ui.selected + 1 < networks.len() && ui.selected < 9 {
                ui.update_selection(ui.selected + 1);
            }
        }
        KeyCode::Enter => {
            if *wifi_enabled && !networks.is_empty() && ui.selected < networks.len() {
                let selected_network = &networks[ui.selected];
                handle_network_connection(selected_network, ui, last_status_clear).await;
            }
        }
        KeyCode::Char('p') => {
            if let Err(e) = WifiScanner::toggle_wifi(true) {
                ui.set_status(format!("Failed to enable WiFi: {}", e));
            } else {
                ui.set_status("Enabling WiFi...".to_string());
            }
            *last_status_clear = Instant::now();
        }
        KeyCode::Char('o') => {
            if let Err(e) = WifiScanner::toggle_wifi(false) {
                ui.set_status(format!("Failed to disable WiFi: {}", e));
            } else {
                ui.set_status("Disabling WiFi...".to_string());
            }
            *last_status_clear = Instant::now();
        }
        KeyCode::Char('r') => {
            if *wifi_enabled {
                ui.set_status("Manual refresh...".to_string());
                *last_status_clear = Instant::now();
            }
        }
        KeyCode::Char('d') => {
            handle_disconnect(ui, last_status_clear).await;
        }
        KeyCode::Char('f') => {
            if *wifi_enabled && !networks.is_empty() && ui.selected < networks.len() {
                let selected_network = &networks[ui.selected];
                handle_forget_network(selected_network, ui, last_status_clear).await;
            }
        }
        KeyCode::Esc | KeyCode::Char('q') => {
            *running = false;
        }
        _ => {}
    }
}

fn handle_password_input(
    key: KeyCode,
    ui: &mut WifiUI,
    running: &mut bool,
) {
    match key {
        KeyCode::Char(c) if c.is_ascii() && !c.is_control() => {
            ui.add_password_char(c);
        }
        KeyCode::Backspace => {
            ui.remove_password_char();
        }
        KeyCode::Left => {
            ui.move_password_cursor(-1);
        }
        KeyCode::Right => {
            ui.move_password_cursor(1);
        }
        KeyCode::Enter => {
            // Password will be used in the next connection attempt
            ui.hide_password_dialog();
        }
        KeyCode::Esc | KeyCode::Char('q') => {
            ui.hide_password_dialog();
            *running = false;
        }
        _ => {}
    }
}

async fn handle_network_connection(
    network: &WifiNetwork,
    ui: &mut WifiUI,
    last_status_clear: &mut Instant,
) {
    // Check if already connected
    let connection = WifiConnection::new(network.ssid.clone());
    if let Ok(true) = connection.is_connected_to() {
        ui.set_status(format!("Already connected to {}", network.ssid));
        *last_status_clear = Instant::now();
        return;
    }

    // Check connection status
    match connection.get_connection_status() {
        Ok(ConnectionStatus::Saved) => {
            // Try to connect with saved credentials
            match connection.connect() {
                Ok(_) => {
                    ui.set_status(format!("Connected to {}", network.ssid));
                }
                Err(e) => {
                    ui.set_status(format!("Failed to connect to {}: {}", network.ssid, e));
                }
            }
        }
        Ok(ConnectionStatus::Unknown) => {
            // Show password prompt
            ui.show_password_dialog();
            return;
        }
        Ok(ConnectionStatus::Connected(_)) => {
            ui.set_status(format!("Already connected to {}", network.ssid));
        }
        Err(e) => {
            ui.set_status(format!("Error checking connection status: {}", e));
        }
    }
    *last_status_clear = Instant::now();
}

async fn handle_disconnect(
    ui: &mut WifiUI,
    last_status_clear: &mut Instant,
) {
    match WifiConnection::disconnect() {
        Ok(_) => {
            ui.set_status("Disconnected from WiFi".to_string());
        }
        Err(e) => {
            ui.set_status(format!("Failed to disconnect: {}", e));
        }
    }
    *last_status_clear = Instant::now();
}

async fn handle_forget_network(
    network: &WifiNetwork,
    ui: &mut WifiUI,
    last_status_clear: &mut Instant,
) {
    let connection = WifiConnection::new(network.ssid.clone());
    match connection.forget_network() {
        Ok(_) => {
            ui.set_status(format!("Forgot network {}", network.ssid));
        }
        Err(e) => {
            ui.set_status(format!("Failed to forget network {}: {}", network.ssid, e));
        }
    }
    *last_status_clear = Instant::now();
}

async fn handle_tick(
    wifi_enabled: &mut bool,
    networks: &mut Vec<WifiNetwork>,
    ui: &mut WifiUI,
    last_scan: &mut Instant,
    last_status_clear: &mut Instant,
    wifi_scanner: &WifiScanner,
) {
    let new_wifi_enabled = WifiScanner::is_wifi_enabled();
    let wifi_state_changed = new_wifi_enabled != *wifi_enabled;
    
    if wifi_state_changed {
        *wifi_enabled = new_wifi_enabled;
        ui.update_selection(0);
        if *wifi_enabled {
            ui.set_status("WiFi enabled, scanning...".to_string());
        } else {
            ui.set_status("WiFi disabled".to_string());
            networks.clear();
        }
        *last_status_clear = Instant::now();
    }
    
    if *wifi_enabled && (last_scan.elapsed().as_secs() >= 5 || wifi_state_changed) {
        match wifi_scanner.scan_networks().await {
            Ok(new_networks) => {
                *networks = new_networks;
                *last_scan = Instant::now();
            }
            Err(_) => {
                // Keep previous networks if scan fails
            }
        }
    }
    
    if !ui.status_msg.is_empty() && last_status_clear.elapsed().as_secs() >= 3 {
        ui.clear_status();
    }
} 