use super::error::{ServiceError, ServiceResult};
use std::collections::VecDeque;
use std::time::SystemTime;

pub struct Logger {
    logs: VecDeque<LogEntry>,
    filters: Vec<LogFilter>,
    writers: Vec<Box<dyn LogWriter>>,
    config: LoggerConfig,
}

struct LogEntry {
    timestamp: SystemTime,
    level: LogLevel,
    module: String,
    message: String,
    metadata: LogMetadata,
}

enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

struct LogMetadata {
    process_id: ProcessId,
    thread_id: ThreadId,
    file: String,
    line: u32,
}

impl Logger {
    pub fn new(config: LoggerConfig) -> Self {
        Self {
            logs: VecDeque::with_capacity(config.buffer_size),
            filters: Vec::new(),
            writers: Vec::new(),
            config,
        }
    }

    pub fn log(&mut self, level: LogLevel, module: &str, message: &str, metadata: LogMetadata) -> ServiceResult<()> {
        let entry = LogEntry {
            timestamp: SystemTime::now(),
            level,
            module: module.to_string(),
            message: message.to_string(),
            metadata,
        };

        // Apply filters
        if self.should_log(&entry) {
            self.logs.push_back(entry.clone());
            
            // Write to all configured outputs
            for writer in &mut self.writers {
                writer.write(&entry)?;
            }
        }

        Ok(())
    }

    fn should_log(&self, entry: &LogEntry) -> bool {
        self.filters.iter().all(|filter| filter.allow(entry))
    }
} 