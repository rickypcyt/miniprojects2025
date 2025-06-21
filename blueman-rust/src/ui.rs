use crate::app::{App, View};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table, TableState},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    match app.current_view {
        View::DeviceList => draw_device_list(f, app),
        View::DeviceDetails => draw_device_details(f, app),
        View::Settings => draw_settings(f, app),
    }
}

fn draw_device_list(f: &mut Frame, app: &App) {
    let chunks = create_main_layout(f.size());
    
    draw_header(f, app, chunks[0]);
    draw_device_table(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);
}

fn draw_device_details(f: &mut Frame, app: &App) {
    let chunks = create_main_layout(f.size());
    
    draw_header(f, app, chunks[0]);
    draw_device_info(f, app, chunks[1]);
    draw_device_actions(f, app, chunks[2]);
}

fn draw_settings(f: &mut Frame, app: &App) {
    let chunks = create_main_layout(f.size());
    
    draw_header(f, app, chunks[0]);
    draw_settings_panel(f, app, chunks[1]);
    draw_status_bar(f, app, chunks[2]);
}

fn create_main_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(10),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area)
        .to_vec()
}

fn draw_header(f: &mut Frame, app: &App, area: Rect) {
    let title = create_header_widget(app);
    f.render_widget(title, area);
}

fn draw_device_table(f: &mut Frame, app: &App, area: Rect) {
    let table = create_device_table_widget(app);
    f.render_widget(table, area);
}

fn draw_device_info(f: &mut Frame, app: &App, area: Rect) {
    let info = create_device_info_widget(app);
    f.render_widget(info, area);
}

fn draw_device_actions(f: &mut Frame, app: &App, area: Rect) {
    let actions = create_device_actions_widget(app);
    f.render_widget(actions, area);
}

fn draw_settings_panel(f: &mut Frame, app: &App, area: Rect) {
    let settings = create_settings_widget(app);
    f.render_widget(settings, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let status = create_status_widget(app);
    f.render_widget(status, area);
}

fn create_header_widget(app: &App) -> Paragraph {
    let title_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    
    let bluetooth_status = if app.bluetooth_enabled {
        "●".to_string()
    } else {
        "○".to_string()
    };
    
    let scanning_status = if app.scanning {
        " [SCANNING]".to_string()
    } else {
        "".to_string()
    };
    
    let title = Line::from(vec![
        Span::styled("Bluetooth Manager ", title_style),
        Span::styled(bluetooth_status, Style::default().fg(if app.bluetooth_enabled { Color::Green } else { Color::Red })),
        Span::styled(scanning_status, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]);
    
    Paragraph::new(title)
        .block(Block::default().borders(Borders::ALL).title("Bluetooth Manager"))
        .alignment(ratatui::layout::Alignment::Center)
}

fn create_device_table_widget(app: &App) -> Table {
    let header_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    
    let headers = vec![
        "Name",
        "Address", 
        "Type",
        "Status",
        "RSSI",
        "Known",
    ];
    
    let header_row = Row::new(
        headers.iter().map(|h| Span::styled(*h, header_style))
    );
    
    let rows: Vec<Row> = if app.devices.is_empty() {
        // Show a helpful message when no devices are found
        vec![Row::new(vec![
            "No devices found".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ]).style(Style::default().fg(Color::Yellow))]
    } else {
        app.devices.iter().enumerate().map(|(i, device)| {
            let status = if device.connected {
                "Connected"
            } else if device.paired {
                "Paired"
            } else {
                "Available"
            };
            
            let _status_color = if device.connected {
                Color::Green
            } else if device.paired {
                Color::Yellow
            } else {
                Color::White
            };
            
            let rssi_text = device.rssi.map(|r| format!("{} dBm", r)).unwrap_or_else(|| "N/A".to_string());
            let known_text = if device.known { "Yes" } else { "No" };
            
            let row_style = if i == app.selected_index {
                Style::default().fg(Color::Black).bg(Color::White)
            } else {
                Style::default()
            };
            
            // Color code known devices differently
            let _name_style = if device.known {
                Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            Row::new(vec![
                device.display_name(),
                device.address.clone(),
                device.device_type.clone(),
                status.to_string(),
                rssi_text,
                known_text.to_string(),
            ]).style(row_style)
        }).collect()
    };
    
    let widths = [
        Constraint::Percentage(20),
        Constraint::Percentage(20),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
        Constraint::Percentage(15),
    ];
    
    let title = if app.devices.is_empty() {
        "Devices (Press 's' to scan)"
    } else {
        "Devices"
    };
    
    Table::new(rows, widths)
        .header(header_row)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
}

fn create_device_info_widget(app: &App) -> Paragraph {
    if let Some(device) = app.get_selected_device() {
        let device_name = device.display_name();
        let device_address = device.address.clone();
        let device_type = device.device_type.clone();
        let rssi_text = device.rssi.map(|r| format!("{} dBm", r)).unwrap_or_else(|| "N/A".to_string());
        
        let info_text = vec![
            Line::from(vec![
                Span::styled("Device Information:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("Name: ", Style::default().fg(Color::Yellow)),
                Span::styled(device_name, Style::default()),
            ]),
            Line::from(vec![
                Span::styled("Address: ", Style::default().fg(Color::Yellow)),
                Span::styled(device_address, Style::default()),
            ]),
            Line::from(vec![
                Span::styled("Type: ", Style::default().fg(Color::Yellow)),
                Span::styled(device_type, Style::default()),
            ]),
            Line::from(vec![
                Span::styled("Connected: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    if device.connected { "Yes" } else { "No" },
                    Style::default().fg(if device.connected { Color::Green } else { Color::Red })
                ),
            ]),
            Line::from(vec![
                Span::styled("Paired: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    if device.paired { "Yes" } else { "No" },
                    Style::default().fg(if device.paired { Color::Green } else { Color::Red })
                ),
            ]),
            Line::from(vec![
                Span::styled("Trusted: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    if device.trusted { "Yes" } else { "No" },
                    Style::default().fg(if device.trusted { Color::Green } else { Color::Red })
                ),
            ]),
            Line::from(vec![
                Span::styled("Previously Known: ", Style::default().fg(Color::Yellow)),
                Span::styled(
                    if device.known { "Yes" } else { "No" },
                    Style::default().fg(if device.known { Color::Blue } else { Color::White })
                ),
            ]),
            Line::from(vec![
                Span::styled("Signal Strength: ", Style::default().fg(Color::Yellow)),
                Span::styled(rssi_text, Style::default()),
            ]),
        ];
        
        Paragraph::new(info_text)
            .block(Block::default().borders(Borders::ALL).title("Device Details"))
            .alignment(ratatui::layout::Alignment::Left)
    } else {
        Paragraph::new("No device selected")
            .block(Block::default().borders(Borders::ALL).title("Device Details"))
    }
}

fn create_device_actions_widget(_app: &App) -> Paragraph {
    let actions_text = vec![
        Line::from(vec![
            Span::styled("Device Actions:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("c - Connect/Disconnect", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("p - Pair Device", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("r - Remove Device", Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::styled("ESC - Back to Device List", Style::default().fg(Color::White)),
        ]),
    ];
    
    Paragraph::new(actions_text)
        .block(Block::default().borders(Borders::ALL).title("Actions"))
        .alignment(ratatui::layout::Alignment::Left)
}

fn create_settings_widget(app: &App) -> Paragraph {
    let settings_text = vec![
        Line::from(vec![
            Span::styled("Settings:", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("Bluetooth: ", Style::default().fg(Color::Yellow)),
            Span::styled(
                if app.bluetooth_enabled { "Enabled" } else { "Disabled" },
                Style::default().fg(if app.bluetooth_enabled { Color::Green } else { Color::Red })
            ),
        ]),
        Line::from(vec![
            Span::styled("Total Devices: ", Style::default().fg(Color::Yellow)),
            Span::styled(app.get_device_count().to_string(), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("Known Devices: ", Style::default().fg(Color::Yellow)),
            Span::styled(app.get_known_devices_count().to_string(), Style::default().fg(Color::Blue)),
        ]),
        Line::from(vec![
            Span::styled("Available Devices: ", Style::default().fg(Color::Yellow)),
            Span::styled(app.get_available_devices_count().to_string(), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("Connected Devices: ", Style::default().fg(Color::Yellow)),
            Span::styled(app.get_connected_count().to_string(), Style::default()),
        ]),
        Line::from(vec![
            Span::styled("", Style::default()),
        ]),
        Line::from(vec![
            Span::styled("t - Toggle Bluetooth", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("s - Start/Stop Scanning", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("ESC - Back to Device List", Style::default().fg(Color::White)),
        ]),
    ];
    
    Paragraph::new(settings_text)
        .block(Block::default().borders(Borders::ALL).title("Settings"))
        .alignment(ratatui::layout::Alignment::Left)
}

fn create_status_widget(app: &App) -> Paragraph {
    let status_style = if app.error_message.is_some() {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::Green)
    };
    
    let status_text = if let Some(error) = &app.error_message {
        error.clone()
    } else {
        app.status_message.clone()
    };
    
    let status = Line::from(vec![
        Span::styled("Status: ", Style::default().fg(Color::Cyan)),
        Span::styled(status_text, status_style),
    ]);
    
    Paragraph::new(status)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .alignment(ratatui::layout::Alignment::Left)
} 