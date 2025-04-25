use super::super::error::{MemoryError, MemoryResult};
use super::super::types::{VirtualAddress, ProcessID};
use super::paging::PageFlags;

pub struct MemoryProtection {
    current_pid: ProcessID,
    kernel_mode: bool,
    write_protect: bool,
    nx_enabled: bool,  // No-Execute protection
    stats: ProtectionStats,
}

struct ProtectionStats {
    permission_violations: u64,
    segmentation_faults: u64,
    privilege_violations: u64,
}

impl MemoryProtection {
    pub fn new() -> Self {
        Self {
            current_pid: ProcessID(0),
            kernel_mode: true,
            write_protect: true,
            nx_enabled: true,
            stats: ProtectionStats::default(),
        }
    }

    pub fn check_access(&mut self, addr: VirtualAddress, flags: PageFlags, 
                       is_write: bool, is_execute: bool) -> MemoryResult<()> 
    {
        // Check basic permissions
        if is_write && !flags.writable {
            self.stats.permission_violations += 1;
            return Err(MemoryError::PermissionDenied);
        }

        if is_execute && !flags.executable && self.nx_enabled {
            self.stats.permission_violations += 1;
            return Err(MemoryError::PermissionDenied);
        }

        // Check privilege level
        if !self.kernel_mode && !flags.user_accessible {
            self.stats.privilege_violations += 1;
            return Err(MemoryError::PermissionDenied);
        }

        // Check address space bounds
        if !self.is_address_valid(addr) {
            self.stats.segmentation_faults += 1;
            return Err(MemoryError::SegmentationFault);
        }

        Ok(())
    }

    pub fn set_kernel_mode(&mut self, enabled: bool) {
        self.kernel_mode = enabled;
    }

    pub fn set_current_process(&mut self, pid: ProcessID) {
        self.current_pid = pid;
    }

    fn is_address_valid(&self, addr: VirtualAddress) -> bool {
        // Check if address is in valid range for current process
        if self.kernel_mode {
            // Kernel can access all memory
            true
        } else {
            // User process can only access lower half of address space
            addr.0 < 0x8000_0000_0000_0000
        }
    }

    // Methods for protection features
    pub fn enable_nx(&mut self) {
        self.nx_enabled = true;
    }

    pub fn disable_nx(&mut self) {
        self.nx_enabled = false;
    }

    pub fn enable_write_protect(&mut self) {
        self.write_protect = true;
    }

    pub fn disable_write_protect(&mut self) {
        self.write_protect = false;
    }
}
