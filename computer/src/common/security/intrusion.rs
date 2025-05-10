// Intrusion detection
use std::collections::VecDeque;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct SecurityEvent {
    event_type: String,
    source: String,
    timestamp: Instant,
    threat_level: ThreatLevel,
    details: String,
    resolved: bool,
}

impl SecurityEvent {
    pub fn new(
        event_type: &str,
        source: &str,
        threat_level: ThreatLevel,
        details: &str,
    ) -> Self {
        Self {
            event_type: event_type.to_string(),
            source: source.to_string(),
            timestamp: Instant::now(),
            threat_level,
            details: details.to_string(),
            resolved: false,
        }
    }
    
    pub fn event_type(&self) -> &str {
        &self.event_type
    }
    
    pub fn source(&self) -> &str {
        &self.source
    }
    
    pub fn timestamp(&self) -> Instant {
        self.timestamp
    }
    
    pub fn threat_level(&self) -> &ThreatLevel {
        &self.threat_level
    }
    
    pub fn details(&self) -> &str {
        &self.details
    }
    
    pub fn is_resolved(&self) -> bool {
        self.resolved
    }
    
    pub fn resolve(&mut self) {
        self.resolved = true;
    }
    
    pub fn elapsed(&self) -> Duration {
        self.timestamp.elapsed()
    }
}

#[derive(Debug)]
pub struct IntrusionDetectionSystem {
    events: VecDeque<SecurityEvent>,
    max_events: usize,
    alert_threshold: ThreatLevel,
    auto_response_enabled: bool,
}

impl IntrusionDetectionSystem {
    pub fn new(max_events: usize, alert_threshold: ThreatLevel) -> Self {
        Self {
            events: VecDeque::with_capacity(max_events),
            max_events,
            alert_threshold,
            auto_response_enabled: false,
        }
    }
    
    pub fn record_event(&mut self, event: SecurityEvent) {
        // Trigger alert if threat level meets or exceeds threshold
        if self.should_alert(&event) {
            self.trigger_alert(&event);
        }
        
        // Automatic response for critical threats
        if self.auto_response_enabled && event.threat_level() == &ThreatLevel::Critical {
            self.auto_respond(&event);
        }
        
        // Add to event queue
        self.events.push_back(event);
        
        // Maintain maximum size
        while self.events.len() > self.max_events {
            self.events.pop_front();
        }
    }
    
    pub fn recent_events(&self, count: usize) -> Vec<&SecurityEvent> {
        self.events.iter().rev().take(count).collect()
    }
    
    pub fn unresolved_events(&self) -> Vec<&SecurityEvent> {
        self.events.iter().filter(|e| !e.is_resolved()).collect()
    }
    
    pub fn resolve_event(&mut self, event_type: &str, source: &str) -> usize {
        let mut resolved_count = 0;
        
        for event in &mut self.events {
            if event.event_type() == event_type && event.source() == source && !event.is_resolved() {
                event.resolve();
                resolved_count += 1;
            }
        }
        
        resolved_count
    }
    
    pub fn set_alert_threshold(&mut self, threshold: ThreatLevel) {
        self.alert_threshold = threshold;
    }
    
    pub fn alert_threshold(&self) -> &ThreatLevel {
        &self.alert_threshold
    }
    
    pub fn enable_auto_response(&mut self, enabled: bool) {
        self.auto_response_enabled = enabled;
    }
    
    pub fn is_auto_response_enabled(&self) -> bool {
        self.auto_response_enabled
    }
    
    pub fn clear_resolved_events(&mut self) {
        self.events.retain(|e| !e.is_resolved());
    }
    
    fn should_alert(&self, event: &SecurityEvent) -> bool {
        match (event.threat_level(), &self.alert_threshold) {
            (ThreatLevel::Low, _) => false,
            (ThreatLevel::Medium, ThreatLevel::Low) => true,
            (ThreatLevel::Medium, _) => false,
            (ThreatLevel::High, ThreatLevel::Critical) => false,
            (ThreatLevel::High, _) => true,
            (ThreatLevel::Critical, _) => true,
        }
    }
    
    fn trigger_alert(&self, event: &SecurityEvent) {
        // In a real system, this would send notifications or log to a security system
        println!(
            "SECURITY ALERT: {} from {} - Threat Level: {:?} - {}",
            event.event_type(),
            event.source(),
            event.threat_level(),
            event.details()
        );
    }
    
    fn auto_respond(&self, event: &SecurityEvent) {
        // In a real system, this would implement automatic countermeasures
        println!(
            "AUTO RESPONSE: Blocking {} after {:?} threat detected: {}",
            event.source(),
            event.threat_level(),
            event.event_type()
        );
    }
} 