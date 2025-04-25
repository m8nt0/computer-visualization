use super::error::{MemoryError, MemoryResult};
use std::collections::HashMap;

pub struct MemoryManager {
    page_table: HashMap<VirtualAddress, PhysicalAddress>,
    free_frames: Vec<PhysicalAddress>,
    allocated_pages: HashMap<ProcessId, Vec<VirtualAddress>>,
    config: MemoryConfig,
    stats: MemoryStats,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct VirtualAddress(u64);

#[derive(Clone, Copy)]
struct PhysicalAddress(u64);

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct ProcessId(u32);

struct MemoryConfig {
    page_size: usize,
    total_frames: usize,
    kernel_reserved: usize,
}

impl MemoryManager {
    pub fn new(config: MemoryConfig) -> Self {
        let mut free_frames = Vec::with_capacity(config.total_frames - config.kernel_reserved);
        for i in config.kernel_reserved..config.total_frames {
            free_frames.push(PhysicalAddress(i as u64 * config.page_size as u64));
        }

        Self {
            page_table: HashMap::new(),
            free_frames,
            allocated_pages: HashMap::new(),
            config,
            stats: MemoryStats::default(),
        }
    }

    pub fn allocate_pages(&mut self, process: ProcessId, count: usize) -> MemoryResult<Vec<VirtualAddress>> {
        if count > self.free_frames.len() {
            return Err(MemoryError::OutOfMemory);
        }

        let mut allocated = Vec::with_capacity(count);
        for _ in 0..count {
            if let Some(frame) = self.free_frames.pop() {
                let virtual_addr = self.generate_virtual_address();
                self.page_table.insert(virtual_addr, frame);
                allocated.push(virtual_addr);
            }
        }

        self.allocated_pages.entry(process)
            .or_default()
            .extend(allocated.clone());

        Ok(allocated)
    }

    pub fn free_pages(&mut self, process: ProcessId) {
        if let Some(pages) = self.allocated_pages.remove(&process) {
            for virtual_addr in pages {
                if let Some(frame) = self.page_table.remove(&virtual_addr) {
                    self.free_frames.push(frame);
                }
            }
        }
    }

    fn generate_virtual_address(&self) -> VirtualAddress {
        // Simple implementation - would need proper address space management
        VirtualAddress(self.page_table.len() as u64)
    }
} 