// Safe shutdown procedures

use crate::common::traits::powerable::Powerable;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum ShutdownStage {
    SaveData,
    CloseApplications,
    StopServices,
    UnloadDrivers,
    PowerOff,
    Complete,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct ShutdownSequence {
    stages: Vec<ShutdownStage>,
    current_stage: usize,
    shutdown_time: Duration,
}

impl ShutdownSequence {
    pub fn new() -> Self {
        Self {
            stages: vec![
                ShutdownStage::SaveData,
                ShutdownStage::CloseApplications,
                ShutdownStage::StopServices,
                ShutdownStage::UnloadDrivers,
                ShutdownStage::PowerOff,
                ShutdownStage::Complete,
            ],
            current_stage: 0,
            shutdown_time: Duration::from_secs(0),
        }
    }
    
    pub fn current_stage(&self) -> Option<&ShutdownStage> {
        self.stages.get(self.current_stage)
    }
    
    pub fn is_complete(&self) -> bool {
        self.current_stage >= self.stages.len() - 1 && 
        matches!(self.current_stage(), Some(ShutdownStage::Complete))
    }
    
    pub fn has_failed(&self) -> bool {
        matches!(self.current_stage(), Some(ShutdownStage::Failed(_)))
    }
    
    pub fn shutdown_time(&self) -> Duration {
        self.shutdown_time
    }
    
    pub fn proceed(&mut self) -> Result<&ShutdownStage, &'static str> {
        if self.is_complete() {
            return Err("Shutdown sequence already complete");
        }
        
        if self.has_failed() {
            return Err("Shutdown sequence has failed");
        }
        
        // Simulate time passing during shutdown stage
        self.shutdown_time += Duration::from_millis(300);
        
        // Move to next stage
        self.current_stage += 1;
        
        // Ensure we don't go out of bounds
        if self.current_stage >= self.stages.len() {
            self.current_stage = self.stages.len() - 1;
        }
        
        Ok(self.current_stage().unwrap())
    }
    
    pub fn fail(&mut self, reason: &str) {
        self.stages[self.current_stage] = ShutdownStage::Failed(reason.to_string());
    }
}

// Safe shutdown function for any device implementing Powerable
pub fn shutdown_device<T: Powerable>(device: &mut T, force: bool) -> Result<(), &'static str> {
    // For non-forced shutdowns, we should check if it's safe to power off
    if !force {
        // Perform checks (example placeholder)
        // In a real system, this would check for unsaved data, running processes, etc.
    }
    
    // Power off the device
    if device.power_off() {
        Ok(())
    } else {
        Err("Failed to power off device")
    }
}

// Emergency shutdown function
pub fn emergency_shutdown<T: Powerable>(device: &mut T) -> Result<(), &'static str> {
    // Emergency shutdowns bypass normal shutdown procedures
    shutdown_device(device, true)
} 