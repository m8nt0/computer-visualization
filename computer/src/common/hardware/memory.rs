// Memory management
#[derive(Debug, Clone)]
pub struct Memory {
    name: String,
    capacity: u64, // in MB
    type_: String, // e.g., DDR4, DDR5
    speed: u32,    // in MHz
}

impl Memory {
    pub fn new(name: &str, capacity: u64, type_: &str, speed: u32) -> Self {
        Self {
            name: name.to_string(),
            capacity,
            type_: type_.to_string(),
            speed,
        }
    }
    
    pub fn capacity(&self) -> u64 {
        self.capacity
    }
    
    pub fn type_(&self) -> &str {
        &self.type_
    }
    
    pub fn speed(&self) -> u32 {
        self.speed
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    // Memory allocation simulation
    pub fn allocate(&self, size: u64) -> Result<MemoryAllocation, &'static str> {
        if size > self.capacity {
            return Err("Requested allocation exceeds available memory");
        }
        
        Ok(MemoryAllocation {
            size,
            memory_type: self.type_.clone(),
        })
    }
    
    // Memory deallocation simulation
    pub fn deallocate(&self, _allocation: MemoryAllocation) -> Result<(), &'static str> {
        // Simple simulation - in a real system, this would handle the actual deallocation
        Ok(())
    }
}

#[derive(Debug)]
pub struct MemoryAllocation {
    size: u64,
    memory_type: String,
} 