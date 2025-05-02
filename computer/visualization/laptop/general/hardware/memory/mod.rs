use super::common::{Point, Size, Color, Rect};

pub mod cache;
pub mod controller;
pub mod dram;
pub mod mmu;

pub struct MemoryVisualizer {
    position: Point,
    size: Size,
    
    // Memory hierarchy components
    dram_visualizer: dram::DramVisualizer,
    cache_hierarchy: cache::CacheHierarchyVisualizer,
    controller_visualizer: controller::MemoryControllerVisualizer,
    mmu_visualizer: mmu::MmuVisualizer,
    
    // Animation state
    memory_accesses: Vec<MemoryAccessAnimation>,
    page_faults: Vec<PageFaultAnimation>,
}

impl MemoryVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        let layout = MemoryLayout::new(position, size);
        
        Self {
            position,
            size,
            dram_visualizer: dram::DramVisualizer::new(layout.dram_region),
            cache_hierarchy: cache::CacheHierarchyVisualizer::new(layout.cache_region),
            controller_visualizer: controller::MemoryControllerVisualizer::new(layout.controller_region),
            mmu_visualizer: mmu::MmuVisualizer::new(layout.mmu_region),
            memory_accesses: Vec::new(),
            page_faults: Vec::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw memory system overview
        self.draw_memory_overview(frame);
        
        // Render individual components
        self.dram_visualizer.render(frame);
        self.cache_hierarchy.render(frame);
        self.controller_visualizer.render(frame);
        self.mmu_visualizer.render(frame);
        
        // Render memory access animations
        for access in &self.memory_accesses {
            access.render(frame);
        }
        
        // Render page fault animations
        for fault in &self.page_faults {
            fault.render(frame);
        }
        
        // Draw performance metrics
        self.draw_memory_metrics(frame);
    }
} 