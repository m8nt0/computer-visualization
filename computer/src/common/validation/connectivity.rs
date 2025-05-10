// Network connection validation
use super::super::types::status::{Metric, OperationalStatus};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ConnectivityValidator {
    latency_warning_threshold_ms: u64,
    latency_critical_threshold_ms: u64,
    packet_loss_warning_threshold: f64, // Percentage (0.0-1.0)
    packet_loss_critical_threshold: f64, // Percentage (0.0-1.0)
    ping_timeout: Duration,
    max_retry_count: u32,
}

impl Default for ConnectivityValidator {
    fn default() -> Self {
        Self {
            latency_warning_threshold_ms: 100,
            latency_critical_threshold_ms: 250,
            packet_loss_warning_threshold: 0.05, // 5%
            packet_loss_critical_threshold: 0.15, // 15%
            ping_timeout: Duration::from_secs(2),
            max_retry_count: 3,
        }
    }
}

impl ConnectivityValidator {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_latency_thresholds(mut self, warning_ms: u64, critical_ms: u64) -> Self {
        self.latency_warning_threshold_ms = warning_ms;
        self.latency_critical_threshold_ms = critical_ms;
        self
    }
    
    pub fn with_packet_loss_thresholds(mut self, warning: f64, critical: f64) -> Self {
        self.packet_loss_warning_threshold = warning;
        self.packet_loss_critical_threshold = critical;
        self
    }
    
    pub fn with_ping_timeout(mut self, timeout: Duration) -> Self {
        self.ping_timeout = timeout;
        self
    }
    
    pub fn with_max_retry_count(mut self, count: u32) -> Self {
        self.max_retry_count = count;
        self
    }
    
    pub fn validate_latency(&self, latency_ms: u64) -> OperationalStatus {
        if latency_ms >= self.latency_critical_threshold_ms {
            OperationalStatus::Critical
        } else if latency_ms >= self.latency_warning_threshold_ms {
            OperationalStatus::Warning
        } else {
            OperationalStatus::Normal
        }
    }
    
    pub fn validate_packet_loss(&self, packet_loss: f64) -> OperationalStatus {
        if packet_loss >= self.packet_loss_critical_threshold {
            OperationalStatus::Critical
        } else if packet_loss >= self.packet_loss_warning_threshold {
            OperationalStatus::Warning
        } else {
            OperationalStatus::Normal
        }
    }
    
    pub fn create_latency_metric(&self, latency_ms: u64) -> Metric {
        Metric::new("Network Latency", latency_ms as f64)
            .with_unit("ms")
            .with_thresholds(
                Some(self.latency_warning_threshold_ms as f64),
                Some(self.latency_critical_threshold_ms as f64)
            )
    }
    
    pub fn create_packet_loss_metric(&self, packet_loss: f64) -> Metric {
        Metric::new("Packet Loss", packet_loss * 100.0)
            .with_unit("%")
            .with_thresholds(
                Some(self.packet_loss_warning_threshold * 100.0),
                Some(self.packet_loss_critical_threshold * 100.0)
            )
    }
    
    pub fn ping_timeout(&self) -> Duration {
        self.ping_timeout
    }
    
    pub fn max_retry_count(&self) -> u32 {
        self.max_retry_count
    }
}

// Connection quality categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionQuality {
    Excellent,
    Good,
    Fair,
    Poor,
    Unusable,
}

impl ConnectionQuality {
    pub fn from_metrics(latency_ms: u64, packet_loss: f64, bandwidth_mbps: f64) -> Self {
        // Simple classification based on all three metrics
        if latency_ms < 30 && packet_loss < 0.01 && bandwidth_mbps > 50.0 {
            ConnectionQuality::Excellent
        } else if latency_ms < 80 && packet_loss < 0.05 && bandwidth_mbps > 20.0 {
            ConnectionQuality::Good
        } else if latency_ms < 150 && packet_loss < 0.1 && bandwidth_mbps > 5.0 {
            ConnectionQuality::Fair
        } else if latency_ms < 300 && packet_loss < 0.2 && bandwidth_mbps > 1.0 {
            ConnectionQuality::Poor
        } else {
            ConnectionQuality::Unusable
        }
    }
    
    pub fn as_str(&self) -> &'static str {
        match self {
            ConnectionQuality::Excellent => "Excellent",
            ConnectionQuality::Good => "Good",
            ConnectionQuality::Fair => "Fair",
            ConnectionQuality::Poor => "Poor",
            ConnectionQuality::Unusable => "Unusable",
        }
    }
} 


// =====================================================================


// use std::time::Duration;

// pub struct ConnectionThresholds {
//     pub min_signal_strength: f32,
//     pub warning_latency_ms: u32,
//     pub critical_latency_ms: u32,
//     pub min_bandwidth: u64,
// }

// impl ConnectionThresholds {
//     pub fn new(min_signal_strength: f32, warning_latency_ms: u32, critical_latency_ms: u32, min_bandwidth: u64) -> Self {
//         Self {
//             min_signal_strength,
//             warning_latency_ms,
//             critical_latency_ms,
//             min_bandwidth,
//         }
//     }
    
//     pub fn is_signal_acceptable(&self, signal_strength: f32) -> bool {
//         signal_strength >= self.min_signal_strength
//     }
    
//     pub fn get_latency_status(&self, latency_ms: u32) -> LatencyStatus {
//         if latency_ms >= self.critical_latency_ms {
//             LatencyStatus::Critical
//         } else if latency_ms >= self.warning_latency_ms {
//             LatencyStatus::Warning
//         } else {
//             LatencyStatus::Good
//         }
//     }
    
//     pub fn is_bandwidth_sufficient(&self, bandwidth: u64) -> bool {
//         bandwidth >= self.min_bandwidth
//     }
// }

// pub enum LatencyStatus {
//     Good,
//     Warning,
//     Critical,
// }

// pub struct ConnectionQuality {
//     pub signal_strength: f32, // 0.0 to 1.0
//     pub latency: Duration,
//     pub packet_loss: f32, // 0.0 to 1.0
//     pub bandwidth: u64, // bits per second
// }

// impl ConnectionQuality {
//     pub fn new(signal_strength: f32, latency: Duration, packet_loss: f32, bandwidth: u64) -> Self {
//         Self {
//             signal_strength,
//             latency,
//             packet_loss,
//             bandwidth,
//         }
//     }
    
//     pub fn overall_quality(&self) -> f32 {
//         // Simple weighted average of normalized metrics
//         let latency_factor = 1.0 - (self.latency.as_millis() as f32 / 1000.0).min(1.0);
//         let packet_factor = 1.0 - self.packet_loss;
        
//         // Weighted sum (signal strength and packet loss weigh more than latency)
//         (self.signal_strength * 0.4) + (latency_factor * 0.2) + (packet_factor * 0.4)
//     }
    
//     pub fn quality_rating(&self) -> ConnectionRating {
//         let quality = self.overall_quality();
        
//         if quality >= 0.8 {
//             ConnectionRating::Excellent
//         } else if quality >= 0.6 {
//             ConnectionRating::Good
//         } else if quality >= 0.4 {
//             ConnectionRating::Fair
//         } else if quality >= 0.2 {
//             ConnectionRating::Poor
//         } else {
//             ConnectionRating::Critical
//         }
//     }
// }

// pub enum ConnectionRating {
//     Excellent,
//     Good,
//     Fair,
//     Poor,
//     Critical,
// }

// pub fn validate_ip_address(ip: &str) -> bool {
//     // Very basic IP validation
//     // In a real implementation, use a proper IP parsing library
    
//     // IPv4 simple validation
//     if ip.split('.').count() == 4 {
//         return ip.split('.')
//             .all(|octet| {
//                 if let Ok(num) = octet.parse::<u8>() {
//                     true
//                 } else {
//                     false
//                 }
//             });
//     }
    
//     // IPv6 simple validation
//     if ip.split(':').count() <= 8 {
//         return ip.split(':')
//             .all(|segment| {
//                 segment.is_empty() || 
//                 (segment.len() <= 4 && segment.chars().all(|c| c.is_ascii_hexdigit()))
//             });
//     }
    
//     false
// }


