use super::super::error::{MemoryError, MemoryResult};
use super::super::types::{VirtualAddress, ProcessID};
use super::paging::PageFlags;
use std::collections::HashMap;
use std::ops::Range;

pub struct SegmentationUnit {
    segments: HashMap<ProcessID, Vec<MemorySegment>>,
    current_pid: ProcessID,
    kernel_segments: Vec<MemorySegment>,
    stats: SegmentStats,
}

#[derive(Clone)]
pub struct MemorySegment {
    range: Range<u64>,
    flags: SegmentFlags,
    segment_type: SegmentType,
    name: String,
}

#[derive(Clone, Copy)]
pub struct SegmentFlags {
    readable: bool,
    writable: bool,
    executable: bool,
    user_accessible: bool,
    cacheable: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SegmentType {
    Code,
    Data,
    Stack,
    Heap,
    SharedLibrary,
    MappedFile,
    Device,
}

struct SegmentStats {
    total_segments: u64,
    segment_faults: u64,
    permission_violations: u64,
}

impl SegmentationUnit {
    pub fn new() -> Self {
        let mut unit = Self {
            segments: HashMap::new(),
            current_pid: ProcessID(0),
            kernel_segments: Vec::new(),
            stats: SegmentStats::default(),
        };
        
        // Set up kernel segments
        unit.setup_kernel_segments();
        unit
    }

    pub fn check_access(&mut self, addr: VirtualAddress, is_write: bool, 
                       is_execute: bool) -> MemoryResult<SegmentFlags> 
    {
        let segments = if addr.0 >= 0xFFFF_8000_0000_0000 {
            &self.kernel_segments
        } else {
            self.segments.get(&self.current_pid)
                .ok_or(MemoryError::SegmentationFault)?
        };

        // Find containing segment
        for segment in segments {
            if segment.contains(addr) {
                return self.check_permissions(segment, is_write, is_execute);
            }
        }

        self.stats.segment_faults += 1;
        Err(MemoryError::SegmentationFault)
    }

    pub fn create_segment(&mut self, pid: ProcessID, range: Range<u64>, 
                         flags: SegmentFlags, segment_type: SegmentType, 
                         name: String) -> MemoryResult<()> 
    {
        // Check for overlapping segments
        let segments = self.segments.entry(pid).or_insert_with(Vec::new);
        if segments.iter().any(|s| ranges_overlap(&s.range, &range)) {
            return Err(MemoryError::SegmentationFault);
        }

        segments.push(MemorySegment {
            range,
            flags,
            segment_type,
            name,
        });
        self.stats.total_segments += 1;
        Ok(())
    }

    pub fn delete_segment(&mut self, pid: ProcessID, addr: VirtualAddress) -> MemoryResult<()> {
        if let Some(segments) = self.segments.get_mut(&pid) {
            if let Some(index) = segments.iter().position(|s| s.contains(addr)) {
                segments.remove(index);
                return Ok(());
            }
        }
        Err(MemoryError::SegmentationFault)
    }

    fn setup_kernel_segments(&mut self) {
        // Text segment
        self.kernel_segments.push(MemorySegment {
            range: 0xFFFF_8000_0000_0000..0xFFFF_8080_0000_0000,
            flags: SegmentFlags::kernel_code(),
            segment_type: SegmentType::Code,
            name: "kernel_text".to_string(),
        });

        // Data segment
        self.kernel_segments.push(MemorySegment {
            range: 0xFFFF_8080_0000_0000..0xFFFF_8100_0000_0000,
            flags: SegmentFlags::kernel_data(),
            segment_type: SegmentType::Data,
            name: "kernel_data".to_string(),
        });
    }

    fn check_permissions(&mut self, segment: &MemorySegment, 
                        is_write: bool, is_execute: bool) -> MemoryResult<SegmentFlags> 
    {
        if is_write && !segment.flags.writable {
            self.stats.permission_violations += 1;
            return Err(MemoryError::PermissionDenied);
        }

        if is_execute && !segment.flags.executable {
            self.stats.permission_violations += 1;
            return Err(MemoryError::PermissionDenied);
        }

        Ok(segment.flags)
    }

    // Methods for segment management
    pub fn get_segment_info(&self, addr: VirtualAddress) -> Option<&MemorySegment> {
        let segments = if addr.0 >= 0xFFFF_8000_0000_0000 {
            &self.kernel_segments
        } else {
            self.segments.get(&self.current_pid)?
        };

        segments.iter().find(|s| s.contains(addr))
    }

    pub fn set_current_process(&mut self, pid: ProcessID) {
        self.current_pid = pid;
    }
}

impl MemorySegment {
    fn contains(&self, addr: VirtualAddress) -> bool {
        self.range.contains(&addr.0)
    }
}

impl SegmentFlags {
    pub fn kernel_code() -> Self {
        Self {
            readable: true,
            writable: false,
            executable: true,
            user_accessible: false,
            cacheable: true,
        }
    }

    pub fn kernel_data() -> Self {
        Self {
            readable: true,
            writable: true,
            executable: false,
            user_accessible: false,
            cacheable: true,
        }
    }

    pub fn to_page_flags(&self) -> PageFlags {
        PageFlags {
            present: true,
            writable: self.writable,
            user_accessible: self.user_accessible,
            write_through: !self.cacheable,
            cache_disabled: !self.cacheable,
            executable: self.executable,
            global: !self.user_accessible,
        }
    }
}

fn ranges_overlap(a: &Range<u64>, b: &Range<u64>) -> bool {
    a.start < b.end && b.start < a.end
} 