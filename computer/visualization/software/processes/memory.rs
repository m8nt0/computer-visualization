use crate::src::os::memory::{MemoryManager, MemoryBlock, MemoryState};
use super::common::{blend_colors, ACTIVE_COLOR, INACTIVE_COLOR};

pub struct MemoryVisualizer {
    memory_blocks: Vec<MemoryBlockView>,
    total_memory: usize,
    used_memory: usize,
    page_size: usize,
}

struct MemoryBlockView {
    address: usize,
    size: usize,
    state: MemoryState,
    process_id: Option<u32>,
    highlighted: bool,
}

impl MemoryVisualizer {
    pub fn new(total_memory: usize, page_size: usize) -> Self {
        Self {
            memory_blocks: Vec::new(),
            total_memory,
            used_memory: 0,
            page_size,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw memory map
        self.draw_memory_map(buffer, width, height);
        
        // Draw memory usage statistics
        self.draw_memory_stats(buffer, width, height);
        
        // Draw page tables
        self.draw_page_tables(buffer, width, height);
    }

    fn draw_memory_map(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        let block_height = 20;
        let mut y = 40;

        for block in &self.memory_blocks {
            let block_width = (block.size as f32 / self.total_memory as f32 * width as f32) as usize;
            
            // Draw memory block
            let color = self.get_block_color(block);
            for dy in 0..block_height {
                for dx in 0..block_width {
                    let pos = (y + dy) * width + dx;
                    if pos < buffer.len() {
                        buffer[pos] = color;
                    }
                }
            }

            // Draw block information
            // ... text rendering implementation

            y += block_height + 2;
        }
    }

    fn get_block_color(&self, block: &MemoryBlockView) -> u32 {
        match block.state {
            MemoryState::Free => 0x40FF40,
            MemoryState::Used => if block.highlighted { ACTIVE_COLOR } else { 0x4040FF },
            MemoryState::Reserved => 0xFF4040,
            MemoryState::Shared => 0xFFFF40,
        }
    }

    pub fn update(&mut self, memory_manager: &MemoryManager) {
        self.memory_blocks.clear();
        self.used_memory = 0;

        // Update memory blocks from actual memory manager state
        for block in memory_manager.get_memory_blocks() {
            if block.state != MemoryState::Free {
                self.used_memory += block.size;
            }

            self.memory_blocks.push(MemoryBlockView {
                address: block.address,
                size: block.size,
                state: block.state,
                process_id: block.process_id,
                highlighted: false,
            });
        }
    }
}
