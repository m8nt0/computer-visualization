// Software/firmware update mechanisms
use crate::common::traits::updatable::Updatable;
use crate::common::types::version::Version;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum UpdateStage {
    CheckingForUpdates,
    DownloadingUpdate,
    VerifyingUpdate,
    BackingUpCurrentVersion,
    InstallingUpdate,
    FinalizingUpdate,
    Complete,
    Failed(String),
}

#[derive(Debug, Clone)]
pub struct UpdateProcess {
    stages: Vec<UpdateStage>,
    current_stage: usize,
    start_time: Instant,
    update_size: u64, // in bytes
    download_progress: f32, // 0.0 to 1.0
    requires_restart: bool,
}

impl UpdateProcess {
    pub fn new(update_size: u64, requires_restart: bool) -> Self {
        Self {
            stages: vec![
                UpdateStage::CheckingForUpdates,
                UpdateStage::DownloadingUpdate,
                UpdateStage::VerifyingUpdate,
                UpdateStage::BackingUpCurrentVersion,
                UpdateStage::InstallingUpdate,
                UpdateStage::FinalizingUpdate,
                UpdateStage::Complete,
            ],
            current_stage: 0,
            start_time: Instant::now(),
            update_size,
            download_progress: 0.0,
            requires_restart,
        }
    }
    
    pub fn current_stage(&self) -> Option<&UpdateStage> {
        self.stages.get(self.current_stage)
    }
    
    pub fn is_complete(&self) -> bool {
        self.current_stage >= self.stages.len() - 1 && 
        matches!(self.current_stage(), Some(UpdateStage::Complete))
    }
    
    pub fn has_failed(&self) -> bool {
        matches!(self.current_stage(), Some(UpdateStage::Failed(_)))
    }
    
    pub fn elapsed_time(&self) -> Duration {
        self.start_time.elapsed()
    }
    
    pub fn proceed(&mut self) -> Result<&UpdateStage, &'static str> {
        if self.is_complete() {
            return Err("Update process already complete");
        }
        
        if self.has_failed() {
            return Err("Update process has failed");
        }
        
        // Special case for downloading stage to update progress
        if matches!(self.current_stage(), Some(UpdateStage::DownloadingUpdate)) && self.download_progress < 1.0 {
            self.download_progress += 0.2;
            if self.download_progress >= 1.0 {
                self.download_progress = 1.0;
                self.current_stage += 1;
            }
        } else {
            // Move to next stage
            self.current_stage += 1;
        }
        
        // Ensure we don't go out of bounds
        if self.current_stage >= self.stages.len() {
            self.current_stage = self.stages.len() - 1;
        }
        
        Ok(self.current_stage().unwrap())
    }
    
    pub fn fail(&mut self, reason: &str) {
        self.stages[self.current_stage] = UpdateStage::Failed(reason.to_string());
    }
    
    pub fn requires_restart(&self) -> bool {
        self.requires_restart
    }
    
    pub fn update_size(&self) -> u64 {
        self.update_size
    }
    
    pub fn download_progress(&self) -> f32 {
        self.download_progress
    }
}

// Function to check for updates
pub fn check_for_updates<T: Updatable>(updatable: &T) -> Option<Version> {
    let current_version = updatable.current_version();
    let latest_version = updatable.latest_available_version();
    
    if latest_version > current_version {
        Some(latest_version)
    } else {
        None
    }
}

// Function to perform update
pub fn update_device<T: Updatable>(updatable: &mut T) -> Result<Version, String> {
    let result = updatable.update();
    
    match result {
        Ok(new_version) => {
            println!("Updated {} to version {}", updatable.device_name(), new_version);
            Ok(new_version)
        },
        Err(e) => {
            println!("Failed to update {}: {}", updatable.device_name(), e);
            Err(e)
        }
    }
} 