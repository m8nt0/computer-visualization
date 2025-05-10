// Firmware/sofftware update interface

// Updatable trait (firmware updates) - common/traits/updatable.rs - Applies to tablets, smart fridges, routers, etc.

use super::super::types::version::Version;

/// A trait for devices that can be updated
pub trait Updatable {
    /// Get the device name
    fn device_name(&self) -> &str;
    
    /// Get the current version
    fn current_version(&self) -> Version;
    
    /// Get the latest available version
    fn latest_available_version(&self) -> Version;
    
    /// Check if an update is available
    fn update_available(&self) -> bool {
        self.latest_available_version() > self.current_version()
    }
    
    /// Update the device to the latest version
    fn update(&mut self) -> Result<Version, String>;
    
    /// Rollback to the previous version
    fn rollback(&mut self) -> Result<Version, String> {
        // Default implementation
        Err("Rollback not supported".to_string())
    }
    
    /// Get update release notes
    fn release_notes(&self) -> Option<String> {
        // Default implementation
        None
    }
    
    /// Get update size in bytes
    fn update_size(&self) -> Option<u64> {
        // Default implementation
        None
    }
    
    /// Check if update requires restart
    fn requires_restart(&self) -> bool {
        // Default implementation
        true
    }
    
    /// Get update history
    fn update_history(&self) -> Vec<(Version, String)> {
        // Default implementation returns empty vector
        Vec::new()
    }
}
