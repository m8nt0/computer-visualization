// Boot sequence handling

use crate::common::traits::bootable::Bootable;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum BootStage {
    PowerOn,
    Initialization,
    HardwareChecks,
    LoadingBootloader,
    LoadingOS,
    StartingServices,
    Complete,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct BootSequence {
    stages: Vec<BootStage>,
    current_stage: usize,
    boot_time: Duration,
    auto_proceed: bool,
}

impl BootSequence {
    pub fn new(auto_proceed: bool) -> Self {
        Self {
            stages: vec![
                BootStage::PowerOn,
                BootStage::Initialization,
                BootStage::HardwareChecks,
                BootStage::LoadingBootloader,
                BootStage::LoadingOS,
                BootStage::StartingServices,
                BootStage::Complete,
            ],
            current_stage: 0,
            boot_time: Duration::from_secs(0),
            auto_proceed,
        }
    }
    
    pub fn current_stage(&self) -> Option<&BootStage> {
        self.stages.get(self.current_stage)
    }
    
    pub fn is_complete(&self) -> bool {
        self.current_stage >= self.stages.len() - 1 && 
        matches!(self.current_stage(), Some(BootStage::Complete))
    }
    
    pub fn has_failed(&self) -> bool {
        matches!(self.current_stage(), Some(BootStage::Failed(_)))
    }
    
    pub fn boot_time(&self) -> Duration {
        self.boot_time
    }
    
    pub fn proceed(&mut self) -> Result<&BootStage, &'static str> {
        if self.is_complete() {
            return Err("Boot sequence already complete");
        }
        
        if self.has_failed() {
            return Err("Boot sequence has failed");
        }
        
        // Simulate time passing during boot stage
        self.boot_time += Duration::from_millis(500);
        
        // Move to next stage
        self.current_stage += 1;
        
        // Ensure we don't go out of bounds
        if self.current_stage >= self.stages.len() {
            self.current_stage = self.stages.len() - 1;
        }
        
        Ok(self.current_stage().unwrap())
    }
    
    pub fn fail(&mut self, reason: &str) {
        self.stages[self.current_stage] = BootStage::Failed(reason.to_string());
    }
    
    pub fn reset(&mut self) {
        self.current_stage = 0;
        self.boot_time = Duration::from_secs(0);
        
        // Reset any failed stages
        for stage in &mut self.stages {
            if matches!(stage, BootStage::Failed(_)) {
                *stage = BootStage::PowerOn;
            }
        }
    }
}

pub fn boot_device<T: Bootable>(device: &mut T) -> Result<(), String> {
    let boot_result = device.boot();
    
    match boot_result {
        Ok(()) => {
            println!("Device {} booted successfully", device.device_name());
            Ok(())
        },
        Err(e) => {
            println!("Device {} failed to boot: {}", device.device_name(), e);
            Err(e)
        }
    }
} 