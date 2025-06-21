#[derive(Debug, Clone)]
pub struct BluetoothDevice {
    pub address: String,
    pub name: String,
    pub alias: Option<String>,
    pub connected: bool,
    pub paired: bool,
    pub trusted: bool,
    pub device_type: String,
    pub rssi: Option<i16>,
    pub known: bool, // Whether this device was previously paired/known
}

impl BluetoothDevice {
    pub fn new(address: String, name: String) -> Self {
        Self {
            address,
            name,
            alias: None,
            connected: false,
            paired: false,
            trusted: false,
            device_type: "Unknown".to_string(),
            rssi: None,
            known: false,
        }
    }

    pub fn new_known(address: String, name: String, device_type: String) -> Self {
        Self {
            address,
            name,
            alias: None,
            connected: false,
            paired: true,
            trusted: true,
            device_type,
            rssi: None,
            known: true,
        }
    }

    pub fn display_name(&self) -> String {
        self.alias.as_ref().unwrap_or(&self.name).clone()
    }
}

pub struct App {
    pub devices: Vec<BluetoothDevice>,
    pub selected_index: usize,
    pub scanning: bool,
    pub bluetooth_enabled: bool,
    pub current_view: View,
    pub status_message: String,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    DeviceList,
    DeviceDetails,
    Settings,
}

impl App {
    pub fn new() -> App {
        let mut app = App {
            devices: vec![],
            selected_index: 0,
            scanning: false,
            bluetooth_enabled: true,
            current_view: View::DeviceList,
            status_message: "Ready - Press 's' to start scanning".to_string(),
            error_message: None,
        };
        
        app
    }

    pub fn toggle_scanning(&mut self) {
        self.scanning = !self.scanning;
        if self.scanning {
            self.status_message = "Scanning for nearby devices...".to_string();
            self.scan_for_devices();
        } else {
            self.status_message = "Scan stopped".to_string();
        }
    }

    pub fn scan_for_devices(&mut self) {
        // TODO: Replace with real Bluetooth scanning
        // This would integrate with BlueZ via D-Bus
        // For now, this is a placeholder for real device discovery
        
        self.status_message = "No devices found nearby. Try moving closer to Bluetooth devices.".to_string();
        
        // In a real implementation, this would:
        // 1. Call BlueZ D-Bus methods to start discovery
        // 2. Listen for DeviceAdded signals
        // 3. Update the device list with real nearby devices
        // 4. Get real RSSI values and device information
    }

    pub fn select_next(&mut self) {
        if !self.devices.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.devices.len();
        }
    }

    pub fn select_previous(&mut self) {
        if !self.devices.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.devices.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn get_selected_device(&self) -> Option<&BluetoothDevice> {
        self.devices.get(self.selected_index)
    }

    pub fn toggle_bluetooth(&mut self) {
        self.bluetooth_enabled = !self.bluetooth_enabled;
        if self.bluetooth_enabled {
            self.status_message = "Bluetooth enabled".to_string();
        } else {
            self.status_message = "Bluetooth disabled".to_string();
        }
    }

    pub fn connect_device(&mut self) {
        if let Some(device) = self.devices.get_mut(self.selected_index) {
            if !device.connected {
                // TODO: Implement real Bluetooth connection
                device.connected = true;
                self.status_message = format!("Connected to {}", device.display_name());
            } else {
                // TODO: Implement real Bluetooth disconnection
                device.connected = false;
                self.status_message = format!("Disconnected from {}", device.display_name());
            }
        }
    }

    pub fn pair_device(&mut self) {
        if let Some(device) = self.devices.get_mut(self.selected_index) {
            if !device.paired {
                // TODO: Implement real Bluetooth pairing
                device.paired = true;
                device.trusted = true;
                device.known = true;
                self.status_message = format!("Paired with {}", device.display_name());
            }
        }
    }

    pub fn remove_device(&mut self) {
        if !self.devices.is_empty() {
            let device_name = self.devices[self.selected_index].display_name();
            let was_known = self.devices[self.selected_index].known;
            
            // TODO: Implement real device removal from BlueZ
            self.devices.remove(self.selected_index);
            if self.selected_index >= self.devices.len() && !self.devices.is_empty() {
                self.selected_index = self.devices.len() - 1;
            }
            
            if was_known {
                self.status_message = format!("Removed {} from known devices", device_name);
            } else {
                self.status_message = format!("Removed {}", device_name);
            }
        }
    }

    pub fn change_view(&mut self, view: View) {
        self.current_view = view;
    }

    pub fn get_device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn get_connected_count(&self) -> usize {
        self.devices.iter().filter(|d| d.connected).count()
    }

    pub fn get_known_devices_count(&self) -> usize {
        self.devices.iter().filter(|d| d.known).count()
    }

    pub fn get_available_devices_count(&self) -> usize {
        self.devices.iter().filter(|d| !d.known).count()
    }
} 