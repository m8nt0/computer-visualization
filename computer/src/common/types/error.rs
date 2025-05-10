// Common error types
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    // Hardware related errors
    HardwareFailure,
    DeviceNotFound,
    DeviceDisconnected,
    
    // I/O errors
    IoError,
    FileNotFound,
    PermissionDenied,
    
    // Network errors
    NetworkError,
    ConnectionFailed,
    Timeout,
    
    // Security errors
    AuthenticationFailed,
    AuthorizationFailed,
    EncryptionFailed,
    
    // System errors
    ConfigurationError,
    ResourceExhausted,
    SystemOverload,
    
    // Application errors
    InvalidInput,
    OperationFailed,
    NotSupported,
    
    // Others
    Unknown,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct ComputerError {
    kind: ErrorKind,
    message: String,
    source: Option<Box<dyn Error + Send + Sync>>,
    context: Option<String>,
}

impl ComputerError {
    pub fn new(kind: ErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
            source: None,
            context: None,
        }
    }
    
    pub fn with_source<E: Error + Send + Sync + 'static>(mut self, source: E) -> Self {
        self.source = Some(Box::new(source));
        self
    }
    
    pub fn with_context(mut self, context: &str) -> Self {
        self.context = Some(context.to_string());
        self
    }
    
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
    
    pub fn message(&self) -> &str {
        &self.message
    }
    
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }
    
    pub fn is_hardware_error(&self) -> bool {
        matches!(
            self.kind,
            ErrorKind::HardwareFailure | ErrorKind::DeviceNotFound | ErrorKind::DeviceDisconnected
        )
    }
    
    pub fn is_io_error(&self) -> bool {
        matches!(
            self.kind,
            ErrorKind::IoError | ErrorKind::FileNotFound | ErrorKind::PermissionDenied
        )
    }
    
    pub fn is_network_error(&self) -> bool {
        matches!(
            self.kind,
            ErrorKind::NetworkError | ErrorKind::ConnectionFailed | ErrorKind::Timeout
        )
    }
    
    pub fn is_security_error(&self) -> bool {
        matches!(
            self.kind,
            ErrorKind::AuthenticationFailed | ErrorKind::AuthorizationFailed | ErrorKind::EncryptionFailed
        )
    }
}

impl fmt::Display for ComputerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        
        if let Some(context) = &self.context {
            write!(f, " ({})", context)?;
        }
        
        Ok(())
    }
}

impl Error for ComputerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|s| s.as_ref() as &(dyn Error + 'static))
    }
}

impl From<std::io::Error> for ComputerError {
    fn from(err: std::io::Error) -> Self {
        let kind = match err.kind() {
            std::io::ErrorKind::NotFound => ErrorKind::FileNotFound,
            std::io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            std::io::ErrorKind::TimedOut => ErrorKind::Timeout,
            _ => ErrorKind::IoError,
        };
        
        Self::new(kind, &format!("I/O error: {}", err))
            .with_source(err)
    }
}

impl From<&str> for ComputerError {
    fn from(msg: &str) -> Self {
        Self::new(ErrorKind::Unknown, msg)
    }
}

impl From<String> for ComputerError {
    fn from(msg: String) -> Self {
        Self::new(ErrorKind::Unknown, &msg)
    }
}

// Result type alias using ComputerError
pub type Result<T> = std::result::Result<T, ComputerError>;
