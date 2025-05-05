pub mod levels;
pub mod policies;

// adress the error
use super::error::{CacheError, CacheResult};
use std::collections::HashMap;

pub struct CacheHierarchy {
    l1_instruction: Cache,
    l1_data: Cache,
    l2_unified: Cache,
    l3_unified: Cache,
}

struct Cache {
    lines: HashMap<CacheLineAddress, CacheLine>,
    config: CacheConfig,
    stats: CacheStats,
}

struct CacheLine {
    data: Vec<u8>,
    tag: u64,
    flags: CacheFlags,
    last_access: u64,
}

bitflags! {
    struct CacheFlags: u8 {
        const VALID = 0x01;
        const DIRTY = 0x02;
        const EXCLUSIVE = 0x04;
        const SHARED = 0x08;
    }
}

impl CacheHierarchy {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            l1_instruction: Cache::new(config.l1_instruction),
            l1_data: Cache::new(config.l1_data),
            l2_unified: Cache::new(config.l2),
            l3_unified: Cache::new(config.l3),
        }
    }

    pub fn read(&mut self, address: u64) -> CacheResult<Vec<u8>> {
        // Try L1 first
        if let Ok(data) = self.l1_data.read(address) {
            return Ok(data);
        }

        // Try L2
        if let Ok(data) = self.l2_unified.read(address) {
            self.l1_data.insert(address, &data)?;
            return Ok(data);
        }

        // Try L3
        if let Ok(data) = self.l3_unified.read(address) {
            self.l2_unified.insert(address, &data)?;
            self.l1_data.insert(address, &data)?;
            return Ok(data);
        }

        Err(CacheError::Miss)
    }
} 