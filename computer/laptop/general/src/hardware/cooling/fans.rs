// Fan component implementation
pub struct Fan {
    name: String,
    current_speed: u8, // 0-100%
    max_rpm: u16,
    is_running: bool,
}

impl Fan {
    pub fn new(name: &str, initial_speed: u8, max_rpm: u16) -> Self {
        Self {
            name: name.to_string(),
            current_speed: initial_speed,
            max_rpm,
            is_running: initial_speed > 0,
        }
    }
    
    pub fn set_speed(&mut self, speed: u8) {
        let clamped_speed = speed.min(100);
        self.current_speed = clamped_speed;
        self.is_running = clamped_speed > 0;
    }
    
    pub fn get_speed(&self) -> u8 {
        self.current_speed
    }
    
    pub fn get_rpm(&self) -> u16 {
        // Calculate actual RPM based on percentage
        (self.max_rpm as f32 * (self.current_speed as f32 / 100.0)) as u16
    }
    
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    
    pub fn get_cooling_power(&self) -> f32 {
        // Cooling effect is non-linear - increases more at higher speeds
        (self.current_speed as f32 / 100.0).powf(1.5) * 100.0
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
} 