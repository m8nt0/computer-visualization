// External device connections

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionType {
    USB,
    Thunderbolt,
    HDMI,
    DisplayPort,
    Bluetooth,
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum PeripheralType {
    Mouse,
    Keyboard,
    Printer,
    Scanner,
    Camera,
    Storage,
    Audio,
    Display,
    Other(String),
}

pub trait Peripheral {
    fn peripheral_type(&self) -> PeripheralType;
    fn connection_type(&self) -> ConnectionType;
    fn is_connected(&self) -> bool;
    fn connect(&mut self) -> Result<(), &'static str>;
    fn disconnect(&mut self) -> Result<(), &'static str>;
    fn send_data(&mut self, data: &[u8]) -> Result<(), &'static str>;
    fn receive_data(&mut self) -> Option<Vec<u8>>;
}

// USB port implementation
pub struct UsbPort {
    port_number: u8,
    connected_device: Option<Box<dyn Peripheral>>,
    version: String, // USB version, e.g., "2.0", "3.0"
}

impl UsbPort {
    pub fn new(port_number: u8, version: &str) -> Self {
        Self {
            port_number,
            connected_device: None,
            version: version.to_string(),
        }
    }
    
    pub fn port_number(&self) -> u8 {
        self.port_number
    }
    
    pub fn version(&self) -> &str {
        &self.version
    }
    
    pub fn has_device(&self) -> bool {
        self.connected_device.is_some()
    }
    
    pub fn connect_device(&mut self, device: Box<dyn Peripheral>) -> Result<(), &'static str> {
        if self.has_device() {
            return Err("Port already has a device connected");
        }
        
        let mut device = device;
        device.connect()?;
        self.connected_device = Some(device);
        
        Ok(())
    }
    
    pub fn disconnect_device(&mut self) -> Result<(), &'static str> {
        if let Some(mut device) = self.connected_device.take() {
            device.disconnect()?;
            Ok(())
        } else {
            Err("No device connected to disconnect")
        }
    }
    
    pub fn send_to_device(&mut self, data: &[u8]) -> Result<(), &'static str> {
        if let Some(device) = self.connected_device.as_mut() {
            device.send_data(data)
        } else {
            Err("No device connected to send data to")
        }
    }
    
    pub fn receive_from_device(&mut self) -> Option<Vec<u8>> {
        if let Some(device) = self.connected_device.as_mut() {
            device.receive_data()
        } else {
            None
        }
    }
} 