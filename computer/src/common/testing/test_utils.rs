// Test utilities for common components
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Creates a simple simulated delay
pub fn simulate_delay(millis: u64) {
    std::thread::sleep(Duration::from_millis(millis));
}

/// Generates a random number between min and max (inclusive)
pub fn random_in_range(min: i32, max: i32) -> i32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as i32;
        
    min + (seed % (max - min + 1))
}

/// Represents a simple test environment
pub struct TestEnvironment {
    pub temp_celsius: f64,
    pub power_watts: f64,
    pub memory_usage_pct: f64,
    pub cpu_usage_pct: f64,
    pub network_latency_ms: u64,
    pub packet_loss_pct: f64,
}

impl Default for TestEnvironment {
    fn default() -> Self {
        Self {
            temp_celsius: 35.0,
            power_watts: 50.0,
            memory_usage_pct: 0.4,
            cpu_usage_pct: 0.3,
            network_latency_ms: 50,
            packet_loss_pct: 0.01,
        }
    }
}

impl TestEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_temp(mut self, temp_celsius: f64) -> Self {
        self.temp_celsius = temp_celsius;
        self
    }
    
    pub fn with_power(mut self, power_watts: f64) -> Self {
        self.power_watts = power_watts;
        self
    }
    
    pub fn with_memory_usage(mut self, memory_usage_pct: f64) -> Self {
        self.memory_usage_pct = memory_usage_pct;
        self
    }
    
    pub fn with_cpu_usage(mut self, cpu_usage_pct: f64) -> Self {
        self.cpu_usage_pct = cpu_usage_pct;
        self
    }
    
    pub fn with_network_metrics(mut self, latency_ms: u64, packet_loss_pct: f64) -> Self {
        self.network_latency_ms = latency_ms;
        self.packet_loss_pct = packet_loss_pct;
        self
    }
}

/// A utility for capturing log messages during tests
pub struct TestLogger {
    logs: Arc<Mutex<Vec<String>>>,
}

impl TestLogger {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn log(&self, message: &str) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.push(message.to_string());
        }
    }
    
    pub fn get_logs(&self) -> Vec<String> {
        if let Ok(logs) = self.logs.lock() {
            logs.clone()
        } else {
            Vec::new()
        }
    }
    
    pub fn clear(&self) {
        if let Ok(mut logs) = self.logs.lock() {
            logs.clear();
        }
    }
    
    pub fn contains(&self, substring: &str) -> bool {
        if let Ok(logs) = self.logs.lock() {
            logs.iter().any(|log| log.contains(substring))
        } else {
            false
        }
    }
} 