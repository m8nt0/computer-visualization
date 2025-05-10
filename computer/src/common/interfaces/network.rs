// Networking primitives
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, Clone, PartialEq)]
pub enum Protocol {
    TCP,
    UDP,
    HTTP,
    HTTPS,
    FTP,
    SSH,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct Packet {
    protocol: Protocol,
    source: SocketAddr,
    destination: SocketAddr,
    data: Vec<u8>,
}

impl Packet {
    pub fn new(protocol: Protocol, source: SocketAddr, destination: SocketAddr, data: Vec<u8>) -> Self {
        Self {
            protocol,
            source,
            destination,
            data,
        }
    }
    
    pub fn protocol(&self) -> &Protocol {
        &self.protocol
    }
    
    pub fn source(&self) -> &SocketAddr {
        &self.source
    }
    
    pub fn destination(&self) -> &SocketAddr {
        &self.destination
    }
    
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

pub trait NetworkCapable {
    fn get_ip_address(&self) -> Option<IpAddr>;
    fn send_packet(&mut self, packet: Packet) -> Result<(), &'static str>;
    fn receive_packet(&mut self) -> Option<Packet>;
    fn is_connected(&self) -> bool;
}

// Network connection implementation
pub struct NetworkConnection {
    connected: bool,
    ip_address: Option<IpAddr>,
    received_packets: Vec<Packet>,
}

impl NetworkConnection {
    pub fn new() -> Self {
        Self {
            connected: false,
            ip_address: None,
            received_packets: Vec::new(),
        }
    }
    
    pub fn connect(&mut self, ip: IpAddr) -> Result<(), &'static str> {
        self.connected = true;
        self.ip_address = Some(ip);
        Ok(())
    }
    
    pub fn disconnect(&mut self) -> Result<(), &'static str> {
        self.connected = false;
        self.ip_address = None;
        Ok(())
    }
    
    // Simulate receiving a packet
    pub fn queue_packet(&mut self, packet: Packet) {
        self.received_packets.push(packet);
    }
}

impl NetworkCapable for NetworkConnection {
    fn get_ip_address(&self) -> Option<IpAddr> {
        self.ip_address
    }
    
    fn send_packet(&mut self, _packet: Packet) -> Result<(), &'static str> {
        if !self.connected {
            return Err("Not connected to a network");
        }
        
        // Simulate sending a packet
        Ok(())
    }
    
    fn receive_packet(&mut self) -> Option<Packet> {
        if !self.connected || self.received_packets.is_empty() {
            return None;
        }
        
        Some(self.received_packets.remove(0))
    }
    
    fn is_connected(&self) -> bool {
        self.connected
    }
} 