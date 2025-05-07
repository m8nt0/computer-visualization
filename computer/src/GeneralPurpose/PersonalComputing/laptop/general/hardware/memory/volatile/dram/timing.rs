use std::collections::HashMap;

pub struct TimingController {
    // Timing parameters (in cycles)
    tCK: u32,  // Clock cycle time
    tRCD: u32, // RAS to CAS delay
    tRP: u32,  // Row precharge time
    tRAS: u32, // Row active time
    tRC: u32,  // Row cycle time
    tWR: u32,  // Write recovery time
    tCCD: u32, // Column to column delay
    tRRD: u32, // Row to row delay
    tWTR: u32, // Write to read delay
    tRTP: u32, // Read to precharge delay
    
    // Last operation timestamps
    last_activate: HashMap<(usize, usize), u64>, // (rank, bank) -> timestamp
    last_precharge: HashMap<(usize, usize), u64>,
    last_read: HashMap<(usize, usize), u64>,
    last_write: HashMap<(usize, usize), u64>,
    
    current_cycle: u64,
}

impl TimingController {
    pub fn new() -> Self {
        Self {
            // DDR4-2400 timings
            tCK: 1,
            tRCD: 14,
            tRP: 14,
            tRAS: 32,
            tRC: 46,
            tWR: 15,
            tCCD: 4,
            tRRD: 4,
            tWTR: 8,
            tRTP: 8,
            
            last_activate: HashMap::new(),
            last_precharge: HashMap::new(),
            last_read: HashMap::new(),
            last_write: HashMap::new(),
            
            current_cycle: 0,
        }
    }

    pub fn tick(&mut self) {
        self.current_cycle += 1;
    }

    pub fn can_activate(&self, rank: usize, bank: usize) -> bool {
        let key = (rank, bank);
        
        // Check tRP (time since last precharge)
        if let Some(&last) = self.last_precharge.get(&key) {
            if self.current_cycle < last + self.tRP as u64 {
                return false;
            }
        }
        
        // Check tRC (time since last activate)
        if let Some(&last) = self.last_activate.get(&key) {
            if self.current_cycle < last + self.tRC as u64 {
                return false;
            }
        }
        
        true
    }

    pub fn can_read(&self, rank: usize, bank: usize) -> bool {
        let key = (rank, bank);
        
        // Check tRCD (time since activate)
        if let Some(&last) = self.last_activate.get(&key) {
            if self.current_cycle < last + self.tRCD as u64 {
                return false;
            }
        }
        
        // Check tCCD (time since last read)
        if let Some(&last) = self.last_read.get(&key) {
            if self.current_cycle < last + self.tCCD as u64 {
                return false;
            }
        }
        
        // Check tWTR (time since last write)
        if let Some(&last) = self.last_write.get(&key) {
            if self.current_cycle < last + self.tWTR as u64 {
                return false;
            }
        }
        
        true
    }

    pub fn can_write(&self, rank: usize, bank: usize) -> bool {
        let key = (rank, bank);
        
        // Check tRCD (time since activate)
        if let Some(&last) = self.last_activate.get(&key) {
            if self.current_cycle < last + self.tRCD as u64 {
                return false;
            }
        }
        
        // Check tCCD (time since last write)
        if let Some(&last) = self.last_write.get(&key) {
            if self.current_cycle < last + self.tCCD as u64 {
                return false;
            }
        }
        
        true
    }

    pub fn record_activate(&mut self, rank: usize, bank: usize) {
        self.last_activate.insert((rank, bank), self.current_cycle);
    }

    pub fn record_precharge(&mut self, rank: usize, bank: usize) {
        self.last_precharge.insert((rank, bank), self.current_cycle);
    }

    pub fn record_read(&mut self, rank: usize, bank: usize) {
        self.last_read.insert((rank, bank), self.current_cycle);
    }

    pub fn record_write(&mut self, rank: usize, bank: usize) {
        self.last_write.insert((rank, bank), self.current_cycle);
    }

    pub fn get_read_latency(&self) -> u32 {
        self.tRCD + 4 // tRCD + CAS latency
    }

    pub fn get_write_latency(&self) -> u32 {
        self.tRCD + self.tWR
    }
}
