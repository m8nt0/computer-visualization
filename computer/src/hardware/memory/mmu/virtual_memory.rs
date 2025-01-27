use super::super::error::{MemoryError, MemoryResult};
use super::super::types::{VirtualAddress, PhysicalAddress};
use super::paging::{PageTable, PageTableEntry, PageFlags};
use super::tlb::TLB;

pub struct VirtualMemoryManager {
    page_tables: Vec<PageTable>,
    current_asid: usize,  // Address Space ID
    tlb: TLB,
    enabled: bool,
    stats: VMStats,
}

struct VMStats {
    page_faults: u64,
    tlb_hits: u64,
    tlb_misses: u64,
    page_allocations: u64,
    page_deallocations: u64,
}

impl VirtualMemoryManager {
    pub fn new() -> Self {
        Self {
            page_tables: Vec::new(),
            current_asid: 0,
            tlb: TLB::new(),
            enabled: false,
            stats: VMStats::default(),
        }
    }

    pub fn translate(&mut self, virtual_addr: VirtualAddress) -> MemoryResult<PhysicalAddress> {
        if !self.enabled {
            return Ok(PhysicalAddress(virtual_addr.0));
        }

        // Check TLB first
        if let Some(physical_addr) = self.tlb.lookup(virtual_addr) {
            self.stats.tlb_hits += 1;
            return Ok(physical_addr);
        }
        self.stats.tlb_misses += 1;

        // Walk page table
        let page_table = self.get_current_page_table()?;
        match self.walk_page_table(page_table, virtual_addr) {
            Ok((physical_addr, entry)) => {
                // Update TLB
                self.tlb.insert(virtual_addr, physical_addr, entry.flags);
                Ok(physical_addr)
            }
            Err(e) => {
                self.stats.page_faults += 1;
                Err(e)
            }
        }
    }

    fn walk_page_table(&self, table: &PageTable, addr: VirtualAddress) 
        -> MemoryResult<(PhysicalAddress, &PageTableEntry)> 
    {
        let vpn = self.get_page_numbers(addr);
        let mut current_table = table;

        // Walk the page table levels
        for level in (0..4).rev() {
            let index = vpn[level];
            let entry = current_table.get_entry(index)?;

            if level == 0 {
                // Leaf entry - contains physical address
                if !entry.flags.present {
                    return Err(MemoryError::PageFault);
                }
                let physical_addr = self.get_physical_address(entry, addr);
                return Ok((physical_addr, entry));
            } else {
                // Non-leaf entry - points to next level table
                if let Some(next_table) = entry.get_next_table() {
                    current_table = next_table;
                } else {
                    return Err(MemoryError::PageFault);
                }
            }
        }
        Err(MemoryError::PageFault)
    }

    pub fn map_page(&mut self, virtual_addr: VirtualAddress, physical_addr: PhysicalAddress, 
                    flags: PageFlags) -> MemoryResult<()> 
    {
        let page_table = self.get_current_page_table()?;
        let vpn = self.get_page_numbers(virtual_addr);
        
        // Allocate page table entries as needed
        let mut current_table = page_table;
        for level in (1..4).rev() {
            let index = vpn[level];
            current_table = current_table.get_or_create_next_table(index)?;
        }

        // Map the actual page
        let entry = current_table.get_entry_mut(vpn[0])?;
        entry.set_physical_address(physical_addr);
        entry.flags = flags;
        entry.flags.present = true;

        // Invalidate TLB entry
        self.tlb.invalidate(virtual_addr);
        self.stats.page_allocations += 1;

        Ok(())
    }

    pub fn unmap_page(&mut self, virtual_addr: VirtualAddress) -> MemoryResult<()> {
        let page_table = self.get_current_page_table()?;
        let vpn = self.get_page_numbers(virtual_addr);
        
        // Find the leaf entry
        let mut current_table = page_table;
        for level in (1..4).rev() {
            let index = vpn[level];
            current_table = current_table.get_next_table(index)?;
        }

        // Unmap the page
        let entry = current_table.get_entry_mut(vpn[0])?;
        entry.flags.present = false;

        // Invalidate TLB entry
        self.tlb.invalidate(virtual_addr);
        self.stats.page_deallocations += 1;

        Ok(())
    }

    // Helper methods
    fn get_current_page_table(&self) -> MemoryResult<&PageTable> {
        self.page_tables.get(self.current_asid)
            .ok_or(MemoryError::PageFault)
    }

    fn get_page_numbers(&self, addr: VirtualAddress) -> [usize; 4] {
        let addr = addr.0;
        [
            ((addr >> 12) & 0x1FF) as usize,  // Level 0 - 9 bits
            ((addr >> 21) & 0x1FF) as usize,  // Level 1 - 9 bits
            ((addr >> 30) & 0x1FF) as usize,  // Level 2 - 9 bits
            ((addr >> 39) & 0x1FF) as usize,  // Level 3 - 9 bits
        ]
    }

    fn get_physical_address(&self, entry: &PageTableEntry, vaddr: VirtualAddress) -> PhysicalAddress {
        let offset = vaddr.0 & 0xFFF; // 12 bits offset
        PhysicalAddress(entry.get_physical_address().0 | offset)
    }
}
