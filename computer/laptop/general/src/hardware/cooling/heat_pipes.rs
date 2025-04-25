// Heat pipe component implementation
pub struct HeatPipe {
    name: String,
    efficiency: f32, // 0-1.0
    length: f32, // mm
    temperature: f32, // Celsius
    heat_capacity: f32, // Joules per degree C
}

impl HeatPipe {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            efficiency: 0.85,
            length: 150.0, // 150mm
            temperature: 30.0,
            heat_capacity: 120.0,
        }
    }
    
    pub fn with_specs(name: &str, length: f32, efficiency: f32) -> Self {
        Self {
            name: name.to_string(),
            efficiency,
            length,
            temperature: 30.0,
            heat_capacity: length * 0.8, // Heat capacity related to length
        }
    }
    
    pub fn transfer_heat(&mut self, source_temp: f32, sink_temp: f32) -> f32 {
        // Calculate heat transfer
        let temp_difference = source_temp - sink_temp;
        
        if temp_difference <= 0.0 {
            return 0.0; // No heat transfer when sink is hotter than source
        }
        
        // Heat transfer is proportional to temperature difference, efficiency, and length
        let heat_transferred = temp_difference * self.efficiency * (self.length / 100.0);
        
        // Update heat pipe temperature
        self.temperature = source_temp - (heat_transferred / self.heat_capacity);
        
        // Return amount of heat transferred
        heat_transferred
    }
    
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    
    pub fn get_efficiency(&self) -> f32 {
        self.efficiency
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn get_heat_dissipation_rate(&self) -> f32 {
        // Rate at which heat is dissipated (in arbitrary units)
        self.efficiency * self.length / 10.0
    }
} 