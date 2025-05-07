pub enum MemoryEvent {
    CacheAccess { hit: bool, level: CacheLevel },
    DRAMAccess { bank: usize, row: usize },
    PageFault { address: VirtualAddress },
    ErrorDetected { error_type: ErrorType },
    PowerStateChange { new_state: PowerState },
    // ... more events
} 