use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph, ListState},
};
use crate::modules::wifi_scanner::WifiNetwork;
use std::time::Instant;

pub struct WifiUI {
    pub list_state: ListState,
    pub selected: usize,
    pub status_msg: String,
    pub show_password_prompt: bool,
    pub password_input: String,
    pub password_cursor: usize,
    pub animation_start: Instant,
}

impl WifiUI {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        
        Self {
            list_state,
            selected: 0,
            status_msg: String::new(),
            show_password_prompt: false,
            password_input: String::new(),
            password_cursor: 0,
            animation_start: Instant::now(),
        }
    }

    pub fn update_selection(&mut self, selected: usize) {
        self.selected = selected;
        self.list_state.select(Some(selected));
    }

    pub fn set_status(&mut self, msg: String) {
        self.status_msg = msg;
    }

    pub fn clear_status(&mut self) {
        self.status_msg.clear();
    }

    pub fn show_password_dialog(&mut self) {
        self.show_password_prompt = true;
        self.password_input.clear();
        self.password_cursor = 0;
    }

    pub fn hide_password_dialog(&mut self) {
        self.show_password_prompt = false;
        self.password_input.clear();
        self.password_cursor = 0;
    }

    pub fn add_password_char(&mut self, c: char) {
        if self.password_cursor < self.password_input.len() {
            self.password_input.insert(self.password_cursor, c);
        } else {
            self.password_input.push(c);
        }
        self.password_cursor += 1;
    }

    pub fn remove_password_char(&mut self) {
        if self.password_cursor > 0 {
            self.password_input.remove(self.password_cursor - 1);
            self.password_cursor -= 1;
        }
    }

    pub fn move_password_cursor(&mut self, direction: i32) {
        let new_pos = self.password_cursor as i32 + direction;
        if new_pos >= 0 && new_pos <= self.password_input.len() as i32 {
            self.password_cursor = new_pos as usize;
        }
    }

    pub fn get_password(&self) -> String {
        self.password_input.clone()
    }

    pub fn get_loading_dots(&self) -> String {
        let elapsed = self.animation_start.elapsed().as_millis();
        let dot_count = ((elapsed / 500) % 4) as usize;
        ".".repeat(dot_count)
    }
}

pub fn render_main_ui(
    frame: &mut Frame,
    wifi_enabled: bool,
    networks: &[WifiNetwork],
    ui: &mut WifiUI,
) {
    let size = frame.size();
    
    // Buscar red conectada
    let connected = networks.iter().find(|n| n.active);
    let connected_ssid = connected.map(|n| n.ssid.as_str());

    // Main title
    let title = if wifi_enabled {
        if let Some(ssid) = connected_ssid {
            format!("WiFi Manager [ENABLED] - Connected: {}", ssid)
        } else {
            format!("WiFi Manager [ENABLED] - Networks: {}", networks.len())
        }
    } else {
        "WiFi Manager [DISABLED]".to_string()
    };
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .title_alignment(Alignment::Center);
    
    frame.render_widget(block, size);

    if ui.show_password_prompt {
        render_password_dialog(frame, &ui.password_input, ui.password_cursor);
        return;
    }

    // Mostrar lista si hay al menos una red
    if wifi_enabled && !networks.is_empty() {
        render_network_list(frame, networks, ui);
    } else if wifi_enabled && connected_ssid.is_none() {
        // Solo mostrar 'Scanning...' si no hay ninguna red y no hay red activa
        render_scanning_message_with_animation(frame, ui);
    } else if !wifi_enabled {
        render_disabled_message(frame);
    }

    // Help text
    render_help_text(frame);

    // Status message
    if !ui.status_msg.is_empty() {
        render_status_message(frame, &ui.status_msg);
    }
}

fn render_network_list(
    frame: &mut Frame,
    networks: &[WifiNetwork],
    ui: &mut WifiUI,
) {
    let size = frame.size();
    
    let items: Vec<ListItem> = networks
        .iter()
        .take(10)
        .map(|network| {
            let mut style = Style::default();
            if network.active {
                style = style.fg(Color::Green).add_modifier(Modifier::BOLD);
            }
            
            let percent = network.signal_percentage();
            let description = network.signal_description();
            
            let display_text = if network.active {
                format!("{}  [{}%] {} ({} dBm) [CONNECTED]", 
                    network.ssid, percent, description, network.signal)
            } else {
                format!("{}  [{}%] {} ({} dBm)", 
                    network.ssid, percent, description, network.signal)
            };
            
            ListItem::new(display_text).style(style)
        })
        .collect();

    let list = List::new(items)
        .highlight_style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    let area = Rect {
        x: size.x + 2,
        y: size.y + 2,
        width: size.width - 4,
        height: 12,
    };

    ui.list_state.select(Some(ui.selected));
    frame.render_stateful_widget(list, area, &mut ui.list_state);
}

fn render_scanning_message(frame: &mut Frame) {
    let size = frame.size();
    let msg = Paragraph::new("Scanning for networks...")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));
    
    let area = Rect {
        x: size.x + 2,
        y: size.y + 4,
        width: size.width - 4,
        height: 3,
    };
    
    frame.render_widget(msg, area);
}

fn render_scanning_message_with_animation(frame: &mut Frame, ui: &WifiUI) {
    let size = frame.size();
    let dots = ui.get_loading_dots();
    let msg = Paragraph::new(format!("Scanning for networks{}", dots))
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Yellow));
    
    let area = Rect {
        x: size.x + 2,
        y: size.y + 4,
        width: size.width - 4,
        height: 3,
    };
    
    frame.render_widget(msg, area);
}

fn render_disabled_message(frame: &mut Frame) {
    let size = frame.size();
    let msg = Paragraph::new("WiFi is disabled. Press 'p' to enable.")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));
    
    let area = Rect {
        x: size.x + 2,
        y: size.y + 4,
        width: size.width - 4,
        height: 3,
    };
    
    frame.render_widget(msg, area);
}

fn render_help_text(frame: &mut Frame) {
    let size = frame.size();
    let help = Paragraph::new(
        "p: Enable WiFi | o: Disable WiFi | r: Refresh | Enter: Connect | d: Disconnect | f: Forget"
    )
    .alignment(Alignment::Left)
    .style(Style::default().fg(Color::Gray));
    
    let help_area = Rect {
        x: size.x + 2,
        y: size.y + size.height - 3,
        width: size.width - 4,
        height: 2,
    };
    
    frame.render_widget(help, help_area);
}

fn render_status_message(frame: &mut Frame, status: &str) {
    let size = frame.size();
    let status_widget = Paragraph::new(status)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);
    
    let area = Rect {
        x: size.x + 2,
        y: size.y + size.height - 6,
        width: size.width - 4,
        height: 2,
    };
    
    frame.render_widget(status_widget, area);
}

fn render_password_dialog(frame: &mut Frame, password: &str, cursor_pos: usize) {
    let size = frame.size();
    
    // Semi-transparent overlay
    let overlay = Block::default()
        .style(Style::default().bg(Color::Black));
    frame.render_widget(overlay, size);
    
    // Password dialog box
    let dialog_width = 50;
    let dialog_height = 7;
    let dialog_x = (size.width - dialog_width) / 2;
    let dialog_y = (size.height - dialog_height) / 2;
    
    let dialog_area = Rect {
        x: dialog_x,
        y: dialog_y,
        width: dialog_width,
        height: dialog_height,
    };
    
    let dialog_block = Block::default()
        .borders(Borders::ALL)
        .title("Enter WiFi Password")
        .title_alignment(Alignment::Center)
        .style(Style::default().bg(Color::DarkGray));
    
    frame.render_widget(dialog_block, dialog_area);
    
    // Password input field
    let input_area = Rect {
        x: dialog_x + 2,
        y: dialog_y + 3,
        width: dialog_width - 4,
        height: 1,
    };
    
    let masked_password = "*".repeat(password.len());
    let input_text = if cursor_pos < masked_password.len() {
        format!("{}|{}", 
            &masked_password[..cursor_pos], 
            &masked_password[cursor_pos..])
    } else {
        format!("{}|", masked_password)
    };
    
    let input_widget = Paragraph::new(input_text)
        .style(Style::default().fg(Color::White).bg(Color::Black));
    
    frame.render_widget(input_widget, input_area);
    
    // Instructions
    let instructions = Paragraph::new("Press Enter to connect, Esc to cancel")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    
    let instructions_area = Rect {
        x: dialog_x + 2,
        y: dialog_y + 5,
        width: dialog_width - 4,
        height: 1,
    };
    
    frame.render_widget(instructions, instructions_area);
} 