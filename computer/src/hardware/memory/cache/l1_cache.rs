use super::coherency::CoherencyState;
use super::replacement::ReplacementPolicy;
use super::stats::CacheStats;
use super::super::error::{MemoryError, MemoryResult};
use super::super::types::PhysicalAddress;

pub struct L1ICache {
    sets: Vec<CacheSet>,
    ways: usize,
    line_size: usize,
    latency: u8,
    stats: CacheStats,
}

pub struct L1DCache {
    sets: Vec<CacheSet>,
    ways: usize,
    line_size: usize,
    latency: u8,
    write_policy: WritePolicy,
    stats: CacheStats,
}

struct CacheSet {
    lines: Vec<CacheLine>,
    replacement: ReplacementPolicy,
}

struct CacheLine {
    valid: bool,
    dirty: bool,
    tag: u64,
    data: Vec<u8>,
    coherency_state: CoherencyState,
    access_info: AccessInfo,
}

struct AccessInfo {
    last_access: u64,
    access_count: u64,
    reuse_distance: u64,
}

#[derive(Clone, Copy)]
enum WritePolicy {
    WriteThrough,
    WriteBack,
}

impl L1ICache {
    pub fn new(size: usize, ways: usize) -> Self {
        let sets = size / (ways * 64); // 64-byte cache lines
        Self {
            sets: (0..sets).map(|_| CacheSet::new(ways)).collect(),
            ways,
            line_size: 64,
            latency: 1, // 1 cycle latency for L1I
            stats: CacheStats::default(),
        }
    }

    pub fn read(&mut self, address: PhysicalAddress) -> MemoryResult<u32> {
        let (tag, set_index, offset) = self.decode_address(address.0);
        let set = &mut self.sets[set_index];

        // Check for hit
        for line in &mut set.lines {
            if line.valid && line.tag == tag {
                self.stats.hits += 1;
                line.access_info.last_access = self.stats.total_accesses;
                line.access_info.access_count += 1;
                return Ok(self.read_from_line(line, offset));
            }
        }

        // Cache miss
        self.stats.misses += 1;
        Err(MemoryError::CacheMiss)
    }

    fn decode_address(&self, address: u64) -> (u64, usize, usize) {
        let offset_bits = (self.line_size as f64).log2() as u32;
        let index_bits = (self.sets.len() as f64).log2() as u32;
        
        let offset = (address & ((1 << offset_bits) - 1)) as usize;
        let set_index = ((address >> offset_bits) & ((1 << index_bits) - 1)) as usize;
        let tag = address >> (offset_bits + index_bits);

        (tag, set_index, offset)
    }

    fn read_from_line(&self, line: &CacheLine, offset: usize) -> u32 {
        let bytes = &line.data[offset..offset + 4];
        u32::from_le_bytes(bytes.try_into().unwrap())
    }
}

impl L1DCache {
    pub fn new(size: usize, ways: usize) -> Self {
        let sets = size / (ways * 64);
        Self {
            sets: (0..sets).map(|_| CacheSet::new(ways)).collect(),
            ways,
            line_size: 64,
            latency: 2, // 2 cycles latency for L1D
            write_policy: WritePolicy::WriteBack,
            stats: CacheStats::default(),
        }
    }

    pub fn read(&mut self, address: PhysicalAddress) -> MemoryResult<u32> {
        let (tag, set_index, offset) = self.decode_address(address.0);
        let set = &mut self.sets[set_index];

        // Check for hit
        for line in &mut set.lines {
            if line.valid && line.tag == tag {
                self.stats.hits += 1;
                line.access_info.last_access = self.stats.total_accesses;
                line.access_info.access_count += 1;
                return Ok(self.read_from_line(line, offset));
            }
        }

        // Cache miss
        self.stats.misses += 1;
        Err(MemoryError::CacheMiss)
    }

    pub fn write(&mut self, address: PhysicalAddress, data: u32) -> MemoryResult<()> {
        let (tag, set_index, offset) = self.decode_address(address.0);
        let set = &mut self.sets[set_index];

        // Check for hit
        for line in &mut set.lines {
            if line.valid && line.tag == tag {
                self.stats.hits += 1;
                line.access_info.last_access = self.stats.total_accesses;
                line.access_info.access_count += 1;
                self.write_to_line(line, offset, data);
                return Ok(());
            }
        }

        // Cache miss
        self.stats.misses += 1;
        Err(MemoryError::CacheMiss)
    }

    fn write_to_line(&mut self, line: &mut CacheLine, offset: usize, data: u32) {
        line.data[offset..offset + 4].copy_from_slice(&data.to_le_bytes());
        line.dirty = true;
        line.coherency_state = CoherencyState::Modified;
    }
}

impl CacheSet {
    fn new(ways: usize) -> Self {
        Self {
            lines: vec![CacheLine::new(); ways],
            replacement: ReplacementPolicy::new(),
        }
    }
}

impl CacheLine {
    fn new() -> Self {
        Self {
            valid: false,
            dirty: false,
            tag: 0,
            data: vec![0; 64],
            coherency_state: CoherencyState::Invalid,
            access_info: AccessInfo {
                last_access: 0,
                access_count: 0,
                reuse_distance: 0,
            },
        }
    }
} 