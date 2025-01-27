use super::rank::Rank;
use super::power::PowerController;

pub struct ThermalController {
    // Temperature thresholds (°C)
    warning_threshold: f32,
    critical_threshold: f32,
    shutdown_threshold: f32,
    
    // Current state
    current_temp: f32,
    ambient_temp: f32,
    thermal_resistance: f32,  // °C/W
    
    // Thermal throttling
    throttling_active: bool,
    throttle_percentage: f32,
    
    // Statistics
    stats: ThermalStats,
}

struct ThermalStats {
    max_temperature: f32,
    time_above_warning: u64,
    time_above_critical: u64,
    throttle_events: u64,
    emergency_shutdowns: u64,
}

impl ThermalController {
    pub fn new() -> Self {
        Self {
            warning_threshold: 85.0,
            critical_threshold: 95.0,
            shutdown_threshold: 105.0,
            
            current_temp: 45.0,    // Start at reasonable temperature
            ambient_temp: 25.0,    // Room temperature
            thermal_resistance: 0.5, // °C/W
            
            throttling_active: false,
            throttle_percentage: 0.0,
            
            stats: ThermalStats::default(),
        }
    }

    pub fn update(&mut self, ranks: &[Rank], power: &PowerController) {
        // Calculate new temperature based on power consumption and thermal resistance
        let power_temp = power.get_temperature_impact();
        let cooling_factor = (self.current_temp - self.ambient_temp) * 0.001;
        
        self.current_temp = self.current_temp + power_temp - cooling_factor;
        
        // Update statistics
        self.stats.max_temperature = self.stats.max_temperature.max(self.current_temp);
        
        // Check thresholds and take action
        if self.current_temp >= self.shutdown_threshold {
            self.emergency_shutdown();
        } else if self.current_temp >= self.critical_threshold {
            self.stats.time_above_critical += 1;
            self.apply_thermal_throttling(0.5); // 50% throttling
        } else if self.current_temp >= self.warning_threshold {
            self.stats.time_above_warning += 1;
            self.apply_thermal_throttling(0.8); // 20% throttling
        } else if self.throttling_active {
            self.remove_thermal_throttling();
        }
    }

    fn apply_thermal_throttling(&mut self, performance_factor: f32) {
        if !self.throttling_active {
            self.stats.throttle_events += 1;
        }
        self.throttling_active = true;
        self.throttle_percentage = 1.0 - performance_factor;
    }

    fn remove_thermal_throttling(&mut self) {
        self.throttling_active = false;
        self.throttle_percentage = 0.0;
    }

    fn emergency_shutdown(&mut self) {
        self.stats.emergency_shutdowns += 1;
        // In a real system, this would trigger an immediate shutdown
        panic!("DRAM temperature critical: Emergency shutdown triggered!");
    }

    pub fn get_temperature(&self) -> f32 {
        self.current_temp
    }

    pub fn is_throttling(&self) -> bool {
        self.throttling_active
    }

    pub fn get_throttle_percentage(&self) -> f32 {
        self.throttle_percentage
    }
}
