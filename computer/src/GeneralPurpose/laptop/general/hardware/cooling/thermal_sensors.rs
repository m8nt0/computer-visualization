// Thermal sensor implementation
pub struct ThermalSensor {
    name: String,
    temperature: f32, // Celsius
    accuracy: f32, // +/- degrees
    max_temp: f32, // Maximum safe temperature
    is_critical: bool,
}

impl ThermalSensor {
    pub fn new(name: &str, initial_temp: f32) -> Self {
        Self {
            name: name.to_string(),
            temperature: initial_temp,
            accuracy: 0.5, // +/- 0.5 degrees C
            max_temp: 100.0, // Default maximum safe temperature
            is_critical: false,
        }
    }
    
    pub fn with_settings(name: &str, initial_temp: f32, accuracy: f32, max_temp: f32) -> Self {
        Self {
            name: name.to_string(),
            temperature: initial_temp,
            accuracy,
            max_temp,
            is_critical: false,
        }
    }
    
    pub fn update_temperature(&mut self, new_temp: f32) {
        self.temperature = new_temp;
        self.is_critical = new_temp >= self.max_temp;
    }
    
    pub fn get_temperature(&self) -> f32 {
        // Could simulate some sensor noise
        self.temperature
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn is_critical(&self) -> bool {
        self.is_critical
    }
    
    pub fn get_temperature_status(&self) -> TemperatureStatus {
        let temp = self.temperature;
        
        if temp >= self.max_temp {
            TemperatureStatus::Critical
        } else if temp >= self.max_temp * 0.9 {
            TemperatureStatus::Hot
        } else if temp >= self.max_temp * 0.7 {
            TemperatureStatus::Warm
        } else {
            TemperatureStatus::Normal
        }
    }
    
    pub fn get_accuracy(&self) -> f32 {
        self.accuracy
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureStatus {
    Normal,
    Warm,
    Hot,
    Critical,
} 