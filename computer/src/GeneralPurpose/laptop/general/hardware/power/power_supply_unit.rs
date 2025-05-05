// Power Supply Unit implementation
pub struct PowerSupplyUnit {
    max_power: u32, // Watts
    current_power: u32, // Watts
    efficiency: f32, // 0-1.0 (percentage)
    temperature: f32, // Celsius
    is_connected: bool,
    is_initialized: bool,
}

impl PowerSupplyUnit {
    pub fn new() -> Self {
        Self {
            max_power: 65, // 65W typical for laptop
            current_power: 0,
            efficiency: 0.85, // 85% efficiency
            temperature: 30.0,
            is_connected: true, // Assume connected by default
            is_initialized: false,
        }
    }
    
    pub fn initialize(&mut self) -> bool {
        println!("PSU initialized: {}W max power", self.max_power);
        self.is_initialized = true;
        self.current_power = 5; // Baseline power draw
        true
    }
    
    pub fn shutdown(&mut self) -> bool {
        self.is_initialized = false;
        self.current_power = 0;
        true
    }
    
    pub fn connect(&mut self) {
        self.is_connected = true;
    }
    
    pub fn disconnect(&mut self) {
        self.is_connected = false;
    }
    
    pub fn is_connected(&self) -> bool {
        self.is_connected
    }
    
    pub fn set_power_draw(&mut self, power: u32) {
        if !self.is_initialized {
            return;
        }
        
        self.current_power = power.min(self.max_power);
        
        // Update temperature based on power draw
        // Higher power draw means more heat generation
        let power_ratio = self.current_power as f32 / self.max_power as f32;
        self.temperature = 30.0 + 20.0 * power_ratio;
    }
    
    pub fn get_power_draw(&self) -> u32 {
        self.current_power
    }
    
    pub fn get_efficiency(&self) -> f32 {
        // Efficiency drops slightly with higher loads
        let load_factor = self.current_power as f32 / self.max_power as f32;
        self.efficiency * (1.0 - 0.1 * load_factor)
    }
    
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    
    pub fn get_actual_draw(&self) -> f32 {
        // Account for efficiency loss
        self.current_power as f32 / self.get_efficiency()
    }
    
    pub fn is_overloaded(&self) -> bool {
        self.current_power >= self.max_power
    }
} 