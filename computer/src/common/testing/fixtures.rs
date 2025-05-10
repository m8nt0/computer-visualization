// Test data and fixtures

// Test fixtures for common components
use super::super::types::device_id::{DeviceId, DeviceType};
use super::super::types::version::Version;
use super::super::types::status::{OperationalStatus, SystemStatus, ComponentStatus, Metric};
use super::super::config::logging::LogConfig;
use super::super::config::telemetry::{TelemetryConfig, PrivacyLevel};
use std::time::{SystemTime, UNIX_EPOCH};

/// Generate a timestamp for testing (current time)
pub fn test_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Create a standard test device ID
pub fn test_device_id() -> DeviceId {
    DeviceId::new("TEST-1234", DeviceType::Desktop)
}

/// Create a standard test version
pub fn test_version() -> Version {
    Version::new(1, 0, 0)
}

/// Create a test system status with some components
pub fn test_system_status() -> SystemStatus {
    let timestamp = test_timestamp();
    let mut status = SystemStatus::new(OperationalStatus::Normal, timestamp)
        .with_message("Test system status");
    
    // Add CPU component
    let mut cpu = ComponentStatus::new("CPU", OperationalStatus::Normal)
        .with_message("CPU is operating normally");
    
    cpu.add_metric(Metric::new("Temperature", 45.0)
        .with_unit("°C")
        .with_thresholds(Some(70.0), Some(85.0)));
    
    cpu.add_metric(Metric::new("Usage", 25.0)
        .with_unit("%")
        .with_thresholds(Some(80.0), Some(95.0)));
    
    // Add Memory component
    let mut memory = ComponentStatus::new("Memory", OperationalStatus::Normal)
        .with_message("Memory is operating normally");
    
    memory.add_metric(Metric::new("Usage", 40.0)
        .with_unit("%")
        .with_thresholds(Some(80.0), Some(90.0)));
    
    // Add Storage component
    let mut storage = ComponentStatus::new("Storage", OperationalStatus::Normal)
        .with_message("Storage is operating normally");
    
    storage.add_metric(Metric::new("Usage", 60.0)
        .with_unit("%")
        .with_thresholds(Some(85.0), Some(95.0)));
    
    // Add Network component
    let mut network = ComponentStatus::new("Network", OperationalStatus::Normal)
        .with_message("Network is operating normally");
    
    network.add_metric(Metric::new("Latency", 25.0)
        .with_unit("ms")
        .with_thresholds(Some(100.0), Some(200.0)));
    
    network.add_metric(Metric::new("Packet Loss", 0.5)
        .with_unit("%")
        .with_thresholds(Some(5.0), Some(10.0)));
    
    // Add components to system status
    status.add_component(cpu);
    status.add_component(memory);
    status.add_component(storage);
    status.add_component(network);
    
    status
}

/// Create a test log configuration
pub fn test_log_config() -> LogConfig {
    LogConfig::new(super::super::config::logging::LogLevel::Debug)
}

/// Create a test telemetry configuration
pub fn test_telemetry_config() -> TelemetryConfig {
    TelemetryConfig::new(true, PrivacyLevel::Medium)
}