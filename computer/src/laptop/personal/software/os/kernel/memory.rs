pub struct MemoryRegion {
    start: u16,
    size: u16,
    is_free: bool,
}

pub struct MemoryManager {
    regions: Vec<MemoryRegion>,
    #[allow(dead_code)]
    total_memory: u16,
}

impl MemoryManager {
    pub fn new() -> Self {
        Self {
            regions: vec![MemoryRegion {
                start: 0x1000, // Start after system reserved memory
                size: 0xE000,  // Up to 0xF000
                is_free: true,
            }],
            total_memory: 0xFFFF,
        }
    }

    pub fn allocate(&mut self, size: u16) -> Option<u16> {
        let mut i = 0;
        while i < self.regions.len() {
            if self.regions[i].is_free && self.regions[i].size >= size {
                let start = self.regions[i].start;
                
                // Split the region if it's larger than needed
                if self.regions[i].size > size {
                    let new_region = MemoryRegion {
                        start: start + size,
                        size: self.regions[i].size - size,
                        is_free: true,
                    };
                    self.regions[i].size = size;
                    self.regions.push(new_region);
                }
                
                self.regions[i].is_free = false;
                return Some(start);
            }
            i += 1;
        }
        None
    }

    pub fn free(&mut self, start: u16) {
        if let Some(region) = self.regions.iter_mut()
            .find(|r| r.start == start) {
            region.is_free = true;
            self.merge_free_regions();
        }
    }

    fn merge_free_regions(&mut self) {
        self.regions.sort_by_key(|r| r.start);
        let mut i = 0;
        while i < self.regions.len() - 1 {
            if self.regions[i].is_free && self.regions[i + 1].is_free {
                let next = self.regions.remove(i + 1);
                self.regions[i].size += next.size;
            } else {
                i += 1;
            }
        }
    }
}
