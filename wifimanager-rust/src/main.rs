use crossterm::{event::{self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}};
use ratatui::{prelude::*, widgets::{Block, Borders, List, ListItem, Paragraph, ListState}};
use std::{io, process::Command, time::{Duration, Instant}};
use tokio_wifiscanner::scan;
use std::sync::mpsc;
use std::thread;

enum InputEvent {
    Input(KeyCode),
    Tick,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Canal para eventos
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let CEvent::Key(key) = event::read().unwrap() {
                    tx.send(InputEvent::Input(key.code)).unwrap();
                }
            }
            tx.send(InputEvent::Tick).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Estado de la app
    let mut ssids = get_ssids().await?;
    let mut selected = 0;
    let mut running = true;
    let mut last_refresh = Instant::now();
    let mut list_state = ListState::default();
    list_state.select(Some(selected));

    while running {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().borders(Borders::ALL).title("WiFi Manager");
            f.render_widget(block, size);

            let items: Vec<ListItem> = ssids.iter().take(10).map(|ssid| ListItem::new(ssid.clone())).collect();
            let list = List::new(items)
                .highlight_style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
                .highlight_symbol("> ");
            let area = Rect {
                x: size.x + 2,
                y: size.y + 2,
                width: size.width - 4,
                height: 12,
            };
            list_state.select(Some(selected));
            f.render_stateful_widget(list, area, &mut list_state);

            let help = Paragraph::new("Flechas: Navegar | Enter: Seleccionar | r: Refrescar | p: Prender WiFi | o: Apagar WiFi | Esc: Salir")
                .alignment(Alignment::Left);
            let help_area = Rect {
                x: size.x + 2,
                y: size.y + size.height - 3,
                width: size.width - 4,
                height: 2,
            };
            f.render_widget(help, help_area);
        })?;

        match rx.recv()? {
            InputEvent::Input(key) => match key {
                KeyCode::Up => if selected > 0 { selected -= 1; },
                KeyCode::Down => if selected + 1 < ssids.len() && selected < 9 { selected += 1; },
                KeyCode::Enter => {
                    // Aquí podrías mostrar detalles o conectar
                },
                KeyCode::Char('r') => {
                    ssids = get_ssids().await?;
                    selected = 0;
                    last_refresh = Instant::now();
                },
                KeyCode::Char('p') => {
                    toggle_wifi(true);
                },
                KeyCode::Char('o') => {
                    toggle_wifi(false);
                },
                KeyCode::Esc => running = false,
                _ => {}
            },
            InputEvent::Tick => {},
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;
    Ok(())
}

async fn get_ssids() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let networks = scan().await?;
    Ok(networks.iter().map(|n| n.ssid.clone()).collect())
}

fn toggle_wifi(on: bool) {
    let cmd = if on { "on" } else { "off" };
    let _ = Command::new("nmcli").args(["radio", "wifi", cmd]).output();
}
