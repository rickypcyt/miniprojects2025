use std::process::Command;
use anyhow::{Result, anyhow};

#[derive(Debug, Clone)]
pub struct WifiConnection {
    pub ssid: String,
    pub password: Option<String>,
}

impl WifiConnection {
    pub fn new(ssid: String) -> Self {
        Self {
            ssid,
            password: None,
        }
    }

    pub fn with_password(mut self, password: String) -> Self {
        self.password = Some(password);
        self
    }

    /// Connect to a WiFi network using nmcli
    pub fn connect(&self) -> Result<()> {
        let mut cmd = Command::new("nmcli");
        
        if let Some(ref password) = self.password {
            // Connect with password
            cmd.args([
                "device", "wifi", "connect", &self.ssid, "password", password
            ]);
        } else {
            // Try to connect to an open network or use stored credentials
            cmd.args([
                "device", "wifi", "connect", &self.ssid
            ]);
        }

        let output = cmd.output()?;
        
        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to connect to {}: {}", self.ssid, error_msg))
        }
    }

    /// Disconnect from the current WiFi network
    pub fn disconnect() -> Result<()> {
        let output = Command::new("nmcli")
            .args(["device", "disconnect"])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to disconnect: {}", error_msg))
        }
    }

    /// Get the currently connected network SSID
    pub fn get_current_ssid() -> Result<Option<String>> {
        let output = Command::new("nmcli")
            .args(["-t", "-f", "active,ssid", "dev", "wifi"])
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for line in output_str.lines() {
            let mut parts = line.split(':');
            if let (Some(active), Some(ssid)) = (parts.next(), parts.next()) {
                if active == "yes" && !ssid.is_empty() {
                    return Ok(Some(ssid.to_string()));
                }
            }
        }
        
        Ok(None)
    }

    /// Check if we're currently connected to a specific network
    pub fn is_connected_to(&self) -> Result<bool> {
        if let Some(current_ssid) = Self::get_current_ssid()? {
            Ok(current_ssid == self.ssid)
        } else {
            Ok(false)
        }
    }

    /// Forget a saved network
    pub fn forget_network(&self) -> Result<()> {
        let output = Command::new("nmcli")
            .args(["connection", "delete", &self.ssid])
            .output()?;

        if output.status.success() {
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            Err(anyhow!("Failed to forget network {}: {}", self.ssid, error_msg))
        }
    }

    /// Get connection status for a specific network
    pub fn get_connection_status(&self) -> Result<ConnectionStatus> {
        let output = Command::new("nmcli")
            .args(["-t", "-f", "name,type,device", "connection", "show", "--active"])
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for line in output_str.lines() {
            let mut parts = line.split(':');
            if let (Some(name), Some(conn_type), Some(device)) = (parts.next(), parts.next(), parts.next()) {
                if name == &self.ssid && conn_type == "802-11-wireless" {
                    return Ok(ConnectionStatus::Connected(device.to_string()));
                }
            }
        }

        // Check if it's a saved connection
        let saved_output = Command::new("nmcli")
            .args(["-t", "-f", "name", "connection", "show"])
            .output()?;

        let saved_str = String::from_utf8_lossy(&saved_output.stdout);
        for line in saved_str.lines() {
            if line == &self.ssid {
                return Ok(ConnectionStatus::Saved);
            }
        }

        Ok(ConnectionStatus::Unknown)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Connected(String), // device name
    Saved,
    Unknown,
}

impl std::fmt::Display for ConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionStatus::Connected(device) => write!(f, "Connected ({})", device),
            ConnectionStatus::Saved => write!(f, "Saved"),
            ConnectionStatus::Unknown => write!(f, "Unknown"),
        }
    }
} 