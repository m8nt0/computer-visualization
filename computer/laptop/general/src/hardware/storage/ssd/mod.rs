// Basic SSD implementation
pub struct SSD {
    capacity: u32, // GB
    speed_read: u16, // MB/s
    speed_write: u16, // MB/s
    usage: f32, // 0-100%
    temperature: f32, // Celsius
    is_initialized: bool,
    health: f32, // 0-100% (wear level)
    data_blocks: Vec<Block>,
}

struct Block {
    id: usize,
    data: Vec<u8>,
    is_free: bool,
    write_count: u32, // For wear leveling
}

impl SSD {
    pub fn new() -> Self {
        Self {
            capacity: 512,
            speed_read: 550,
            speed_write: 520,
            usage: 0.0,
            temperature: 30.0,
            is_initialized: false,
            health: 100.0,
            data_blocks: Vec::new(),
        }
    }
    
    pub fn initialize(&mut self) -> bool {
        println!("SSD initializing with {}GB capacity", self.capacity);
        self.is_initialized = true;
        
        // Create storage blocks
        const BLOCK_SIZE: usize = 4096; // 4KB blocks
        let num_blocks = (self.capacity as usize) * 1024 * 1024 / BLOCK_SIZE;
        
        // Only create a representative sample of blocks to avoid excessive memory usage
        let sample_blocks = 1000.min(num_blocks);
        
        self.data_blocks = (0..sample_blocks)
            .map(|id| Block {
                id,
                data: vec![0; BLOCK_SIZE],
                is_free: true,
                write_count: 0,
            })
            .collect();
        
        // Some blocks used by filesystem
        for i in 0..50 {
            if let Some(block) = self.data_blocks.get_mut(i) {
                block.is_free = false;
                // INSERT_YOUR_REWRITE_HERE
            }
        }
        self.update_usage();
        true
    }
    
    // pub fn random_data(size: usize) -> Vec<u8> {
    //     let mut data = vec![0; size];
    //     for i in 0..size {
    //         data[i] = random::<u8>();
    //     }
    //     data
    // }

    pub fn shutdown(&mut self) -> bool {
        println!("SSD shutting down");
        self.is_initialized = false;
        self.temperature = 30.0;
        true
    }
    
    pub fn read(&mut self, block_id: usize) -> Option<&[u8]> {
        if !self.is_initialized {
            return None;
        }
        
        // Simulate read operation
        std::thread::sleep(std::time::Duration::from_micros(10));
        
        // Slight temperature increase
        self.temperature = self.temperature * 0.98 + 32.0 * 0.02;
        
        // Find the block
        self.data_blocks.iter()
            .find(|block| block.id == block_id)
            .map(|block| block.data.as_slice())
    }
    
    pub fn write(&mut self, block_id: usize, data: &[u8]) -> bool {
        if !self.is_initialized {
            return false;
        }
        
        // Find the block
        let block = match self.data_blocks.iter_mut().find(|block| block.id == block_id) {
            Some(block) => block,
            None => return false,
        };
        
        // Copy data to block
        let len = block.data.len().min(data.len());
        block.data[0..len].copy_from_slice(&data[0..len]);
        block.is_free = false;
        block.write_count += 1;
        
        // Wear leveling - health decreases with writes
        self.health -= 0.0001;
        self.health = self.health.max(0.0);
        
        // Temperature increases with writes
        self.temperature = self.temperature * 0.95 + 35.0 * 0.05;
        
        // Simulate write operation delay
        std::thread::sleep(std::time::Duration::from_micros(20));
        
        self.update_usage();
        true
    }
    
    pub fn allocate(&mut self, size: usize) -> Option<usize> {
        if !self.is_initialized {
            return None;
        }
        
        // Find a free block
        let blocks_needed = (size + 4095) / 4096; // Round up
        
        if blocks_needed > 1 {
            // Find consecutive blocks (simplified)
            let mut consecutive = 0;
            let mut start_id = 0;
            
            for block in &self.data_blocks {
                if block.is_free {
                    if consecutive == 0 {
                        start_id = block.id;
                    }
                    consecutive += 1;
                    if consecutive >= blocks_needed {
                        return Some(start_id);
                    }
                } else {
                    consecutive = 0;
                }
            }
            
            None
        } else {
            // Find any free block
            self.data_blocks.iter()
                .find(|block| block.is_free)
                .map(|block| block.id)
        }
    }
    
    pub fn free(&mut self, block_id: usize, num_blocks: usize) -> bool {
        if !self.is_initialized {
            return false;
        }
        
        let mut found = false;
        
        for i in 0..num_blocks {
            if let Some(block) = self.data_blocks.iter_mut()
                .find(|block| block.id == block_id + i) {
                block.is_free = true;
                // Zero out the data
                for byte in &mut block.data {
                    *byte = 0;
                }
                found = true;
            }
        }
        
        if found {
            self.update_usage();
        }
        
        found
    }
    
    fn update_usage(&mut self) {
        let used_blocks = self.data_blocks.iter().filter(|block| !block.is_free).count();
        self.usage = (used_blocks as f32 / self.data_blocks.len() as f32) * 100.0;
    }
    
    pub fn get_usage(&self) -> f32 {
        self.usage
    }
    
    pub fn get_health(&self) -> f32 {
        self.health
    }
    
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    
    pub fn get_status(&self) -> String {
        format!("{:.1}% of {}GB used, Health: {:.1}%, Temp: {:.1}°C", 
                self.usage, self.capacity, self.health, self.temperature)
    }
} 