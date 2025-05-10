// Trait for boot-capable devices
use std::time::Duration;
use super::super::types::error::Result;
use super::super::types::status::ComponentStatus;

/// A trait for devices that can be booted
pub trait Bootable {
    /// Attempt to boot the device
    fn boot(&mut self) -> Result<Duration>;
    
    /// Check if the device is booted
    fn is_booted(&self) -> bool;
    
    /// Get the boot status
    fn get_boot_status(&self) -> ComponentStatus;
    
    /// Restart the device
    fn restart(&mut self) -> Result<Duration> {
        // Default implementation shuts down and boots again
        if self.is_booted() {
            // Shutdown first
            self.shutdown()?;
        }
        // Then boot
        self.boot()
    }
    
    /// Shutdown the device
    fn shutdown(&mut self) -> Result<()> {
        // Default implementation - subclasses should override
        Ok(())
    }
    
    /// Get the boot options
    fn boot_options(&self) -> Vec<String> {
        // Default implementation returns an empty vector
        Vec::new()
    }
    
    /// Set a boot option
    fn set_boot_option(&mut self, _option: &str, _value: &str) -> Result<()> {
        // Default implementation does nothing
        Err(super::super::types::error::ComputerError::new(
            super::super::types::error::ErrorKind::NotSupported,
            "Boot options not supported"
        ))
    }
    
    /// Enter recovery mode
    fn enter_recovery_mode(&mut self) -> Result<()> {
        // Default implementation
        Err(super::super::types::error::ComputerError::new(
            super::super::types::error::ErrorKind::NotSupported,
            "Recovery mode not supported"
        ))
    }
}