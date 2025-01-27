use super::super::error::{MemoryError, MemoryResult};
use super::super::types::{VirtualAddress, PhysicalAddress, ProcessID};
use super::segmentation::{SegmentFlags, SegmentType};
use super::paging::PageFlags;
use std::collections::HashMap;
use std::ops::Range;
use std::fs::File;

pub struct MemoryMapper {
    mappings: HashMap<ProcessID, Vec<MemoryMapping>>,
    next_mapping_id: u64,
    stats: MappingStats,
}

#[derive(Clone)]
pub struct MemoryMapping {
    id: u64,
    virtual_range: Range<u64>,
    physical_range: Option<Range<u64>>,  // None for anonymous mappings
    flags: MappingFlags,
    mapping_type: MappingType,
    source: MappingSource,
}

#[derive(Clone)]
pub struct MappingFlags {
    pub segment_flags: SegmentFlags,
    pub shared: bool,
    pub growable: bool,
    pub pinned: bool,
}

#[derive(Clone)]
pub enum MappingType {
    Anonymous,
    File(FileMapping),
    Device(DeviceMapping),
    SharedMemory(SharedMapping),
}

#[derive(Clone)]
pub struct FileMapping {
    pub file: String,
    pub offset: u64,
    pub length: u64,
}

#[derive(Clone)]
pub struct DeviceMapping {
    pub device_id: u64,
    pub region: u64,
}

#[derive(Clone)]
pub struct SharedMapping {
    pub key: u64,
    pub creator: ProcessID,
}

struct MappingStats {
    total_mappings: u64,
    anonymous_mappings: u64,
    file_mappings: u64,
    shared_mappings: u64,
    mapping_faults: u64,
}

impl MemoryMapper {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            next_mapping_id: 1,
            stats: MappingStats::default(),
        }
    }

    pub fn map_anonymous(&mut self, pid: ProcessID, virtual_addr: VirtualAddress, 
                        size: u64, flags: MappingFlags) -> MemoryResult<u64> 
    {
        let mapping = MemoryMapping {
            id: self.next_mapping_id,
            virtual_range: virtual_addr.0..virtual_addr.0 + size,
            physical_range: None,
            flags,
            mapping_type: MappingType::Anonymous,
            source: MappingSource::Anonymous,
        };

        self.add_mapping(pid, mapping.clone())?;
        self.stats.anonymous_mappings += 1;
        self.next_mapping_id += 1;
        
        Ok(mapping.id)
    }

    pub fn map_file(&mut self, pid: ProcessID, virtual_addr: VirtualAddress,
                    file: String, offset: u64, size: u64, 
                    flags: MappingFlags) -> MemoryResult<u64> 
    {
        let mapping = MemoryMapping {
            id: self.next_mapping_id,
            virtual_range: virtual_addr.0..virtual_addr.0 + size,
            physical_range: None,  // Will be allocated on demand
            flags,
            mapping_type: MappingType::File(FileMapping {
                file,
                offset,
                length: size,
            }),
            source: MappingSource::File,
        };

        self.add_mapping(pid, mapping.clone())?;
        self.stats.file_mappings += 1;
        self.next_mapping_id += 1;
        
        Ok(mapping.id)
    }

    pub fn map_shared(&mut self, pid: ProcessID, virtual_addr: VirtualAddress,
                     size: u64, key: u64, flags: MappingFlags) -> MemoryResult<u64> 
    {
        let mapping = MemoryMapping {
            id: self.next_mapping_id,
            virtual_range: virtual_addr.0..virtual_addr.0 + size,
            physical_range: None,  // Will be shared with existing mapping
            flags,
            mapping_type: MappingType::SharedMemory(SharedMapping {
                key,
                creator: pid,
            }),
            source: MappingSource::Shared,
        };

        self.add_mapping(pid, mapping.clone())?;
        self.stats.shared_mappings += 1;
        self.next_mapping_id += 1;
        
        Ok(mapping.id)
    }

    pub fn unmap(&mut self, pid: ProcessID, virtual_addr: VirtualAddress) -> MemoryResult<()> {
        if let Some(mappings) = self.mappings.get_mut(&pid) {
            if let Some(index) = mappings.iter().position(|m| m.contains(virtual_addr)) {
                mappings.remove(index);
                return Ok(());
            }
        }
        Err(MemoryError::SegmentationFault)
    }

    fn add_mapping(&mut self, pid: ProcessID, mapping: MemoryMapping) -> MemoryResult<()> {
        let process_mappings = self.mappings.entry(pid).or_insert_with(Vec::new);
        
        // Check for overlaps
        if process_mappings.iter().any(|m| ranges_overlap(&m.virtual_range, &mapping.virtual_range)) {
            return Err(MemoryError::SegmentationFault);
        }

        process_mappings.push(mapping);
        self.stats.total_mappings += 1;
        Ok(())
    }

    // Methods for visualization
    pub fn get_process_mappings(&self, pid: ProcessID) -> Option<&Vec<MemoryMapping>> {
        self.mappings.get(&pid)
    }

    pub fn get_mapping_info(&self, pid: ProcessID, addr: VirtualAddress) -> Option<&MemoryMapping> {
        self.mappings.get(&pid)?
            .iter()
            .find(|m| m.contains(addr))
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MappingSource {
    Anonymous,
    File,
    Device,
    Shared,
}

impl MemoryMapping {
    fn contains(&self, addr: VirtualAddress) -> bool {
        self.virtual_range.contains(&addr.0)
    }
}

fn ranges_overlap(a: &Range<u64>, b: &Range<u64>) -> bool {
    a.start < b.end && b.start < a.end
} 