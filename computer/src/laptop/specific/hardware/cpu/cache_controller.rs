use std::collections::HashMap;

pub struct CacheController {
    l1_cache: Cache,
    l2_cache: Cache,
    l3_cache: Cache,
    memory_bus: *mut Bus,
    
    // Statistics for visualization
    access_count: u64,
    miss_count: u64,
}

struct Cache {
    lines: Vec<CacheLine>,
    sets: usize,
    ways: usize,
    line_size: usize,
    write_policy: WritePolicy,
    replacement_policy: ReplacementPolicy,
}

struct CacheLine {
    tag: u32,
    data: Vec<u8>,
    state: CacheState,
    last_access: u64,
    dirty: bool,
}

#[derive(Clone, Copy)]
pub enum CacheState {
    Modified,
    Exclusive,
    Shared,
    Invalid,
}

enum WritePolicy {
    WriteBack,
    WriteThrough,
}

enum ReplacementPolicy {
    LRU,
    FIFO,
    Random,
}

impl CacheController {
    pub fn new(memory_bus: *mut Bus) -> Self {
        Self {
            l1_cache: Cache::new(32, 8, 64, WritePolicy::WriteThrough),
            l2_cache: Cache::new(256, 8, 64, WritePolicy::WriteBack),
            l3_cache: Cache::new(2048, 16, 64, WritePolicy::WriteBack),
            memory_bus,
            access_count: 0,
            miss_count: 0,
        }
    }

    pub fn read(&mut self, address: u32) -> Option<u32> {
        self.access_count += 1;
        
        // Try L1 cache first
        if let Some(data) = self.l1_cache.read(address) {
            return Some(data);
        }
        
        // L1 miss, try L2
        self.miss_count += 1;
        if let Some(data) = self.l2_cache.read(address) {
            self.l1_cache.insert(address, data);
            return Some(data);
        }
        
        // L2 miss, try L3
        if let Some(data) = self.l3_cache.read(address) {
            self.l2_cache.insert(address, data);
            self.l1_cache.insert(address, data);
            return Some(data);
        }
        
        // Cache miss, fetch from memory
        let data = unsafe { (*self.memory_bus).read(address) };
        if let Some(data) = data {
            self.l3_cache.insert(address, data);
            self.l2_cache.insert(address, data);
            self.l1_cache.insert(address, data);
            Some(data)
        } else {
            None
        }
    }

    pub fn write(&mut self, address: u32, data: u32) {
        self.access_count += 1;
        
        // Write to L1 cache
        match self.l1_cache.write_policy {
            WritePolicy::WriteThrough => {
                // Write to all cache levels and memory
                self.l1_cache.write(address, data);
                self.l2_cache.write(address, data);
                self.l3_cache.write(address, data);
                unsafe { (*self.memory_bus).write(address, data); }
            }
            WritePolicy::WriteBack => {
                // Write only to L1, mark as dirty
                self.l1_cache.write(address, data);
            }
        }
    }

    // Methods for visualization system
    pub fn get_miss_rate(&self) -> f32 {
        if self.access_count == 0 {
            return 0.0;
        }
        self.miss_count as f32 / self.access_count as f32
    }

    pub fn get_cache_state(&self, level: usize) -> &Cache {
        match level {
            1 => &self.l1_cache,
            2 => &self.l2_cache,
            3 => &self.l3_cache,
            _ => panic!("Invalid cache level")
        }
    }

    pub fn get_line_state(&self, level: usize, set: usize, way: usize) -> CacheState {
        let cache = self.get_cache_state(level);
        let index = set * cache.ways + way;
        cache.lines[index].state
    }
}

impl Cache {
    pub fn new(sets: usize, ways: usize, line_size: usize, write_policy: WritePolicy) -> Self {
        let total_lines = sets * ways;
        Self {
            lines: vec![CacheLine {
                tag: 0,
                data: vec![0; line_size],
                state: CacheState::Invalid,
                last_access: 0,
                dirty: false,
            }; total_lines],
            sets,
            ways,
            line_size,
            write_policy,
            replacement_policy: ReplacementPolicy::LRU,
        }
    }

    pub fn read(&self, address: u32) -> Option<u32> {
        let set_index = self.get_set_index(address);
        let tag = self.get_tag(address);

        // Check if line exists in cache
        for way in 0..self.ways {
            let line_index = set_index * self.ways + way;
            let line = &self.lines[line_index];
            
            if line.tag == tag && line.state != CacheState::Invalid {
                // Cache hit
                let offset = self.get_offset(address);
                return Some(u32::from_le_bytes(line.data[offset..offset+4].try_into().unwrap()));
            }
        }
        None
    }

    pub fn write(&mut self, address: u32, data: u32) {
        let set_index = self.get_set_index(address);
        let tag = self.get_tag(address);
        let offset = self.get_offset(address);

        // Find or allocate cache line
        let line_index = self.find_or_allocate_line(set_index, tag);
        let line = &mut self.lines[line_index];

        // Write data
        line.data[offset..offset+4].copy_from_slice(&data.to_le_bytes());
        line.state = CacheState::Modified;
        line.dirty = true;
    }

    fn get_set_index(&self, address: u32) -> usize {
        ((address >> self.get_offset_bits()) & ((1 << self.get_set_bits()) - 1)) as usize
    }

    fn get_tag(&self, address: u32) -> u32 {
        address >> (self.get_offset_bits() + self.get_set_bits())
    }

    fn get_offset(&self, address: u32) -> usize {
        (address & ((1 << self.get_offset_bits()) - 1)) as usize
    }

    fn get_offset_bits(&self) -> u32 {
        (self.line_size as f32).log2() as u32
    }

    fn get_set_bits(&self) -> u32 {
        (self.sets as f32).log2() as u32
    }

    fn find_or_allocate_line(&mut self, set_index: usize, tag: u32) -> usize {
        // Try to find existing line
        let set_start = set_index * self.ways;
        for way in 0..self.ways {
            let line_index = set_start + way;
            if self.lines[line_index].tag == tag {
                return line_index;
            }
        }

        // Need to allocate new line
        self.evict_line(set_index)
    }

    fn evict_line(&mut self, set_index: usize) -> usize {
        // Simple LRU implementation
        let set_start = set_index * self.ways;
        let mut oldest_access = u64::MAX;
        let mut oldest_index = set_start;

        for way in 0..self.ways {
            let line_index = set_start + way;
            if self.lines[line_index].last_access < oldest_access {
                oldest_access = self.lines[line_index].last_access;
                oldest_index = line_index;
            }
        }

        oldest_index
    }

    fn insert(&mut self, address: u32, data: u32) {
        let set_index = self.get_set_index(address);
        let tag = self.get_tag(address);
        let offset = self.get_offset(address);

        // Find or allocate cache line
        let line_index = self.find_or_allocate_line(set_index, tag);
        let line = &mut self.lines[line_index];

        // Write data
        line.data[offset..offset+4].copy_from_slice(&data.to_le_bytes());
        line.state = CacheState::Modified;
        line.dirty = true;
    }
}
