// Export all modules in error
pub mod ecc;
pub mod logging;
pub mod scrubbing;

#[derive(Debug)]
pub enum MemoryError {
    // Access errors
    AddressOutOfRange,
    PageFault,
    SegmentationFault,
    PermissionDenied,
    
    // Cache errors
    CacheMiss,
    CacheCoherencyViolation,
    WritebackFailed,
    
    // DRAM errors
    RefreshError,
    TimingViolation,
    ECCError(ECCError),
    
    // Controller errors
    BankBusy,
    CommandQueueFull,
    PowerStateError,
    
    // System errors
    OutOfMemory,
    BusError,
    HardwareFailure,
}

#[derive(Debug)]
pub enum ECCError {
    SingleBitError { address: u64, syndrome: u8 },
    DoubleBitError { address: u64, syndrome: u8 },
    ChipFailure { chip_id: u8 },
    UncorrectableError { address: u64 },
}

pub type MemoryResult<T> = Result<T, MemoryError>;

// Error logging and tracking
#[derive(Debug, Clone)]
pub struct ErrorEvent {
    pub timestamp: u64,
    pub error_type: MemoryError,
    pub address: Option<u64>,
    pub corrected: bool,
    pub details: String,
}

impl MemoryError {
    pub fn is_recoverable(&self) -> bool {
        match self {
            MemoryError::CacheMiss => true,
            MemoryError::ECCError(ecc_err) => matches!(ecc_err, ECCError::SingleBitError {..}),
            MemoryError::BankBusy => true,
            MemoryError::CommandQueueFull => true,
            _ => false
        }
    }

    pub fn needs_immediate_attention(&self) -> bool {
        match self {
            MemoryError::ECCError(ECCError::ChipFailure {..}) => true,
            MemoryError::HardwareFailure => true,
            MemoryError::RefreshError => true,
            _ => false
        }
    }
}
