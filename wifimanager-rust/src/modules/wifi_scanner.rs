use std::collections::HashMap;
use std::process::Command;
use anyhow::Result;
use crate::modules::network_manager::NetworkManagerClient;

#[derive(Clone, Debug)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: i32,
    pub active: bool,
    pub security: String,
    pub frequency: Option<String>,
}

impl WifiNetwork {
    pub fn new(ssid: String, signal: i32, active: bool) -> Self {
        Self {
            ssid,
            signal,
            active,
            security: String::new(),
            frequency: None,
        }
    }

    pub fn with_security(mut self, security: String) -> Self {
        self.security = security;
        self
    }

    pub fn with_frequency(mut self, frequency: String) -> Self {
        self.frequency = Some(frequency);
        self
    }

    /// Get signal strength as percentage
    pub fn signal_percentage(&self) -> u8 {
        if self.signal <= -100 {
            0
        } else {
            let normalized = (self.signal + 100).max(0).min(70);
            (normalized as f32 / 70.0 * 100.0).round() as u8
        }
    }

    /// Get signal strength description
    pub fn signal_description(&self) -> &'static str {
        match self.signal {
            -30..=0 => "Excellent",
            -50..=-31 => "Good",
            -70..=-51 => "Fair",
            -80..=-71 => "Poor",
            _ => "Very Poor",
        }
    }
}

pub struct WifiScanner {
    nm_client: Option<NetworkManagerClient>,
}

impl WifiScanner {
    pub async fn new() -> Self {
        let nm_client = NetworkManagerClient::new().await.ok();
        Self { nm_client }
    }

    /// Check if WiFi is enabled
    pub fn is_wifi_enabled() -> bool {
        let output = Command::new("nmcli")
            .args(["radio", "wifi"])
            .output()
            .ok();
        
        if let Some(out) = output {
            let s = String::from_utf8_lossy(&out.stdout);
            s.trim() == "enabled"
        } else {
            false
        }
    }

    /// Toggle WiFi on/off
    pub fn toggle_wifi(on: bool) -> Result<()> {
        let cmd = if on { "on" } else { "off" };
        let output = Command::new("nmcli")
            .args(["radio", "wifi", cmd])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to toggle WiFi: {}", error_msg))
        }
    }

    /// Scan for WiFi networks using D-Bus or nmcli as fallback
    pub async fn scan_networks(&self) -> Result<Vec<WifiNetwork>> {
        // Try D-Bus first
        if let Some(ref nm_client) = self.nm_client {
            match nm_client.scan_networks().await {
                Ok(dbus_networks) => {
                    return Ok(dbus_networks.into_iter().map(|dbus_net| {
                        WifiNetwork {
                            ssid: dbus_net.ssid,
                            signal: dbus_net.signal as i32,
                            active: dbus_net.active,
                            security: dbus_net.security,
                            frequency: dbus_net.frequency,
                        }
                    }).collect());
                }
                Err(e) => {
                    eprintln!("D-Bus scan failed: {}, falling back to nmcli", e);
                }
            }
        }

        // Fallback to nmcli
        self.scan_networks_nmcli().await
    }

    /// Scan using nmcli (fallback method)
    async fn scan_networks_nmcli(&self) -> Result<Vec<WifiNetwork>> {
        let output = Command::new("nmcli")
            .args(["-t", "-f", "ssid,signal,security,freq", "dev", "wifi", "list"])
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!("nmcli command failed"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let active_ssid = Self::get_active_ssid();
        let mut network_map: HashMap<String, WifiNetwork> = HashMap::new();

        for line in output_str.lines() {
            let mut parts = line.split(':');
            if let (Some(ssid), Some(signal_str), Some(security), Some(freq)) = 
                (parts.next(), parts.next(), parts.next(), parts.next()) {
                
                if !ssid.is_empty() {
                    let signal = signal_str.parse().unwrap_or(-100);
                    let active = active_ssid.as_ref().map(|a| a == ssid).unwrap_or(false);
                    
                    let entry = WifiNetwork::new(ssid.to_string(), signal, active)
                        .with_security(security.to_string())
                        .with_frequency(freq.to_string());
                    
                    network_map
                        .entry(ssid.to_string())
                        .and_modify(|e| {
                            if signal > e.signal {
                                *e = entry.clone();
                            }
                        })
                        .or_insert(entry);
                }
            }
        }

        let mut entries: Vec<WifiNetwork> = network_map.into_values().collect();
        entries.sort_by(|a, b| b.signal.cmp(&a.signal));
        
        Ok(entries)
    }

    /// Get the currently active SSID
    pub fn get_active_ssid() -> Option<String> {
        let output = Command::new("nmcli")
            .args(["-t", "-f", "active,ssid", "dev", "wifi"])
            .output()
            .ok()?;
        
        let out = String::from_utf8_lossy(&output.stdout);
        for line in out.lines() {
            let mut parts = line.split(':');
            if let (Some(active), Some(ssid)) = (parts.next(), parts.next()) {
                if active == "yes" && !ssid.is_empty() {
                    return Some(ssid.to_string());
                }
            }
        }
        None
    }

    /// Get detailed network information using nmcli
    pub fn get_network_details(ssid: &str) -> Result<Option<NetworkDetails>> {
        let output = Command::new("nmcli")
            .args(["-t", "-f", "ssid,signal,security,freq", "dev", "wifi", "list"])
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for line in output_str.lines() {
            let mut parts = line.split(':');
            if let (Some(network_ssid), Some(signal), Some(security), Some(freq)) = 
                (parts.next(), parts.next(), parts.next(), parts.next()) {
                if network_ssid == ssid {
                    let signal_strength = signal.parse().unwrap_or(-100);
                    return Ok(Some(NetworkDetails {
                        ssid: ssid.to_string(),
                        signal: signal_strength,
                        security: security.to_string(),
                        frequency: freq.to_string(),
                    }));
                }
            }
        }
        
        Ok(None)
    }

    /// Get WiFi device information
    pub fn get_wifi_devices() -> Result<Vec<WifiDevice>> {
        let output = Command::new("nmcli")
            .args(["-t", "-f", "device,type,state", "dev"])
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut devices = Vec::new();

        for line in output_str.lines() {
            let mut parts = line.split(':');
            if let (Some(device), Some(device_type), Some(state)) = 
                (parts.next(), parts.next(), parts.next()) {
                if device_type == "wifi" {
                    devices.push(WifiDevice {
                        name: device.to_string(),
                        state: state.to_string(),
                    });
                }
            }
        }

        Ok(devices)
    }
}

#[derive(Debug, Clone)]
pub struct NetworkDetails {
    pub ssid: String,
    pub signal: i32,
    pub security: String,
    pub frequency: String,
}

#[derive(Debug, Clone)]
pub struct WifiDevice {
    pub name: String,
    pub state: String,
} 