// Mock implementations of common components for testing
use std::sync::Mutex;
use std::time::Duration;
use std::collections::HashMap;
use super::super::types::error::{ComputerError, ErrorKind, Result};
use super::super::types::status::{OperationalStatus, ComponentStatus};
use super::super::types::device_id::{DeviceId, DeviceType};
use super::super::traits::bootable::Bootable;
use super::super::traits::powerable::Powerable;

/// A mock hardware component that tracks method calls for testing
pub struct MockComponent {
    name: String,
    call_log: Mutex<Vec<String>>,
    status: Mutex<OperationalStatus>,
    properties: Mutex<HashMap<String, String>>,
    failure_mode: Mutex<Option<ErrorKind>>,
}

impl MockComponent {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            call_log: Mutex::new(Vec::new()),
            status: Mutex::new(OperationalStatus::Normal),
            properties: Mutex::new(HashMap::new()),
            failure_mode: Mutex::new(None),
        }
    }
    
    pub fn with_status(self, status: OperationalStatus) -> Self {
        if let Ok(mut s) = self.status.lock() {
            *s = status;
        }
        self
    }
    
    pub fn with_property(self, key: &str, value: &str) -> Self {
        if let Ok(mut props) = self.properties.lock() {
            props.insert(key.to_string(), value.to_string());
        }
        self
    }
    
    pub fn set_failure_mode(&self, failure: Option<ErrorKind>) {
        if let Ok(mut mode) = self.failure_mode.lock() {
            *mode = failure;
        }
    }
    
    pub fn log_call(&self, method: &str) {
        if let Ok(mut log) = self.call_log.lock() {
            log.push(format!("{}::{}", self.name, method));
        }
    }
    
    pub fn get_call_log(&self) -> Vec<String> {
        if let Ok(log) = self.call_log.lock() {
            log.clone()
        } else {
            Vec::new()
        }
    }
    
    pub fn has_called(&self, method: &str) -> bool {
        if let Ok(log) = self.call_log.lock() {
            log.iter().any(|call| call.ends_with(method))
        } else {
            false
        }
    }
    
    pub fn call_count(&self, method: &str) -> usize {
        if let Ok(log) = self.call_log.lock() {
            log.iter().filter(|call| call.ends_with(method)).count()
        } else {
            0
        }
    }
    
    pub fn get_status(&self) -> OperationalStatus {
        if let Ok(status) = self.status.lock() {
            *status
        } else {
            OperationalStatus::Unknown
        }
    }
    
    pub fn set_status(&self, status: OperationalStatus) {
        if let Ok(mut s) = self.status.lock() {
            *s = status;
        }
    }
    
    pub fn get_property(&self, key: &str) -> Option<String> {
        if let Ok(props) = self.properties.lock() {
            props.get(key).cloned()
        } else {
            None
        }
    }
    
    pub fn check_failure(&self) -> Result<()> {
        if let Ok(mode) = self.failure_mode.lock() {
            if let Some(kind) = *mode {
                return Err(ComputerError::new(kind, &format!("Mock failure in {}", self.name)));
            }
        }
        Ok(())
    }
}

/// A mock device that implements common traits for testing
pub struct MockDevice {
    component: MockComponent,
    device_id: DeviceId,
    boot_time_ms: u64,
    shutdown_time_ms: u64,
}

impl MockDevice {
    pub fn new(name: &str, device_type: DeviceType) -> Self {
        Self {
            component: MockComponent::new(name),
            device_id: DeviceId::new(&format!("MOCK-{}", name), device_type),
            boot_time_ms: 500,
            shutdown_time_ms: 300,
        }
    }
    
    pub fn with_boot_time(mut self, ms: u64) -> Self {
        self.boot_time_ms = ms;
        self
    }
    
    pub fn with_shutdown_time(mut self, ms: u64) -> Self {
        self.shutdown_time_ms = ms;
        self
    }
    
    pub fn component(&self) -> &MockComponent {
        &self.component
    }
    
    pub fn device_id(&self) -> &DeviceId {
        &self.device_id
    }
}

impl Bootable for MockDevice {
    fn boot(&mut self) -> Result<Duration> {
        self.component.log_call("boot");
        self.component.check_failure()?;
        
        let duration = Duration::from_millis(self.boot_time_ms);
        Ok(duration)
    }
    
    fn is_booted(&self) -> bool {
        self.component.log_call("is_booted");
        self.component.has_called("boot") && 
            self.component.get_status() != OperationalStatus::Offline
    }
    
    fn get_boot_status(&self) -> ComponentStatus {
        self.component.log_call("get_boot_status");
        
        ComponentStatus::new("MockBoot", self.component.get_status())
            .with_message("Mock boot status")
    }
    
    fn shutdown(&mut self) -> Result<()> {
        self.component.log_call("shutdown");
        self.component.check_failure()?;
        
        self.component.set_status(OperationalStatus::Offline);
        Ok(())
    }
}

impl Powerable for MockDevice {
    fn power_on(&mut self) -> Result<()> {
        self.component.log_call("power_on");
        self.component.check_failure()?;
        
        self.component.set_status(OperationalStatus::Normal);
        Ok(())
    }
    
    fn power_off(&mut self) -> Result<()> {
        self.component.log_call("power_off");
        self.component.check_failure()?;
        
        self.component.set_status(OperationalStatus::Offline);
        std::thread::sleep(Duration::from_millis(self.shutdown_time_ms));
        Ok(())
    }
    
    fn is_powered(&self) -> bool {
        self.component.log_call("is_powered");
        self.component.get_status() != OperationalStatus::Offline
    }
    
    fn reset(&mut self) -> Result<()> {
        self.component.log_call("reset");
        self.component.check_failure()?;
        
        self.component.set_status(OperationalStatus::Offline);
        std::thread::sleep(Duration::from_millis(100));
        self.component.set_status(OperationalStatus::Normal);
        Ok(())
    }
}