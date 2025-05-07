use super::super::error::{MemoryError, MemoryResult};
use super::super::types::PhysicalAddress;
use std::collections::HashMap;

pub struct TimingController {
    // Timing parameters (in cycles)
    tCK: u32,  // Clock cycle time
    tRCD: u32, // RAS to CAS delay
    tRP: u32,  // Row precharge time
    tRAS: u32, // Row active time
    tRC: u32,  // Row cycle time
    tWR: u32,  // Write recovery time
    tCCD: u32, // Column to column delay
    tRRD: u32, // Row to row delay
    tWTR: u32, // Write to read delay
    tRTP: u32, // Read to precharge delay
    
    // Last operation timestamps
    last_activate: HashMap<(usize, usize), u64>,  // (rank, bank) -> timestamp
    last_precharge: HashMap<(usize, usize), u64>,
    last_read: HashMap<(usize, usize), u64>,
    last_write: HashMap<(usize, usize), u64>,
    
    current_cycle: u64,
    stats: TimingStats,
}

struct TimingStats {
    timing_violations: u64,
    total_checks: u64,
    violations_by_type: HashMap<&'static str, u64>,
}

impl TimingController {
    pub fn new() -> Self {
        Self {
            // DDR4-2400 timings
            tCK: 1,
            tRCD: 14,
            tRP: 14,
            tRAS: 32,
            tRC: 46,
            tWR: 15,
            tCCD: 4,
            tRRD: 4,
            tWTR: 8,
            tRTP: 8,
            
            last_activate: HashMap::new(),
            last_precharge: HashMap::new(),
            last_read: HashMap::new(),
            last_write: HashMap::new(),
            
            current_cycle: 0,
            stats: TimingStats::default(),
        }
    }

    pub fn tick(&mut self) {
        self.current_cycle += 1;
    }

    pub fn check_read_timing(&mut self, address: PhysicalAddress) -> MemoryResult<()> {
        self.stats.total_checks += 1;
        let (rank, bank) = self.get_rank_bank(address);
        let key = (rank, bank);

        // Check tRCD (time since activate)
        if let Some(&last) = self.last_activate.get(&key) {
            if self.current_cycle < last + self.tRCD as u64 {
                self.record_violation("tRCD");
                return Err(MemoryError::TimingViolation);
            }
        }

        // Check tCCD (time since last read)
        if let Some(&last) = self.last_read.get(&key) {
            if self.current_cycle < last + self.tCCD as u64 {
                self.record_violation("tCCD");
                return Err(MemoryError::TimingViolation);
            }
        }

        Ok(())
    }

    pub fn check_write_timing(&mut self, address: PhysicalAddress) -> MemoryResult<()> {
        self.stats.total_checks += 1;
        let (rank, bank) = self.get_rank_bank(address);
        let key = (rank, bank);

        // Check tRCD (time since activate)
        if let Some(&last) = self.last_activate.get(&key) {
            if self.current_cycle < last + self.tRCD as u64 {
                self.record_violation("tRCD");
                return Err(MemoryError::TimingViolation);
            }
        }

        // Check tCCD (time since last write)
        if let Some(&last) = self.last_write.get(&key) {
            if self.current_cycle < last + self.tCCD as u64 {
                self.record_violation("tCCD");
                return Err(MemoryError::TimingViolation);
            }
        }

        Ok(())
    }

    fn get_rank_bank(&self, address: PhysicalAddress) -> (usize, usize) {
        let addr = address.0;
        let bank = ((addr >> 13) & 0x7) as usize;
        let rank = ((addr >> 16) & 0x3) as usize;
        (rank, bank)
    }

    fn record_violation(&mut self, timing_param: &'static str) {
        self.stats.timing_violations += 1;
        *self.stats.violations_by_type.entry(timing_param).or_insert(0) += 1;
    }
}
