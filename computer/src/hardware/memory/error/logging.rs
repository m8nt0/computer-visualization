use super::super::types::{VirtualAddress, PhysicalAddress};
use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ErrorLogger {
    logs: VecDeque<ErrorLog>,
    max_logs: usize,
    severity_filter: ErrorSeverity,
    stats: LoggingStats,
}

#[derive(Clone, Debug)]
pub struct ErrorLog {
    timestamp: u64,
    error_type: ErrorType,
    severity: ErrorSeverity,
    virtual_addr: Option<VirtualAddress>,
    physical_addr: Option<PhysicalAddress>,
    details: String,
    corrected: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
    Fatal,
}

#[derive(Clone, Debug)]
pub enum ErrorType {
    HardwareFailure,
    MemoryCorruption,
    EccError,
    AddressError,
    PermissionError,
    TimingError,
    PowerError,
    ThermalError,
}

struct LoggingStats {
    total_errors: u64,
    corrected_errors: u64,
    uncorrected_errors: u64,
    critical_errors: u64,
}

impl ErrorLogger {
    pub fn new() -> Self {
        Self {
            logs: VecDeque::with_capacity(1000),
            max_logs: 1000,
            severity_filter: ErrorSeverity::Info,
            stats: LoggingStats::default(),
        }
    }

    pub fn log_error(&mut self, error_type: ErrorType, severity: ErrorSeverity,
                     virtual_addr: Option<VirtualAddress>,
                     physical_addr: Option<PhysicalAddress>,
                     details: String, corrected: bool) 
    {
        if severity >= self.severity_filter {
            let log = ErrorLog {
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                error_type,
                severity,
                virtual_addr,
                physical_addr,
                details,
                corrected,
            };

            // Update statistics
            self.stats.total_errors += 1;
            if corrected {
                self.stats.corrected_errors += 1;
            } else {
                self.stats.uncorrected_errors += 1;
            }
            if severity >= ErrorSeverity::Critical {
                self.stats.critical_errors += 1;
            }

            // Add log entry
            if self.logs.len() >= self.max_logs {
                self.logs.pop_front();
            }
            self.logs.push_back(log);
        }
    }

    pub fn get_logs(&self) -> &VecDeque<ErrorLog> {
        &self.logs
    }

    pub fn set_severity_filter(&mut self, severity: ErrorSeverity) {
        self.severity_filter = severity;
    }

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }
}
