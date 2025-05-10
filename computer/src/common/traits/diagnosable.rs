// Self-diagnostic capabilities

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    Basic,
    Standard,
    Extended,
    Comprehensive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticStatus {
    Passed,
    Warning(String),
    Failed(String),
    NotApplicable,
}

#[derive(Debug, Clone)]
pub struct DiagnosticResult {
    component: String,
    status: DiagnosticStatus,
    details: Option<String>,
    timestamp: u64, // Unix timestamp
}

impl DiagnosticResult {
    pub fn new(component: &str, status: DiagnosticStatus, details: Option<String>, timestamp: u64) -> Self {
        Self {
            component: component.to_string(),
            status,
            details,
            timestamp,
        }
    }
    
    pub fn component(&self) -> &str {
        &self.component
    }
    
    pub fn status(&self) -> &DiagnosticStatus {
        &self.status
    }
    
    pub fn details(&self) -> Option<&String> {
        self.details.as_ref()
    }
    
    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }
    
    pub fn is_passed(&self) -> bool {
        matches!(self.status, DiagnosticStatus::Passed)
    }
    
    pub fn is_warning(&self) -> bool {
        matches!(self.status, DiagnosticStatus::Warning(_))
    }
    
    pub fn is_failed(&self) -> bool {
        matches!(self.status, DiagnosticStatus::Failed(_))
    }
}

/// A trait for devices that can perform self-diagnostics
pub trait Diagnosable {
    /// Run diagnostics at the specified level
    fn run_diagnostics(&self, level: DiagnosticLevel) -> Vec<DiagnosticResult>;
    
    /// Get the diagnostic history
    fn diagnostic_history(&self) -> Vec<DiagnosticResult> {
        // Default implementation returns empty vector
        Vec::new()
    }
    
    /// Run diagnostics on a specific component
    fn diagnose_component(&self, component: &str, level: DiagnosticLevel) -> Option<DiagnosticResult> {
        // Default implementation runs all diagnostics and finds the component
        self.run_diagnostics(level).into_iter()
            .find(|result| result.component() == component)
    }
    
    /// Clear diagnostic history
    fn clear_diagnostic_history(&mut self) -> Result<(), String> {
        // Default implementation
        Err("Clearing diagnostic history not supported".to_string())
    }
    
    /// Get available diagnostic components
    fn get_diagnostic_components(&self) -> Vec<String> {
        // Default implementation returns empty vector
        Vec::new()
    }
    
    /// Check if the device needs service
    fn needs_service(&self) -> bool {
        // Default implementation checks if any diagnostic has failed
        self.diagnostic_history().iter().any(|result| result.is_failed())
    }
}
