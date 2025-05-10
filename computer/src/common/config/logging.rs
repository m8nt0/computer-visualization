// Logging configuration used by any device
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogConfig {
    level: LogLevel,
    show_timestamp: bool,
    show_source: bool,
    max_log_size: Option<usize>,
    log_rotation_count: Option<usize>,
    log_file_path: Option<String>,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Info,
            show_timestamp: true,
            show_source: true,
            max_log_size: Some(10 * 1024 * 1024), // 10 MB
            log_rotation_count: Some(5),
            log_file_path: None,
        }
    }
}

impl LogConfig {
    pub fn new(level: LogLevel) -> Self {
        let mut config = Self::default();
        config.level = level;
        config
    }
    
    pub fn level(&self) -> LogLevel {
        self.level
    }
    
    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }
    
    pub fn show_timestamp(&self) -> bool {
        self.show_timestamp
    }
    
    pub fn set_show_timestamp(&mut self, show: bool) {
        self.show_timestamp = show;
    }
    
    pub fn show_source(&self) -> bool {
        self.show_source
    }
    
    pub fn set_show_source(&mut self, show: bool) {
        self.show_source = show;
    }
    
    pub fn max_log_size(&self) -> Option<usize> {
        self.max_log_size
    }
    
    pub fn set_max_log_size(&mut self, size: Option<usize>) {
        self.max_log_size = size;
    }
    
    pub fn log_rotation_count(&self) -> Option<usize> {
        self.log_rotation_count
    }
    
    pub fn set_log_rotation_count(&mut self, count: Option<usize>) {
        self.log_rotation_count = count;
    }
    
    pub fn log_file_path(&self) -> Option<&str> {
        self.log_file_path.as_deref()
    }
    
    pub fn set_log_file_path(&mut self, path: Option<String>) {
        self.log_file_path = path;
    }
    
    pub fn should_log(&self, level: LogLevel) -> bool {
        level >= self.level
    }
}

#[derive(Debug, Clone)]
pub struct Logger {
    config: LogConfig,
}

impl Logger {
    pub fn new(config: LogConfig) -> Self {
        Self { config }
    }
    
    pub fn config(&self) -> &LogConfig {
        &self.config
    }
    
    pub fn config_mut(&mut self) -> &mut LogConfig {
        &mut self.config
    }
    
    pub fn log(&self, level: LogLevel, source: &str, message: &str) {
        if !self.config.should_log(level) {
            return;
        }
        
        let mut log_entry = String::new();
        
        if self.config.show_timestamp {
            // In a real implementation, this would use actual time
            log_entry.push_str("[TIMESTAMP] ");
        }
        
        log_entry.push_str(&format!("[{}] ", level));
        
        if self.config.show_source {
            log_entry.push_str(&format!("[{}] ", source));
        }
        
        log_entry.push_str(message);
        
        // In a real implementation, this would write to the log file
        // or use a proper logging framework
        println!("{}", log_entry);
    }
    
    pub fn trace(&self, source: &str, message: &str) {
        self.log(LogLevel::Trace, source, message);
    }
    
    pub fn debug(&self, source: &str, message: &str) {
        self.log(LogLevel::Debug, source, message);
    }
    
    pub fn info(&self, source: &str, message: &str) {
        self.log(LogLevel::Info, source, message);
    }
    
    pub fn warning(&self, source: &str, message: &str) {
        self.log(LogLevel::Warning, source, message);
    }
    
    pub fn error(&self, source: &str, message: &str) {
        self.log(LogLevel::Error, source, message);
    }
    
    pub fn critical(&self, source: &str, message: &str) {
        self.log(LogLevel::Critical, source, message);
    }
}



// ===============================================================


// use crate::common::config::logging::{LogLevel, LoggingConfig};
// use chrono::Utc;

// pub fn log(level: LogLevel, module: &str, message: &str) {
//     let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
//     let level_str = match level {
//         LogLevel::Error => "ERROR",
//         LogLevel::Warning => "WARN ",
//         LogLevel::Info => "INFO ",
//         LogLevel::Debug => "DEBUG",
//         LogLevel::Trace => "TRACE",
//     };
//     println!("{} {} [{}] {}", timestamp, level_str, module, message);
// }

// pub fn format_log_message(level: LogLevel, module: &str, message: &str, config: &LoggingConfig) -> String {
//     match config.format {
//         crate::common::config::logging::LogFormat::Plain => {
//             format!("{} [{}] {}", level_to_string(level), module, message)
//         },
//         crate::common::config::logging::LogFormat::Json => {
//             format!(
//                 r#"{{"timestamp":"{}", "level":"{}", "module":"{}", "message":"{}"}}"#,
//                 Utc::now().to_rfc3339(),
//                 level_to_string(level),
//                 module,
//                 message.replace('"', "\\\"") // Escape quotes for JSON
//             )
//         },
//         crate::common::config::logging::LogFormat::Xml => {
//             format!(
//                 r#"<log><timestamp>{}</timestamp><level>{}</level><module>{}</module><message>{}</message></log>"#,
//                 Utc::now().to_rfc3339(),
//                 level_to_string(level),
//                 module,
//                 // Replace special characters for XML
//                 message.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace("\"", "&quot;")
//             )
//         },
//         crate::common::config::logging::LogFormat::Binary => {
//             // In a real implementation, this would serialize to a binary format
//             // For this example, we'll just return a placeholder
//             format!("BINARY_LOG:{:?}:{:?}:{:?}", level, module, message)
//         },
//     }
// }

// fn level_to_string(level: LogLevel) -> &'static str {
//     match level {
//         LogLevel::Error => "ERROR",
//         LogLevel::Warning => "WARNING",
//         LogLevel::Info => "INFO",
//         LogLevel::Debug => "DEBUG",
//         LogLevel::Trace => "TRACE",
//     }
// }

// pub fn should_log(message_level: LogLevel, config_level: LogLevel) -> bool {
//     // Convert levels to integers for comparison
//     let message_value = level_to_value(message_level);
//     let config_value = level_to_value(config_level);
    
//     // Log if message level is less than or equal to config level
//     // (ERROR is more severe than DEBUG, so ERROR has a lower value)
//     message_value <= config_value
// }

// fn level_to_value(level: LogLevel) -> u8 {
//     match level {
//         LogLevel::Error => 1,
//         LogLevel::Warning => 2,
//         LogLevel::Info => 3,
//         LogLevel::Debug => 4,
//         LogLevel::Trace => 5,
//     }
// }