use anyhow::{Result, anyhow};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zbus::{Connection, dbus_interface, dbus_proxy};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: i32,
    pub active: bool,
    pub security: String,
    pub frequency: Option<String>,
    pub bssid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiDevice {
    pub name: String,
    pub state: String,
    pub active: bool,
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager",
    default_service = "org.freedesktop.NetworkManager",
    default_path = "/org/freedesktop/NetworkManager"
)]
trait NetworkManager {
    fn get_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
    fn get_active_connections(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
    fn get_connectivity(&self) -> zbus::Result<u32>;
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.Device",
    default_service = "org.freedesktop.NetworkManager"
)]
trait Device {
    fn get_device_type(&self) -> zbus::Result<u32>;
    fn get_state(&self) -> zbus::Result<u32>;
    fn get_active_connection(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.Device.Wireless",
    default_service = "org.freedesktop.NetworkManager"
)]
trait WirelessDevice {
    fn get_all_access_points(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
    fn request_scan(&self, options: HashMap<String, zbus::zvariant::Value<'_>>) -> zbus::Result<()>;
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.AccessPoint",
    default_service = "org.freedesktop.NetworkManager"
)]
trait AccessPoint {
    fn get_ssid(&self) -> zbus::Result<Vec<u8>>;
    fn get_strength(&self) -> zbus::Result<u8>;
    fn get_flags(&self) -> zbus::Result<u32>;
    fn get_wpa_flags(&self) -> zbus::Result<u32>;
    fn get_rsn_flags(&self) -> zbus::Result<u32>;
    fn get_frequency(&self) -> zbus::Result<u32>;
    fn get_bssid(&self) -> zbus::Result<Vec<u8>>;
}

#[dbus_proxy(
    interface = "org.freedesktop.NetworkManager.Connection.Active",
    default_service = "org.freedesktop.NetworkManager"
)]
trait ActiveConnection {
    fn get_connection(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
    fn get_devices(&self) -> zbus::Result<Vec<zbus::zvariant::OwnedObjectPath>>;
    fn get_state(&self) -> zbus::Result<u32>;
}

pub struct NetworkManagerClient {
    connection: Connection,
    nm_proxy: NetworkManagerProxy<'static>,
}

impl NetworkManagerClient {
    pub async fn new() -> Result<Self> {
        let connection = Connection::system().await?;
        let nm_proxy = NetworkManagerProxy::new(&connection).await?;
        
        Ok(Self {
            connection,
            nm_proxy,
        })
    }

    pub async fn is_wifi_enabled(&self) -> Result<bool> {
        // Check if any wireless device is available and enabled
        let devices = self.nm_proxy.get_devices().await?;
        
        for device_path in devices {
            let device_proxy = DeviceProxy::builder(&self.connection)
                .path(device_path)?
                .build()
                .await?;
            
            let device_type = device_proxy.get_device_type().await?;
            if device_type == 2 { // NM_DEVICE_TYPE_WIFI
                let state = device_proxy.get_state().await?;
                return Ok(state != 20); // NM_DEVICE_STATE_UNAVAILABLE
            }
        }
        
        Ok(false)
    }

    pub async fn scan_networks(&self) -> Result<Vec<WifiNetwork>> {
        let mut networks = Vec::new();
        let devices = self.nm_proxy.get_devices().await?;
        let active_connections = self.nm_proxy.get_active_connections().await?;
        
        // Get active SSIDs
        let active_ssids = Vec::new();
        for conn_path in active_connections {
            let conn_proxy = ActiveConnectionProxy::builder(&self.connection)
                .path(conn_path)?
                .build()
                .await?;
            
            let conn_state = conn_proxy.get_state().await?;
            if conn_state == 2 { // NM_ACTIVE_CONNECTION_STATE_ACTIVATED
                // Get connection details to find SSID
                let _connection_path = conn_proxy.get_connection().await?;
                // For simplicity, we'll get the SSID from the connection
                // This is a simplified version
            }
        }
        
        for device_path in devices {
            let device_proxy = DeviceProxy::builder(&self.connection)
                .path(device_path.clone())?
                .build()
                .await?;
            
            let device_type = device_proxy.get_device_type().await?;
            if device_type == 2 { // NM_DEVICE_TYPE_WIFI
                let wireless_proxy = WirelessDeviceProxy::builder(&self.connection)
                    .path(device_path)?
                    .build()
                    .await?;
                
                let access_points = wireless_proxy.get_all_access_points().await?;
                
                for ap_path in access_points {
                    let ap_proxy = AccessPointProxy::builder(&self.connection)
                        .path(ap_path)?
                        .build()
                        .await?;
                    
                    let ssid_bytes = ap_proxy.get_ssid().await?;
                    let ssid = String::from_utf8_lossy(&ssid_bytes).to_string();
                    
                    if !ssid.is_empty() {
                        let strength = ap_proxy.get_strength().await? as i32;
                        let frequency = ap_proxy.get_frequency().await?;
                        let bssid_bytes = ap_proxy.get_bssid().await?;
                        let bssid = if !bssid_bytes.is_empty() {
                            Some(format!("{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                                bssid_bytes[0], bssid_bytes[1], bssid_bytes[2],
                                bssid_bytes[3], bssid_bytes[4], bssid_bytes[5]))
                        } else {
                            None
                        };
                        
                        let security = self.get_security_type(&ap_proxy).await?;
                        let active = active_ssids.contains(&ssid);
                        
                        networks.push(WifiNetwork {
                            ssid,
                            signal: strength,
                            active,
                            security,
                            frequency: Some(frequency.to_string()),
                            bssid,
                        });
                    }
                }
            }
        }
        
        // Remove duplicates and sort by signal strength
        let mut unique_networks = HashMap::new();
        for network in networks {
            unique_networks.entry(network.ssid.clone())
                .and_modify(|existing: &mut WifiNetwork| {
                    if network.signal > existing.signal {
                        *existing = network.clone();
                    }
                })
                .or_insert(network);
        }
        
        let mut result: Vec<WifiNetwork> = unique_networks.into_values().collect();
        result.sort_by(|a, b| b.signal.cmp(&a.signal));
        
        Ok(result)
    }

    async fn get_security_type(&self, ap_proxy: &AccessPointProxy<'_>) -> Result<String> {
        let flags = ap_proxy.get_flags().await?;
        let wpa_flags = ap_proxy.get_wpa_flags().await?;
        let rsn_flags = ap_proxy.get_rsn_flags().await?;
        
        if wpa_flags > 0 || rsn_flags > 0 {
            Ok("WPA/WPA2".to_string())
        } else if flags & 0x1 != 0 { // NM_802_11_AP_FLAGS_PRIVACY
            Ok("WEP".to_string())
        } else {
            Ok("Open".to_string())
        }
    }

    pub async fn connect_to_network(&self, _ssid: &str, _password: Option<&str>) -> Result<()> {
        // This is a simplified version. In a real implementation,
        // you would need to create a connection configuration
        // and activate it through NetworkManager
        Err(anyhow!("Connection via D-Bus not implemented yet. Use nmcli for now."))
    }

    pub async fn disconnect(&self) -> Result<()> {
        // Similar to connect, this would need to deactivate
        // the current connection through NetworkManager
        Err(anyhow!("Disconnection via D-Bus not implemented yet. Use nmcli for now."))
    }

    pub async fn get_current_ssid(&self) -> Result<Option<String>> {
        let active_connections = self.nm_proxy.get_active_connections().await?;
        
        for conn_path in active_connections {
            let conn_proxy = ActiveConnectionProxy::builder(&self.connection)
                .path(conn_path)?
                .build()
                .await?;
            
            let conn_state = conn_proxy.get_state().await?;
            if conn_state == 2 { // NM_ACTIVE_CONNECTION_STATE_ACTIVATED
                // This is simplified - in reality you'd need to get the connection
                // settings to extract the SSID
                return Ok(None);
            }
        }
        
        Ok(None)
    }
}

impl WifiNetwork {
    pub fn signal_percentage(&self) -> u8 {
        self.signal.try_into().unwrap_or(0)
    }

    pub fn signal_description(&self) -> &'static str {
        match self.signal {
            80..=100 => "Excellent",
            60..=79 => "Good",
            40..=59 => "Fair",
            20..=39 => "Poor",
            _ => "Very Poor",
        }
    }
} 