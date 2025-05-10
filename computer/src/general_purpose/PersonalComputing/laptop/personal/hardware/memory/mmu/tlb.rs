use std::collections::HashMap;
use super::super::types::{VirtualAddress, PhysicalAddress};
use super::paging::PageFlags;

pub struct TLB {
    entries: HashMap<u64, TLBEntry>,
    max_entries: usize,
    replacement_policy: ReplacementPolicy,
    stats: TLBStats,
}

struct TLBEntry {
    virtual_addr: VirtualAddress,
    physical_addr: PhysicalAddress,
    flags: PageFlags,
    asid: u16,
    last_access: u64,
    access_count: u64,
}

#[derive(Clone, Copy)]
enum ReplacementPolicy {
    LRU,
    FIFO,
    Random,
}

struct TLBStats {
    hits: u64,
    misses: u64,
    evictions: u64,
    invalidations: u64,
    cycles_saved: u64,
}

impl TLB {
    pub fn new() -> Self {
        Self {
            entries: HashMap::with_capacity(64),
            max_entries: 64,  // 64-entry TLB
            replacement_policy: ReplacementPolicy::LRU,
            stats: TLBStats::default(),
        }
    }

    pub fn lookup(&mut self, virtual_addr: VirtualAddress) -> Option<PhysicalAddress> {
        let page_addr = self.get_page_addr(virtual_addr);
        
        if let Some(entry) = self.entries.get_mut(&page_addr) {
            // Update access statistics
            entry.last_access = self.stats.hits + self.stats.misses;
            entry.access_count += 1;
            self.stats.hits += 1;
            self.stats.cycles_saved += 20;  // Assume 20 cycles saved per hit
            
            Some(entry.physical_addr)
        } else {
            self.stats.misses += 1;
            None
        }
    }

    pub fn insert(&mut self, virtual_addr: VirtualAddress, 
                 physical_addr: PhysicalAddress, flags: PageFlags) 
    {
        let page_addr = self.get_page_addr(virtual_addr);
        
        // Check if we need to evict an entry
        if self.entries.len() >= self.max_entries {
            self.evict_entry();
        }

        // Insert new entry
        self.entries.insert(page_addr, TLBEntry {
            virtual_addr,
            physical_addr,
            flags,
            asid: 0,  // Current ASID
            last_access: self.stats.hits + self.stats.misses,
            access_count: 0,
        });
    }

    pub fn invalidate(&mut self, virtual_addr: VirtualAddress) {
        let page_addr = self.get_page_addr(virtual_addr);
        if self.entries.remove(&page_addr).is_some() {
            self.stats.invalidations += 1;
        }
    }

    pub fn flush(&mut self) {
        self.entries.clear();
        self.stats.invalidations += 1;
    }

    fn evict_entry(&mut self) {
        match self.replacement_policy {
            ReplacementPolicy::LRU => self.evict_lru(),
            ReplacementPolicy::FIFO => self.evict_fifo(),
            ReplacementPolicy::Random => self.evict_random(),
        }
        self.stats.evictions += 1;
    }

    fn evict_lru(&mut self) {
        if let Some((&addr, _)) = self.entries.iter()
            .min_by_key(|(_, entry)| entry.last_access) {
            self.entries.remove(&addr);
        }
    }

    fn evict_fifo(&mut self) {
        if let Some((&addr, _)) = self.entries.iter().next() {
            self.entries.remove(&addr);
        }
    }

    fn evict_random(&mut self) {
        if let Some((&addr, _)) = self.entries.iter()
            .nth(fastrand::usize(..self.entries.len())) {
            self.entries.remove(&addr);
        }
    }

    fn get_page_addr(&self, addr: VirtualAddress) -> u64 {
        addr.0 & !0xFFF // Clear offset bits
    }

    // Methods for statistics and monitoring
    pub fn get_hit_rate(&self) -> f32 {
        let total = self.stats.hits + self.stats.misses;
        if total == 0 {
            return 0.0;
        }
        self.stats.hits as f32 / total as f32
    }

    pub fn get_stats(&self) -> &TLBStats {
        &self.stats
    }
}
