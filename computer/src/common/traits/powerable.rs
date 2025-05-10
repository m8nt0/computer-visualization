// Power on/off/suspend capabilities
use super::super::types::error::Result;

/// A trait for devices that can be powered on and off
pub trait Powerable {
    /// Power on the device
    fn power_on(&mut self) -> Result<()>;
    
    /// Power off the device
    fn power_off(&mut self) -> Result<()>;
    
    /// Check if the device is powered on
    fn is_powered(&self) -> bool;
    
    /// Reset the device (reboot/restart)
    fn reset(&mut self) -> Result<()> {
        // Default implementation turns off and back on
        self.power_off()?;
        self.power_on()
    }
    
    /// Put the device in sleep mode
    fn sleep(&mut self) -> Result<()> {
        // Default implementation just reports whether it's feasible
        if self.is_powered() {
            Ok(())
        } else {
            Err(super::super::types::error::ComputerError::new(
                super::super::types::error::ErrorKind::OperationFailed,
                "Cannot sleep: device is not powered on"
            ))
        }
    }
    
    /// Wake the device from sleep mode
    fn wake(&mut self) -> Result<()> {
        // Default implementation
        if !self.is_powered() {
            self.power_on()
        } else {
            Ok(())
        }
    }
    
    /// Put the device in hibernate mode
    fn hibernate(&mut self) -> Result<()> {
        // Default implementation
        if self.is_powered() {
            Ok(())
        } else {
            Err(super::super::types::error::ComputerError::new(
                super::super::types::error::ErrorKind::OperationFailed,
                "Cannot hibernate: device is not powered on"
            ))
        }
    }
    
    /// Resume from hibernation
    fn resume(&mut self) -> Result<()> {
        // Default implementation
        if !self.is_powered() {
            self.power_on()
        } else {
            Ok(())
        }
    }
    
    /// Get the current power consumption in watts
    fn power_consumption(&self) -> f32 {
        // Default implementation returns 0.0
        0.0
    }
    
    /// Get the maximum power consumption in watts
    fn max_power_consumption(&self) -> f32 {
        // Default implementation returns 0.0
        0.0
    }
    
    /// Enable power saving mode
    fn enable_power_saving(&mut self) -> Result<()> {
        // Default implementation
        Ok(())
    }
    
    /// Disable power saving mode
    fn disable_power_saving(&mut self) -> Result<()> {
        // Default implementation
        Ok(())
    }
}