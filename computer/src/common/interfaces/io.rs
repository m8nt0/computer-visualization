// Input/output interfaces

#[derive(Debug, Clone, PartialEq)]
pub enum InputType {
    Keyboard,
    Mouse,
    Touch,
    Voice,
    Gamepad,
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputType {
    Display,
    Audio,
    Haptic,
    Printer,
    Other(String),
}

pub trait InputDevice {
    fn device_type(&self) -> InputType;
    fn receive_input(&mut self) -> Vec<u8>;
    fn is_connected(&self) -> bool;
}

pub trait OutputDevice {
    fn device_type(&self) -> OutputType;
    fn send_output(&mut self, data: &[u8]) -> Result<(), &'static str>;
    fn is_connected(&self) -> bool;
}

// Basic keyboard implementation
pub struct Keyboard {
    name: String,
    connected: bool,
    buffer: Vec<u8>,
}

impl Keyboard {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            connected: false,
            buffer: Vec::new(),
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn connect(&mut self) -> Result<(), &'static str> {
        self.connected = true;
        Ok(())
    }
    
    pub fn disconnect(&mut self) -> Result<(), &'static str> {
        self.connected = false;
        Ok(())
    }
    
    // Simulate key press
    pub fn press_key(&mut self, key: u8) {
        self.buffer.push(key);
    }
}

impl InputDevice for Keyboard {
    fn device_type(&self) -> InputType {
        InputType::Keyboard
    }
    
    fn receive_input(&mut self) -> Vec<u8> {
        let input = self.buffer.clone();
        self.buffer.clear();
        input
    }
    
    fn is_connected(&self) -> bool {
        self.connected
    }
}

// Basic display implementation
pub struct Display {
    name: String,
    resolution: (u32, u32),
    connected: bool,
}

impl Display {
    pub fn new(name: &str, width: u32, height: u32) -> Self {
        Self {
            name: name.to_string(),
            resolution: (width, height),
            connected: false,
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn resolution(&self) -> (u32, u32) {
        self.resolution
    }
    
    pub fn connect(&mut self) -> Result<(), &'static str> {
        self.connected = true;
        Ok(())
    }
    
    pub fn disconnect(&mut self) -> Result<(), &'static str> {
        self.connected = false;
        Ok(())
    }
}

impl OutputDevice for Display {
    fn device_type(&self) -> OutputType {
        OutputType::Display
    }
    
    fn send_output(&mut self, _data: &[u8]) -> Result<(), &'static str> {
        if !self.connected {
            return Err("Display not connected");
        }
        
        // Simulate displaying data
        Ok(())
    }
    
    fn is_connected(&self) -> bool {
        self.connected
    }
} 