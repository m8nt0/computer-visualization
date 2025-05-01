use crate::hardware::cooling::fans::Fan;
use crate::hardware::cooling::heat_pipes::HeatPipe;
use crate::hardware::cooling::thermal_paste::ThermalPaste;
use crate::hardware::cooling::thermal_sensors::ThermalSensor;
use std::collections::HashMap;

pub struct CoolingSystem {
    fans: Vec<Fan>,
    heat_pipes: Vec<HeatPipe>,
    thermal_paste: HashMap<String, ThermalPaste>,
    thermal_sensors: Vec<ThermalSensor>,
    is_initialized: bool,
    cooling_level: u8, // 0-100%
}

impl CoolingSystem {
    pub fn new() -> Self {
        // Create default cooling system
        let mut fans = Vec::new();
        fans.push(Fan::new("CPU Fan", 0, 3000));
        fans.push(Fan::new("GPU Fan", 0, 2800));
        fans.push(Fan::new("System Fan", 0, 2000));
        
        let mut heat_pipes = Vec::new();
        heat_pipes.push(HeatPipe::new("CPU Heat Pipe"));
        heat_pipes.push(HeatPipe::new("GPU Heat Pipe"));
        
        let mut thermal_paste = HashMap::new();
        thermal_paste.insert("CPU".to_string(), ThermalPaste::new("CPU Thermal Compound"));
        thermal_paste.insert("GPU".to_string(), ThermalPaste::new("GPU Thermal Compound"));
        
        let mut thermal_sensors = Vec::new();
        thermal_sensors.push(ThermalSensor::new("CPU", 35.0));
        thermal_sensors.push(ThermalSensor::new("GPU", 33.0));
        thermal_sensors.push(ThermalSensor::new("System", 30.0));
        
        Self {
            fans,
            heat_pipes,
            thermal_paste,
            thermal_sensors,
            is_initialized: false,
            cooling_level: 0,
        }
    }
    
    pub fn initialize(&mut self) -> bool {
        println!("Initializing cooling system...");
        
        // Start all fans at low speed
        for fan in &mut self.fans {
            fan.set_speed(20);
        }
        
        self.cooling_level = 20;
        self.is_initialized = true;
        true
    }
    
    pub fn shutdown(&mut self) -> bool {
        println!("Shutting down cooling system...");
        
        // Stop all fans
        for fan in &mut self.fans {
            fan.set_speed(0);
        }
        
        self.cooling_level = 0;
        self.is_initialized = false;
        true
    }
    
    pub fn increase_cooling(&mut self, temperature: u8) {
        if !self.is_initialized {
            return;
        }
        
        // Adjust cooling based on temperature
        let new_level = match temperature {
            0..=40 => 20,   // Low temperature, minimal cooling
            41..=60 => 40,  // Moderate temperature, medium cooling
            61..=80 => 70,  // High temperature, high cooling
            81..=90 => 90,  // Very high temperature, maximum cooling
            _ => 100,       // Critical temperature, emergency cooling
        };
        
        if new_level != self.cooling_level {
            self.cooling_level = new_level;
            
            // Update fan speeds
            for fan in &mut self.fans {
                fan.set_speed(self.cooling_level);
            }
            
            println!("Cooling level increased to {}%", self.cooling_level);
        }
    }
    
    pub fn decrease_cooling(&mut self) {
        if !self.is_initialized || self.cooling_level <= 20 {
            return;
        }
        
        // Gradually decrease cooling to save power
        let new_level = (self.cooling_level - 10).max(20);
        
        if new_level != self.cooling_level {
            self.cooling_level = new_level;
            
            // Update fan speeds
            for fan in &mut self.fans {
                fan.set_speed(self.cooling_level);
            }
            
            println!("Cooling level decreased to {}%", self.cooling_level);
        }
    }
    
    pub fn get_cooling_power(&self) -> u8 {
        self.cooling_level
    }
    
    pub fn update_sensor(&mut self, sensor_name: &str, temperature: f32) {
        if !self.is_initialized {
            return;
        }
        
        // Find the sensor and update its temperature
        for sensor in &mut self.thermal_sensors {
            if sensor.get_name() == sensor_name {
                sensor.update_temperature(temperature);
                
                // Check if the temperature is critical
                if sensor.is_critical() {
                    println!("CRITICAL TEMPERATURE DETECTED: {} at {:.1}°C", 
                            sensor_name, temperature);
                    // Trigger emergency cooling
                    self.increase_cooling(100);
                }
                
                break;
            }
        }
    }
    
    pub fn get_sensor_temperature(&self, sensor_name: &str) -> Option<f32> {
        if !self.is_initialized {
            return None;
        }
        
        // Find the sensor and return its temperature
        for sensor in &self.thermal_sensors {
            if sensor.get_name() == sensor_name {
                return Some(sensor.get_temperature());
            }
        }
        
        None
    }
    
    pub fn get_average_temperature(&self) -> f32 {
        if !self.is_initialized || self.thermal_sensors.is_empty() {
            return 0.0;
        }
        
        let sum: f32 = self.thermal_sensors.iter()
            .map(|s| s.get_temperature())
            .sum();
            
        sum / self.thermal_sensors.len() as f32
    }
    
    pub fn replace_thermal_paste(&mut self, component: &str) {
        if !self.is_initialized {
            return;
        }
        
        if let Some(paste) = self.thermal_paste.get_mut(component) {
            paste.apply(0.1, 25.0); // Apply fresh thermal paste
            println!("Thermal paste replaced for {}", component);
        }
    }
    
    pub fn check_thermal_paste_status(&self) -> Vec<(String, bool)> {
        let mut results = Vec::new();
        
        for (component, paste) in &self.thermal_paste {
            results.push((component.clone(), paste.needs_replacement()));
        }
        
        results
    }
    
    pub fn get_fan_speeds(&self) -> Vec<(String, u8)> {
        self.fans.iter()
            .map(|fan| (fan.get_name().to_string(), fan.get_speed()))
            .collect()
    }
    
    pub fn set_fan_speed(&mut self, fan_name: &str, speed: u8) -> bool {
        if !self.is_initialized {
            return false;
        }
        
        // Find the fan and set its speed
        for fan in &mut self.fans {
            if fan.get_name() == fan_name {
                fan.set_speed(speed);
                return true;
            }
        }
        
        false
    }
    
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
