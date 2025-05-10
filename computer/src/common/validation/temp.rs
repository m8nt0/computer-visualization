// Validate temperature thresholds

// Temperature threshold validation
use super::super::types::status::{Metric, OperationalStatus};

#[derive(Debug, Clone)]
pub struct TemperatureValidator {
    max_temp_celsius: f64,
    warning_threshold_celsius: f64,
    critical_threshold_celsius: f64,
}

impl Default for TemperatureValidator {
    fn default() -> Self {
        Self {
            max_temp_celsius: 100.0,
            warning_threshold_celsius: 70.0,
            critical_threshold_celsius: 85.0,
        }
    }
}

impl TemperatureValidator {
    pub fn new(max_temp_celsius: f64, warning_celsius: f64, critical_celsius: f64) -> Self {
        Self {
            max_temp_celsius,
            warning_threshold_celsius: warning_celsius,
            critical_threshold_celsius: critical_celsius,
        }
    }
    
    pub fn max_temp_celsius(&self) -> f64 {
        self.max_temp_celsius
    }
    
    pub fn warning_threshold_celsius(&self) -> f64 {
        self.warning_threshold_celsius
    }
    
    pub fn critical_threshold_celsius(&self) -> f64 {
        self.critical_threshold_celsius
    }
    
    pub fn validate_temperature(&self, current_temp_celsius: f64) -> OperationalStatus {
        if current_temp_celsius >= self.critical_threshold_celsius {
            OperationalStatus::Critical
        } else if current_temp_celsius >= self.warning_threshold_celsius {
            OperationalStatus::Warning
        } else {
            OperationalStatus::Normal
        }
    }
    
    pub fn create_temperature_metric(&self, current_temp_celsius: f64) -> Metric {
        Metric::new("Temperature", current_temp_celsius)
            .with_unit("°C")
            .with_thresholds(
                Some(self.warning_threshold_celsius),
                Some(self.critical_threshold_celsius)
            )
    }
    
    pub fn is_safe_to_operate(&self, current_temp_celsius: f64) -> bool {
        current_temp_celsius < self.critical_threshold_celsius
    }
    
    pub fn celsius_to_fahrenheit(&self, celsius: f64) -> f64 {
        celsius * 9.0 / 5.0 + 32.0
    }
    
    pub fn fahrenheit_to_celsius(&self, fahrenheit: f64) -> f64 {
        (fahrenheit - 32.0) * 5.0 / 9.0
    }
}

// ===============================================================

// pub struct TemperatureThresholds {
//     pub min_operating_c: f32,
//     pub max_operating_c: f32,
//     pub warning_c: f32,
//     pub critical_c: f32,
// }

// impl TemperatureThresholds {
//     pub fn new(min_operating_c: f32, max_operating_c: f32, warning_c: f32, critical_c: f32) -> Self {
//         Self {
//             min_operating_c,
//             max_operating_c,
//             warning_c,
//             critical_c,
//         }
//     }
    
//     pub fn is_within_operating_range(&self, temp_c: f32) -> bool {
//         temp_c >= self.min_operating_c && temp_c <= self.max_operating_c
//     }
    
//     pub fn is_warning(&self, temp_c: f32) -> bool {
//         temp_c >= self.warning_c && temp_c < self.critical_c
//     }
    
//     pub fn is_critical(&self, temp_c: f32) -> bool {
//         temp_c >= self.critical_c
//     }
    
//     pub fn get_status(&self, temp_c: f32) -> TemperatureStatus {
//         if temp_c < self.min_operating_c {
//             TemperatureStatus::TooLow
//         } else if temp_c > self.critical_c {
//             TemperatureStatus::Critical
//         } else if temp_c > self.warning_c {
//             TemperatureStatus::Warning
//         } else if temp_c <= self.max_operating_c {
//             TemperatureStatus::Normal
//         } else {
//             TemperatureStatus::TooHigh
//         }
//     }
// }

// pub enum TemperatureStatus {
//     TooLow,
//     Normal,
//     Warning,
//     TooHigh,
//     Critical,
// }

// pub fn celsius_to_fahrenheit(celsius: f32) -> f32 {
//     (celsius * 9.0 / 5.0) + 32.0
// }

// pub fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
//     (fahrenheit - 32.0) * 5.0 / 9.0
// }

// pub fn celsius_to_kelvin(celsius: f32) -> f32 {
//     celsius + 273.15
// }

// pub fn kelvin_to_celsius(kelvin: f32) -> f32 {
//     kelvin - 273.15
// }