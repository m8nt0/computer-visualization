// Network/communication interfaces
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Clone, PartialEq)]
pub enum NetworkType {
    Ethernet,
    Wifi,
    Bluetooth,
    Cellular,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    name: String,
    network_type: NetworkType,
    mac_address: String,
    ip_address: Option<IpAddr>,
    connected: bool,
    speed: u32, // in Mbps
}

impl NetworkInterface {
    pub fn new(name: &str, network_type: NetworkType, mac_address: &str, speed: u32) -> Self {
        Self {
            name: name.to_string(),
            network_type,
            mac_address: mac_address.to_string(),
            ip_address: None,
            connected: false,
            speed,
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn network_type(&self) -> &NetworkType {
        &self.network_type
    }
    
    pub fn mac_address(&self) -> &str {
        &self.mac_address
    }
    
    pub fn ip_address(&self) -> Option<IpAddr> {
        self.ip_address
    }
    
    pub fn is_connected(&self) -> bool {
        self.connected
    }
    
    pub fn speed(&self) -> u32 {
        self.speed
    }
    
    // Connect to a network
    pub fn connect(&mut self) -> Result<(), &'static str> {
        if self.connected {
            return Err("Already connected");
        }
        
        // Simulate connection
        self.connected = true;
        // Assign a dummy IP address
        self.ip_address = Some(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)));
        
        Ok(())
    }
    
    // Disconnect from a network
    pub fn disconnect(&mut self) -> Result<(), &'static str> {
        if !self.connected {
            return Err("Not connected");
        }
        
        self.connected = false;
        self.ip_address = None;
        
        Ok(())
    }
} 