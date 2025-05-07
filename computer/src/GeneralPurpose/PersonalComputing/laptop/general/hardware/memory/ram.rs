// Basic RAM implementation
pub struct RAM {
    capacity: u32, // GB
    speed: u16, // MHz
    usage: f32, // 0-100%
    temperature: f32, // Celsius
    is_initialized: bool,
    data: Vec<u8>, // Simple representation of memory contents
}

impl RAM {
    pub fn new() -> Self {
        Self {
            capacity: 16,
            speed: 3200,
            usage: 0.0,
            temperature: 30.0,
            is_initialized: false,
            data: Vec::new(),
        }
    }
    
    pub fn initialize(&mut self) -> bool {
        println!("RAM initializing with {}GB at {}MHz", self.capacity, self.speed);
        self.is_initialized = true;
        self.data = vec![0; (self.capacity as usize) * 1024 * 1024]; // Simulated memory space
        self.usage = 15.0; // OS and basic services take some memory
        true
    }
    
    pub fn shutdown(&mut self) -> bool {
        println!("RAM shutting down");
        self.is_initialized = false;
        self.usage = 0.0;
        self.data.clear();
        true
    }
    
    pub fn read(&self, address: usize, size: usize) -> Vec<u8> {
        if !self.is_initialized || address >= self.data.len() || address + size > self.data.len() {
            return vec![0xFF]; // Error code
        }
        
        self.data[address..address+size].to_vec()
    }
    
    pub fn write(&mut self, address: usize, data: &[u8]) -> bool {
        if !self.is_initialized || address >= self.data.len() || address + data.len() > self.data.len() {
            return false;
        }
        
        // Copy data to memory
        for (i, &byte) in data.iter().enumerate() {
            self.data[address + i] = byte;
        }
        
        // Update usage based on a simple heuristic
        self.usage = (self.data.iter().filter(|&&b| b != 0).count() as f32 / self.data.len() as f32) * 100.0;
        self.usage = (self.usage.max(15.0)).min(100.0); // Ensure at least 15% for system usage
        
        // Memory operations increase temperature slightly
        self.temperature = self.temperature * 0.95 + 35.0 * 0.05;
        
        true
    }
    
    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if !self.is_initialized {
            return None;
        }
        
        // Simple allocation strategy - find first block of zeros
        let mut start_address = 0;
        let mut consecutive_zeros = 0;
        
        for (index, &byte) in self.data.iter().enumerate() {
            if byte == 0 {
                if consecutive_zeros == 0 {
                    start_address = index;
                }
                consecutive_zeros += 1;
                if consecutive_zeros >= size {
                    // Zero out the allocated memory
                    for i in 0..size {
                        self.data[start_address + i] = 0xFF; // Mark as allocated
                    }
                    
                    // Update usage
                    self.usage = (self.data.iter().filter(|&&b| b != 0).count() as f32 / self.data.len() as f32) * 100.0;
                    self.usage = (self.usage.max(15.0)).min(100.0); // Ensure at least 15% for system usage
                    
                    return Some(start_address);
                }
            } else {
                consecutive_zeros = 0;
            }
        }
        
        None // No space found
    }
    
    pub fn free(&mut self, address: usize, size: usize) -> bool {
        if !self.is_initialized || address >= self.data.len() || address + size > self.data.len() {
            return false;
        }
        
        // Zero out the memory region
        for i in 0..size {
            self.data[address + i] = 0;
        }
        
        // Update usage
        self.usage = (self.data.iter().filter(|&&b| b != 0).count() as f32 / self.data.len() as f32) * 100.0;
        self.usage = (self.usage.max(15.0)).min(100.0); // Ensure at least 15% for system usage
        
        true
    }
    
    pub fn get_usage(&self) -> f32 {
        self.usage
    }
    
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    
    pub fn get_status(&self) -> String {
        format!("{:.1}% of {}GB @ {:.1}°C", self.usage, self.capacity, self.temperature)
    }
} 