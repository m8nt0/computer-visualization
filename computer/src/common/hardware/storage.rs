// Storage interfaces
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum StorageType {
    SSD,
    HDD,
    NVMe,
    RAID,
    Other(String),
}

impl StorageType {
    pub fn as_str(&self) -> &str {
        match self {
            StorageType::SSD => "SSD",
            StorageType::HDD => "HDD",
            StorageType::NVMe => "NVMe",
            StorageType::RAID => "RAID",
            StorageType::Other(s) => s,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Storage {
    name: String,
    capacity: u64,     // in GB
    storage_type: StorageType,
    read_speed: u32,   // in MB/s
    write_speed: u32,  // in MB/s
    // Simulated file system
    contents: HashMap<String, Vec<u8>>,
}

impl Storage {
    pub fn new(name: &str, capacity: u64, storage_type: StorageType, read_speed: u32, write_speed: u32) -> Self {
        Self {
            name: name.to_string(),
            capacity,
            storage_type,
            read_speed,
            write_speed,
            contents: HashMap::new(),
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn capacity(&self) -> u64 {
        self.capacity
    }
    
    pub fn storage_type(&self) -> &StorageType {
        &self.storage_type
    }
    
    pub fn read_speed(&self) -> u32 {
        self.read_speed
    }
    
    pub fn write_speed(&self) -> u32 {
        self.write_speed
    }
    
    // Simulated read operation
    pub fn read(&self, path: &str) -> Option<&Vec<u8>> {
        self.contents.get(path)
    }
    
    // Simulated write operation
    pub fn write(&mut self, path: &str, data: Vec<u8>) -> Result<(), &'static str> {
        let total_size: u64 = self.contents.values().map(|v| v.len() as u64).sum::<u64>() + data.len() as u64;
        
        if total_size > self.capacity * 1_000_000_000 {
            return Err("Not enough storage space");
        }
        
        self.contents.insert(path.to_string(), data);
        Ok(())
    }
    
    // Simulated delete operation
    pub fn delete(&mut self, path: &str) -> bool {
        self.contents.remove(path).is_some()
    }
} 