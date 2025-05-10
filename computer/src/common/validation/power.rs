// Validate safe power levels
use super::super::types::status::{Metric, OperationalStatus};

#[derive(Debug, Clone)]
pub struct PowerValidator {
    max_power_draw_watts: f64,
    warning_threshold_pct: f64,
    critical_threshold_pct: f64,
}

impl Default for PowerValidator {
    fn default() -> Self {
        Self {
            max_power_draw_watts: 100.0,
            warning_threshold_pct: 0.8,
            critical_threshold_pct: 0.95,
        }
    }
}

impl PowerValidator {
    pub fn new(max_power_draw_watts: f64) -> Self {
        Self {
            max_power_draw_watts,
            ..Default::default()
        }
    }
    
    pub fn with_thresholds(mut self, warning_pct: f64, critical_pct: f64) -> Self {
        self.warning_threshold_pct = warning_pct;
        self.critical_threshold_pct = critical_pct;
        self
    }
    
    pub fn max_power_draw_watts(&self) -> f64 {
        self.max_power_draw_watts
    }
    
    pub fn warning_threshold_watts(&self) -> f64 {
        self.max_power_draw_watts * self.warning_threshold_pct
    }
    
    pub fn critical_threshold_watts(&self) -> f64 {
        self.max_power_draw_watts * self.critical_threshold_pct
    }
    
    pub fn validate_power_draw(&self, current_draw_watts: f64) -> OperationalStatus {
        if current_draw_watts >= self.critical_threshold_watts() {
            OperationalStatus::Critical
        } else if current_draw_watts >= self.warning_threshold_watts() {
            OperationalStatus::Warning
        } else {
            OperationalStatus::Normal
        }
    }
    
    pub fn create_power_metric(&self, current_draw_watts: f64) -> Metric {
        Metric::new("Power Draw", current_draw_watts)
            .with_unit("W")
            .with_thresholds(
                Some(self.warning_threshold_watts()),
                Some(self.critical_threshold_watts())
            )
    }
    
    pub fn is_safe_to_operate(&self, current_draw_watts: f64) -> bool {
        current_draw_watts < self.critical_threshold_watts()
    }
}


// =================================================================

// pub fn validate_voltage(voltage: f32, nominal_voltage: f32, tolerance_percent: f32) -> bool {
//     let lower_bound = nominal_voltage * (1.0 - tolerance_percent / 100.0);
//     let upper_bound = nominal_voltage * (1.0 + tolerance_percent / 100.0);
    
//     voltage >= lower_bound && voltage <= upper_bound
// }

// pub fn validate_current(current: f32, max_current: f32) -> bool {
//     current >= 0.0 && current <= max_current
// }

// pub fn validate_power_consumption(watts: f32, max_watts: f32) -> bool {
//     watts >= 0.0 && watts <= max_watts
// }

// pub fn estimate_battery_time(capacity_wh: f32, current_draw_w: f32) -> Option<f32> {
//     if current_draw_w <= 0.0 {
//         return None;
//     }
    
//     Some(capacity_wh / current_draw_w) // Hours
// }

// pub fn calculate_efficiency(power_in: f32, power_out: f32) -> f32 {
//     if power_in <= 0.0 {
//         return 0.0;
//     }
    
//     (power_out / power_in) * 100.0 // Percentage
// }

// pub fn is_battery_critical(percentage: f32, critical_threshold: f32) -> bool {
//     percentage <= critical_threshold
// }

// pub struct PowerRange {
//     pub min_watts: f32,
//     pub max_watts: f32,
// }

// impl PowerRange {
//     pub fn new(min_watts: f32, max_watts: f32) -> Self {
//         Self { min_watts, max_watts }
//     }
    
//     pub fn is_within_range(&self, watts: f32) -> bool {
//         watts >= self.min_watts && watts <= self.max_watts
//     }
// }