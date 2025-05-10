// Resource availability validation
use super::super::types::status::{Metric, OperationalStatus};

#[derive(Debug, Clone)]
pub struct ResourceValidator {
    memory_warning_threshold: f64,  // Percentage (0.0-1.0)
    memory_critical_threshold: f64, // Percentage (0.0-1.0)
    cpu_warning_threshold: f64,     // Percentage (0.0-1.0)
    cpu_critical_threshold: f64,    // Percentage (0.0-1.0)
    storage_warning_threshold: f64, // Percentage (0.0-1.0)
    storage_critical_threshold: f64, // Percentage (0.0-1.0)
}

impl Default for ResourceValidator {
    fn default() -> Self {
        Self {
            memory_warning_threshold: 0.8,
            memory_critical_threshold: 0.95,
            cpu_warning_threshold: 0.8,
            cpu_critical_threshold: 0.95,
            storage_warning_threshold: 0.85,
            storage_critical_threshold: 0.95,
        }
    }
}

impl ResourceValidator {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_memory_thresholds(mut self, warning: f64, critical: f64) -> Self {
        self.memory_warning_threshold = warning;
        self.memory_critical_threshold = critical;
        self
    }
    
    pub fn with_cpu_thresholds(mut self, warning: f64, critical: f64) -> Self {
        self.cpu_warning_threshold = warning;
        self.cpu_critical_threshold = critical;
        self
    }
    
    pub fn with_storage_thresholds(mut self, warning: f64, critical: f64) -> Self {
        self.storage_warning_threshold = warning;
        self.storage_critical_threshold = critical;
        self
    }
    
    pub fn validate_memory_usage(&self, usage_percentage: f64) -> OperationalStatus {
        if usage_percentage >= self.memory_critical_threshold {
            OperationalStatus::Critical
        } else if usage_percentage >= self.memory_warning_threshold {
            OperationalStatus::Warning
        } else {
            OperationalStatus::Normal
        }
    }
    
    pub fn validate_cpu_usage(&self, usage_percentage: f64) -> OperationalStatus {
        if usage_percentage >= self.cpu_critical_threshold {
            OperationalStatus::Critical
        } else if usage_percentage >= self.cpu_warning_threshold {
            OperationalStatus::Warning
        } else {
            OperationalStatus::Normal
        }
    }
    
    pub fn validate_storage_usage(&self, usage_percentage: f64) -> OperationalStatus {
        if usage_percentage >= self.storage_critical_threshold {
            OperationalStatus::Critical
        } else if usage_percentage >= self.storage_warning_threshold {
            OperationalStatus::Warning
        } else {
            OperationalStatus::Normal
        }
    }
    
    pub fn create_memory_metric(&self, usage_percentage: f64) -> Metric {
        Metric::new("Memory Usage", usage_percentage * 100.0)
            .with_unit("%")
            .with_thresholds(
                Some(self.memory_warning_threshold * 100.0),
                Some(self.memory_critical_threshold * 100.0)
            )
    }
    
    pub fn create_cpu_metric(&self, usage_percentage: f64) -> Metric {
        Metric::new("CPU Usage", usage_percentage * 100.0)
            .with_unit("%")
            .with_thresholds(
                Some(self.cpu_warning_threshold * 100.0),
                Some(self.cpu_critical_threshold * 100.0)
            )
    }
    
    pub fn create_storage_metric(&self, usage_percentage: f64) -> Metric {
        Metric::new("Storage Usage", usage_percentage * 100.0)
            .with_unit("%")
            .with_thresholds(
                Some(self.storage_warning_threshold * 100.0),
                Some(self.storage_critical_threshold * 100.0)
            )
    }
}

pub fn calculate_memory_usage(used_memory: u64, total_memory: u64) -> f64 {
    if total_memory == 0 {
        return 1.0; // Avoid division by zero
    }
    used_memory as f64 / total_memory as f64
}

pub fn calculate_storage_usage(used_space: u64, total_space: u64) -> f64 {
    if total_space == 0 {
        return 1.0; // Avoid division by zero
    }
    used_space as f64 / total_space as f64
}


// =================================================================


// pub struct ResourceThresholds {
//     pub warning_percent: f32,
//     pub critical_percent: f32,
// }

// impl ResourceThresholds {
//     pub fn new(warning_percent: f32, critical_percent: f32) -> Self {
//         Self {
//             warning_percent,
//             critical_percent,
//         }
//     }
    
//     pub fn get_status(&self, used_percent: f32) -> ResourceStatus {
//         if used_percent >= self.critical_percent {
//             ResourceStatus::Critical
//         } else if used_percent >= self.warning_percent {
//             ResourceStatus::Warning
//         } else {
//             ResourceStatus::Normal
//         }
//     }
// }

// pub enum ResourceStatus {
//     Normal,
//     Warning,
//     Critical,
// }

// pub fn calculate_usage_percent(used: u64, total: u64) -> f32 {
//     if total == 0 {
//         return 100.0;
//     }
    
//     (used as f32 / total as f32) * 100.0
// }

// pub fn calculate_free_percent(used: u64, total: u64) -> f32 {
//     100.0 - calculate_usage_percent(used, total)
// }

// pub fn is_resource_critical(used: u64, total: u64, critical_threshold_percent: f32) -> bool {
//     calculate_usage_percent(used, total) >= critical_threshold_percent
// }

// pub fn estimate_time_until_full(current: u64, total: u64, rate_per_second: f64) -> Option<f64> {
//     if rate_per_second <= 0.0 || current >= total {
//         return None;
//     }
    
//     Some((total - current) as f64 / rate_per_second)
// }

// pub fn estimate_time_until_empty(current: u64, rate_per_second: f64) -> Option<f64> {
//     if rate_per_second <= 0.0 {
//         return None;
//     }
    
//     Some(current as f64 / rate_per_second)
// }