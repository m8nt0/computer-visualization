use super::super::memory::cache::{L1ICache, L1DCache};

pub struct CPUCacheInterface {
    l1i: *mut L1ICache,
    l1d: *mut L1DCache,
    stats: CacheInterfaceStats,
}

impl CPUCacheInterface {
    pub fn new(l1i: *mut L1ICache, l1d: *mut L1DCache) -> Self {
        Self {
            l1i,
            l1d,
            stats: CacheInterfaceStats::default(),
        }
    }

    pub fn read_instruction(&mut self, address: u64) -> Option<u32> {
        unsafe { (*self.l1i).read(address) }
    }

    pub fn read_data(&mut self, address: u64) -> Option<u32> {
        unsafe { (*self.l1d).read(address) }
    }

    pub fn write_data(&mut self, address: u64, data: u32) {
        unsafe { (*self.l1d).write(address, data); }
    }
} 