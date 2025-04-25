// Thermal management implementation
use std::collections::HashMap;

pub struct ThermalManagement {
    thermal_zones: HashMap<String, ThermalZone>,
    throttling_thresholds: Vec<ThrottlingThreshold>,
    current_throttling: ThrottlingLevel,
    is_initialized: bool,
}

pub struct ThermalZone {
    name: String,
    current_temperature: f32, // Celsius
    max_temperature: f32, // Celsius
    critical_temperature: f32, // Celsius
    cooling_device: Option<String>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ThrottlingLevel {
    None,
    Light,
    Moderate,
    Heavy,
    Critical,
}

struct ThrottlingThreshold {
    level: ThrottlingLevel,
    temperature: f32,
    cpu_frequency_reduction: f32, // % reduction
    gpu_frequency_reduction: f32, // % reduction
}

impl ThermalManagement {
    pub fn new() -> Self {
        // Set up default thermal zones
        let mut zones = HashMap::new();
        zones.insert("CPU".to_string(), ThermalZone {
            name: "CPU".to_string(),
            current_temperature: 40.0,
            max_temperature: 90.0,
            critical_temperature: 100.0,
            cooling_device: Some("CPU Fan".to_string()),
        });
        
        zones.insert("GPU".to_string(), ThermalZone {
            name: "GPU".to_string(),
            current_temperature: 38.0,
            max_temperature: 85.0,
            critical_temperature: 95.0,
            cooling_device: Some("GPU Fan".to_string()),
        });
        
        zones.insert("BATTERY".to_string(), ThermalZone {
            name: "BATTERY".to_string(),
            current_temperature: 30.0,
            max_temperature: 45.0,
            critical_temperature: 60.0,
            cooling_device: None,
        });
        
        // Set up throttling thresholds
        let thresholds = vec![
            ThrottlingThreshold {
                level: ThrottlingLevel::Light,
                temperature: 75.0,
                cpu_frequency_reduction: 10.0,
                gpu_frequency_reduction: 5.0,
            },
            ThrottlingThreshold {
                level: ThrottlingLevel::Moderate,
                temperature: 85.0,
                cpu_frequency_reduction: 25.0,
                gpu_frequency_reduction: 15.0,
            },
            ThrottlingThreshold {
                level: ThrottlingLevel::Heavy,
                temperature: 90.0,
                cpu_frequency_reduction: 40.0,
                gpu_frequency_reduction: 30.0,
            },
            ThrottlingThreshold {
                level: ThrottlingLevel::Critical,
                temperature: 95.0,
                cpu_frequency_reduction: 60.0,
                gpu_frequency_reduction: 50.0,
            },
        ];
        
        Self {
            thermal_zones: zones,
            throttling_thresholds: thresholds,
            current_throttling: ThrottlingLevel::None,
            is_initialized: false,
        }
    }
    
    pub fn initialize(&mut self) -> bool {
        println!("Thermal management system initialized");
        self.is_initialized = true;
        true
    }
    
    pub fn shutdown(&mut self) -> bool {
        self.is_initialized = false;
        true
    }
    
    pub fn update_temperature(&mut self, zone: &str, temperature: f32) {
        if !self.is_initialized {
            return;
        }
        
        if let Some(thermal_zone) = self.thermal_zones.get_mut(zone) {
            thermal_zone.current_temperature = temperature;
            
            // Check if we need to update throttling
            self.update_throttling();
        }
    }
    
    fn update_throttling(&mut self) {
        // Find the highest temperature among all zones
        let max_temp = self.thermal_zones.values()
            .map(|zone| zone.current_temperature)
            .fold(0.0, f32::max);
        
        // Determine throttling level
        let new_level = self.throttling_thresholds.iter()
            .rev() // Start from highest threshold
            .find(|t| max_temp >= t.temperature)
            .map(|t| t.level)
            .unwrap_or(ThrottlingLevel::None);
        
        // Apply throttling if needed
        if new_level != self.current_throttling {
            self.current_throttling = new_level;
            
            if new_level != ThrottlingLevel::None {
                let threshold = self.throttling_thresholds.iter()
                    .find(|t| t.level == new_level)
                    .unwrap();
                
                println!("Thermal throttling activated: {:?}", new_level);
                println!("CPU frequency reduced by {:.1}%", threshold.cpu_frequency_reduction);
                println!("GPU frequency reduced by {:.1}%", threshold.gpu_frequency_reduction);
            } else {
                println!("Thermal throttling deactivated");
            }
        }
    }
    
    pub fn get_zone_temperature(&self, zone: &str) -> Option<f32> {
        self.thermal_zones.get(zone).map(|z| z.current_temperature)
    }
    
    pub fn get_throttling_level(&self) -> ThrottlingLevel {
        self.current_throttling
    }
    
    pub fn is_throttling(&self) -> bool {
        self.current_throttling != ThrottlingLevel::None
    }
    
    pub fn get_cpu_frequency_reduction(&self) -> f32 {
        if self.current_throttling == ThrottlingLevel::None {
            return 0.0;
        }
        
        self.throttling_thresholds.iter()
            .find(|t| t.level == self.current_throttling)
            .map(|t| t.cpu_frequency_reduction)
            .unwrap_or(0.0)
    }
    
    pub fn get_gpu_frequency_reduction(&self) -> f32 {
        if self.current_throttling == ThrottlingLevel::None {
            return 0.0;
        }
        
        self.throttling_thresholds.iter()
            .find(|t| t.level == self.current_throttling)
            .map(|t| t.gpu_frequency_reduction)
            .unwrap_or(0.0)
    }
    
    pub fn add_thermal_zone(&mut self, name: &str, max_temp: f32, critical_temp: f32) {
        if !self.is_initialized {
            return;
        }
        
        self.thermal_zones.insert(name.to_string(), ThermalZone {
            name: name.to_string(),
            current_temperature: 30.0, // Default starting temperature
            max_temperature: max_temp,
            critical_temperature: critical_temp,
            cooling_device: None,
        });
    }
} 