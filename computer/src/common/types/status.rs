// Operational status indicators
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalStatus {
    Normal,
    Warning,
    Critical,
    Error,
    Offline,
    Unknown,
}

impl fmt::Display for OperationalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperationalStatus::Normal => write!(f, "Normal"),
            OperationalStatus::Warning => write!(f, "Warning"),
            OperationalStatus::Critical => write!(f, "Critical"),
            OperationalStatus::Error => write!(f, "Error"),
            OperationalStatus::Offline => write!(f, "Offline"),
            OperationalStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemStatus {
    overall: OperationalStatus,
    components: Vec<ComponentStatus>,
    message: Option<String>,
    timestamp: u64, // Unix timestamp
}

#[derive(Debug, Clone)]
pub struct ComponentStatus {
    name: String,
    status: OperationalStatus,
    message: Option<String>,
    metrics: Vec<Metric>,
}

#[derive(Debug, Clone)]
pub struct Metric {
    name: String,
    value: f64,
    unit: Option<String>,
    threshold_warning: Option<f64>,
    threshold_critical: Option<f64>,
}

impl SystemStatus {
    pub fn new(overall: OperationalStatus, timestamp: u64) -> Self {
        Self {
            overall,
            components: Vec::new(),
            message: None,
            timestamp,
        }
    }
    
    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }
    
    pub fn add_component(&mut self, component: ComponentStatus) {
        self.components.push(component);
        self.update_overall_status();
    }
    
    pub fn overall(&self) -> OperationalStatus {
        self.overall
    }
    
    pub fn components(&self) -> &[ComponentStatus] {
        &self.components
    }
    
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
    
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    
    fn update_overall_status(&mut self) {
        let mut worst_status = OperationalStatus::Normal;
        
        for component in &self.components {
            let status = component.status();
            
            // Determine the worst status among components
            worst_status = match (worst_status, status) {
                (_, OperationalStatus::Error) => OperationalStatus::Error,
                (OperationalStatus::Error, _) => OperationalStatus::Error,
                
                (_, OperationalStatus::Critical) => OperationalStatus::Critical,
                (OperationalStatus::Critical, _) => OperationalStatus::Critical,
                
                (_, OperationalStatus::Warning) => OperationalStatus::Warning,
                (OperationalStatus::Warning, _) => OperationalStatus::Warning,
                
                (_, OperationalStatus::Offline) => OperationalStatus::Warning,
                (OperationalStatus::Offline, _) => OperationalStatus::Warning,
                
                (_, OperationalStatus::Unknown) => OperationalStatus::Warning,
                (OperationalStatus::Unknown, _) => OperationalStatus::Warning,
                
                (_, OperationalStatus::Normal) => worst_status,
            };
        }
        
        self.overall = worst_status;
    }
}

impl ComponentStatus {
    pub fn new(name: &str, status: OperationalStatus) -> Self {
        Self {
            name: name.to_string(),
            status,
            message: None,
            metrics: Vec::new(),
        }
    }
    
    pub fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }
    
    pub fn add_metric(&mut self, metric: Metric) {
        self.metrics.push(metric);
        self.update_status_from_metrics();
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn status(&self) -> OperationalStatus {
        self.status
    }
    
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
    
    pub fn metrics(&self) -> &[Metric] {
        &self.metrics
    }
    
    fn update_status_from_metrics(&mut self) {
        // Don't override error or offline status based on metrics
        if matches!(self.status, OperationalStatus::Error | OperationalStatus::Offline) {
            return;
        }
        
        let mut worst_status = OperationalStatus::Normal;
        
        for metric in &self.metrics {
            if let (Some(critical), Some(value)) = (metric.threshold_critical, Some(metric.value)) {
                if value >= critical {
                    worst_status = OperationalStatus::Critical;
                    break; // Found a critical metric, no need to check others
                }
            }
            
            if let (Some(warning), Some(value)) = (metric.threshold_warning, Some(metric.value)) {
                if value >= warning && worst_status == OperationalStatus::Normal {
                    worst_status = OperationalStatus::Warning;
                    // Continue checking other metrics for potential critical values
                }
            }
        }
        
        self.status = worst_status;
    }
}

impl Metric {
    pub fn new(name: &str, value: f64) -> Self {
        Self {
            name: name.to_string(),
            value,
            unit: None,
            threshold_warning: None,
            threshold_critical: None,
        }
    }
    
    pub fn with_unit(mut self, unit: &str) -> Self {
        self.unit = Some(unit.to_string());
        self
    }
    
    pub fn with_thresholds(mut self, warning: Option<f64>, critical: Option<f64>) -> Self {
        self.threshold_warning = warning;
        self.threshold_critical = critical;
        self
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn value(&self) -> f64 {
        self.value
    }
    
    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }
    
    pub fn threshold_warning(&self) -> Option<f64> {
        self.threshold_warning
    }
    
    pub fn threshold_critical(&self) -> Option<f64> {
        self.threshold_critical
    }
    
    pub fn status(&self) -> OperationalStatus {
        if let Some(critical) = self.threshold_critical {
            if self.value >= critical {
                return OperationalStatus::Critical;
            }
        }
        
        if let Some(warning) = self.threshold_warning {
            if self.value >= warning {
                return OperationalStatus::Warning;
            }
        }
        
        OperationalStatus::Normal
    }
    
    pub fn format_value(&self) -> String {
        if let Some(unit) = &self.unit {
            format!("{:.2} {}", self.value, unit)
        } else {
            format!("{:.2}", self.value)
        }
    }
}