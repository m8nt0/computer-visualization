// Battery component implementation
pub struct Battery {
    capacity: u32, // mAh
    current_level: u32, // mAh
    voltage: f32, // Volts
    discharge_rate: f32, // mA
    cycle_count: u32,
    temperature: f32, // Celsius
    is_charging: bool,
    is_initialized: bool,
}

impl Battery {
    pub fn new() -> Self {
        Self {
            capacity: 5000, // 5000 mAh typical laptop battery
            current_level: 4000, // 80% charged by default
            voltage: 11.1, // 11.1V typical
            discharge_rate: 0.0,
            cycle_count: 0,
            temperature: 30.0,
            is_charging: false,
            is_initialized: false,
        }
    }
    
    pub fn initialize(&mut self) -> bool {
        println!("Battery initialized: {}mAh capacity", self.capacity);
        self.is_initialized = true;
        true
    }
    
    pub fn shutdown(&mut self) -> bool {
        self.is_initialized = false;
        true
    }
    
    pub fn charge(&mut self, rate: f32) {
        if !self.is_initialized {
            return;
        }
        
        if !self.is_charging {
            self.is_charging = true;
        }
        
        // Calculate new charge level
        let new_level = self.current_level + (rate as u32);
        self.current_level = new_level.min(self.capacity);
        
        // Battery warms during charging
        self.temperature += 0.01 * rate;
        
        // If fully charged, stop charging
        if self.current_level >= self.capacity {
            self.is_charging = false;
            // Increment cycle count when fully charged
            if self.current_level == self.capacity {
                self.cycle_count += 1;
            }
        }
    }
    
    pub fn discharge(&mut self, rate: f32) {
        if !self.is_initialized {
            return;
        }
        
        self.discharge_rate = rate;
        self.is_charging = false;
        
        // Calculate new charge level
        if self.current_level > rate as u32 {
            self.current_level -= rate as u32;
        } else {
            self.current_level = 0;
        }
        
        // Temperature increases with discharge rate
        self.temperature += 0.005 * rate;
    }
    
    pub fn get_percentage(&self) -> u8 {
        if self.capacity == 0 {
            return 0;
        }
        
        ((self.current_level as f32 / self.capacity as f32) * 100.0) as u8
    }
    
    pub fn get_voltage(&self) -> f32 {
        // Voltage drops as battery discharges
        let percentage = self.get_percentage() as f32 / 100.0;
        self.voltage * (0.85 + 0.15 * percentage)
    }
    
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    
    pub fn get_health(&self) -> u8 {
        // Health decreases with cycle count
        // Typical battery has ~500 cycles before significant degradation
        let health_factor = 1.0 - (self.cycle_count as f32 / 1000.0).min(1.0);
        (health_factor * 100.0) as u8
    }
    
    pub fn is_charging(&self) -> bool {
        self.is_charging
    }
    
    pub fn get_remaining_time(&self) -> Option<u32> {
        if !self.is_initialized || self.discharge_rate <= 0.0 {
            return None;
        }
        
        // Calculate remaining time in minutes
        Some((self.current_level as f32 / self.discharge_rate * 60.0) as u32)
    }
} 