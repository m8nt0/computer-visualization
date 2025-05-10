// Shared telemetry/reporting setup
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum TelemetryEvent {
    Boot,
    Shutdown,
    Error(String),
    PerformanceMetric { name: String, value: f64 },
    ResourceUsage { resource: String, value: f64 },
    UserAction { action: String, parameters: HashMap<String, String> },
    Custom { name: String, data: HashMap<String, String> },
}

#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    enabled: bool,
    batch_size: usize,
    send_interval: Duration,
    endpoint_url: Option<String>,
    privacy_level: PrivacyLevel,
    include_system_info: bool,
    include_device_id: bool,
    include_location: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrivacyLevel {
    High,   // Minimal data collection
    Medium, // Standard data collection
    Low,    // Detailed data collection
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            batch_size: 10,
            send_interval: Duration::from_secs(60),
            endpoint_url: None,
            privacy_level: PrivacyLevel::Medium,
            include_system_info: true,
            include_device_id: true,
            include_location: false,
        }
    }
}

impl TelemetryConfig {
    pub fn new(enabled: bool, privacy_level: PrivacyLevel) -> Self {
        let mut config = Self::default();
        config.enabled = enabled;
        config.privacy_level = privacy_level;
        config
    }
    
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    
    pub fn batch_size(&self) -> usize {
        self.batch_size
    }
    
    pub fn set_batch_size(&mut self, size: usize) {
        self.batch_size = size;
    }
    
    pub fn send_interval(&self) -> Duration {
        self.send_interval
    }
    
    pub fn set_send_interval(&mut self, interval: Duration) {
        self.send_interval = interval;
    }
    
    pub fn endpoint_url(&self) -> Option<&str> {
        self.endpoint_url.as_deref()
    }
    
    pub fn set_endpoint_url(&mut self, url: Option<String>) {
        self.endpoint_url = url;
    }
    
    pub fn privacy_level(&self) -> PrivacyLevel {
        self.privacy_level
    }
    
    pub fn set_privacy_level(&mut self, level: PrivacyLevel) {
        self.privacy_level = level;
    }
    
    pub fn include_system_info(&self) -> bool {
        self.include_system_info
    }
    
    pub fn set_include_system_info(&mut self, include: bool) {
        self.include_system_info = include;
    }
    
    pub fn include_device_id(&self) -> bool {
        self.include_device_id
    }
    
    pub fn set_include_device_id(&mut self, include: bool) {
        self.include_device_id = include;
    }
    
    pub fn include_location(&self) -> bool {
        self.include_location
    }
    
    pub fn set_include_location(&mut self, include: bool) {
        self.include_location = include;
    }
}

#[derive(Debug)]
pub struct TelemetryManager {
    config: TelemetryConfig,
    events: Vec<(TelemetryEvent, Instant)>,
    last_send: Instant,
}

impl TelemetryManager {
    pub fn new(config: TelemetryConfig) -> Self {
        Self {
            config,
            events: Vec::new(),
            last_send: Instant::now(),
        }
    }
    
    pub fn config(&self) -> &TelemetryConfig {
        &self.config
    }
    
    pub fn config_mut(&mut self) -> &mut TelemetryConfig {
        &mut self.config
    }
    
    pub fn record_event(&mut self, event: TelemetryEvent) {
        if !self.config.enabled {
            return;
        }
        
        // Check privacy settings for certain event types
        match &event {
            TelemetryEvent::UserAction { .. } if self.config.privacy_level == PrivacyLevel::High => {
                return; // Don't record user actions at high privacy level
            }
            TelemetryEvent::Custom { .. } if self.config.privacy_level == PrivacyLevel::High => {
                return; // Don't record custom events at high privacy level
            }
            _ => {}
        }
        
        self.events.push((event, Instant::now()));
        
        // Check if we should send the batch
        if self.events.len() >= self.config.batch_size || self.last_send.elapsed() >= self.config.send_interval {
            self.send_events();
        }
    }
    
    pub fn check_send(&mut self) {
        if !self.config.enabled || self.events.is_empty() {
            return;
        }
        
        if self.last_send.elapsed() >= self.config.send_interval {
            self.send_events();
        }
    }
    
    fn send_events(&mut self) {
        if let Some(url) = &self.config.endpoint_url {
            // In a real implementation, this would send the events to the endpoint
            println!("Sending {} telemetry events to {}", self.events.len(), url);
        } else {
            // Just simulate processing the events
            println!("Processing {} telemetry events", self.events.len());
        }
        
        self.events.clear();
        self.last_send = Instant::now();
    }
    
    pub fn pending_event_count(&self) -> usize {
        self.events.len()
    }
    
    pub fn force_send(&mut self) {
        if !self.config.enabled || self.events.is_empty() {
            return;
        }
        
        self.send_events();
    }
}
