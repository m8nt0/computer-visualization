use super::super::error::{MemoryError, MemoryResult};
use super::super::types::{PhysicalAddress, VirtualAddress};
use std::collections::HashMap;

pub struct PageTable {
    entries: Vec<PageTableEntry>,
    level: usize,
    next_tables: HashMap<usize, Box<PageTable>>,
    stats: PageTableStats,
}

#[derive(Clone)]
pub struct PageTableEntry {
    pub physical_frame: Option<u64>,
    pub flags: PageFlags,
    pub accessed: bool,
    pub dirty: bool,
}

#[derive(Clone, Copy)]
pub struct PageFlags {
    pub present: bool,
    pub writable: bool,
    pub user_accessible: bool,
    pub write_through: bool,
    pub cache_disabled: bool,
    pub executable: bool,
    pub global: bool,
}

struct PageTableStats {
    page_faults: u64,
    page_walks: u64,
    allocations: u64,
    deallocations: u64,
}

impl PageTable {
    pub fn new(level: usize) -> Self {
        Self {
            entries: vec![PageTableEntry::new(); 512], // 512 entries per table
            level,
            next_tables: HashMap::new(),
            stats: PageTableStats::default(),
        }
    }

    pub fn get_entry(&self, index: usize) -> MemoryResult<&PageTableEntry> {
        self.entries.get(index)
            .ok_or(MemoryError::PageFault)
    }

    pub fn get_entry_mut(&mut self, index: usize) -> MemoryResult<&mut PageTableEntry> {
        self.entries.get_mut(index)
            .ok_or(MemoryError::PageFault)
    }

    pub fn get_next_table(&self, index: usize) -> MemoryResult<&PageTable> {
        self.next_tables.get(&index)
            .map(|table| table.as_ref())
            .ok_or(MemoryError::PageFault)
    }

    pub fn get_or_create_next_table(&mut self, index: usize) -> MemoryResult<&mut PageTable> {
        if !self.next_tables.contains_key(&index) {
            let entry = self.get_entry_mut(index)?;
            if entry.physical_frame.is_none() {
                // Allocate new physical frame for table
                entry.physical_frame = Some(self.allocate_frame()?);
            }
            
            let new_table = Box::new(PageTable::new(self.level - 1));
            self.next_tables.insert(index, new_table);
            self.stats.allocations += 1;
        }
        
        Ok(self.next_tables.get_mut(&index).unwrap())
    }

    fn allocate_frame(&self) -> MemoryResult<u64> {
        // Frame allocation would be handled by physical memory manager
        Ok(0) // Placeholder
    }

    pub fn map(&mut self, virtual_addr: VirtualAddress, physical_addr: PhysicalAddress, 
               flags: PageFlags) -> MemoryResult<()> 
    {
        let indices = self.get_page_indices(virtual_addr);
        let mut current_table = self;
        
        // Walk/create page tables
        for &index in &indices[1..] {
            current_table = current_table.get_or_create_next_table(index)?;
        }

        // Map the actual page
        let entry = current_table.get_entry_mut(indices[0])?;
        entry.physical_frame = Some(physical_addr.0);
        entry.flags = flags;
        entry.flags.present = true;

        Ok(())
    }

    pub fn unmap(&mut self, virtual_addr: VirtualAddress) -> MemoryResult<()> {
        let indices = self.get_page_indices(virtual_addr);
        let mut current_table = self;
        
        // Walk to leaf entry
        for &index in &indices[1..] {
            current_table = current_table.get_next_table(index)?;
        }

        // Unmap the page
        let entry = current_table.get_entry_mut(indices[0])?;
        entry.physical_frame = None;
        entry.flags.present = false;
        self.stats.deallocations += 1;

        Ok(())
    }

    fn get_page_indices(&self, addr: VirtualAddress) -> [usize; 4] {
        let addr = addr.0;
        [
            ((addr >> 12) & 0x1FF) as usize,  // Page offset
            ((addr >> 21) & 0x1FF) as usize,  // PT index
            ((addr >> 30) & 0x1FF) as usize,  // PD index
            ((addr >> 39) & 0x1FF) as usize,  // PDP index
        ]
    }
}

impl PageTableEntry {
    pub fn new() -> Self {
        Self {
            physical_frame: None,
            flags: PageFlags::new(),
            accessed: false,
            dirty: false,
        }
    }

    pub fn get_physical_address(&self) -> PhysicalAddress {
        PhysicalAddress(self.physical_frame.unwrap_or(0))
    }

    pub fn set_physical_address(&mut self, addr: PhysicalAddress) {
        self.physical_frame = Some(addr.0);
    }

    pub fn get_next_table(&self) -> Option<&PageTable> {
        // Implementation would access the actual table
        None // Placeholder
    }
}

impl PageFlags {
    pub fn new() -> Self {
        Self {
            present: false,
            writable: false,
            user_accessible: false,
            write_through: false,
            cache_disabled: false,
            executable: true,
            global: false,
        }
    }

    pub fn kernel_code() -> Self {
        Self {
            present: true,
            writable: false,
            user_accessible: false,
            write_through: false,
            cache_disabled: false,
            executable: true,
            global: true,
        }
    }

    pub fn kernel_data() -> Self {
        Self {
            present: true,
            writable: true,
            user_accessible: false,
            write_through: false,
            cache_disabled: false,
            executable: false,
            global: true,
        }
    }

    pub fn user_code() -> Self {
        Self {
            present: true,
            writable: false,
            user_accessible: true,
            write_through: false,
            cache_disabled: false,
            executable: true,
            global: false,
        }
    }

    pub fn user_data() -> Self {
        Self {
            present: true,
            writable: true,
            user_accessible: true,
            write_through: false,
            cache_disabled: false,
            executable: false,
            global: false,
        }
    }
}
