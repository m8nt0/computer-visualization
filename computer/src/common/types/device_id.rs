// Unique identifier for devices
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeviceId {
    id: String,
    device_type: DeviceType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Desktop,
    Laptop,
    Server,
    Tablet,
    Smartphone,
    Embedded,
    IoT,
    Virtual,
    Other,
}

impl DeviceId {
    pub fn new(id: &str, device_type: DeviceType) -> Self {
        Self {
            id: id.to_string(),
            device_type,
        }
    }
    
    pub fn id(&self) -> &str {
        &self.id
    }
    
    pub fn device_type(&self) -> DeviceType {
        self.device_type
    }
    
    pub fn is_valid(&self) -> bool {
        // Basic validation - ID should be non-empty and follow a certain format
        // In a real system, this would be more sophisticated
        !self.id.is_empty() && self.id.len() >= 4
    }
    
    pub fn generate() -> Self {
        // Simple ID generation with timestamp and random number
        // In a real system, this would use a proper UUID or similar
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
            
        let random_component = rand();
        
        Self {
            id: format!("DEV-{}-{:04X}", timestamp, random_component),
            device_type: DeviceType::Other,
        }
    }
    
    pub fn with_device_type(mut self, device_type: DeviceType) -> Self {
        self.device_type = device_type;
        self
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl FromStr for DeviceId {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Device ID cannot be empty".to_string());
        }
        
        // Try to parse device type from the ID
        let device_type = if s.starts_with("DSK-") {
            DeviceType::Desktop
        } else if s.starts_with("LPT-") {
            DeviceType::Laptop
        } else if s.starts_with("SRV-") {
            DeviceType::Server
        } else if s.starts_with("TAB-") {
            DeviceType::Tablet
        } else if s.starts_with("SPH-") {
            DeviceType::Smartphone
        } else if s.starts_with("EMB-") {
            DeviceType::Embedded
        } else if s.starts_with("IOT-") {
            DeviceType::IoT
        } else if s.starts_with("VRT-") {
            DeviceType::Virtual
        } else {
            DeviceType::Other
        };
        
        Ok(DeviceId {
            id: s.to_string(),
            device_type,
        })
    }
}

impl From<&str> for DeviceId {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap_or_else(|_| Self::new(s, DeviceType::Other))
    }
}

impl DeviceType {
    pub fn as_str(&self) -> &'static str {
        match self {
            DeviceType::Desktop => "Desktop",
            DeviceType::Laptop => "Laptop",
            DeviceType::Server => "Server",
            DeviceType::Tablet => "Tablet",
            DeviceType::Smartphone => "Smartphone",
            DeviceType::Embedded => "Embedded",
            DeviceType::IoT => "IoT",
            DeviceType::Virtual => "Virtual",
            DeviceType::Other => "Other",
        }
    }
}

// Simple pseudo-random number generator
// This is for demonstration only - in a real system, use a proper RNG
fn rand() -> u16 {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
        
    (nanos % 65536) as u16
}

