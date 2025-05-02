use super::common::{Point, Size, Color, Rect};
use crate::hardware::memory::cache::{
    L1Cache,
    L2Cache, 
    L3Cache,
    CacheLine,
    CacheStats,
    ReplacementPolicy,
    CoherencyState
};

pub struct CacheHierarchyVisualizer {
    position: Point,
    size: Size,
    l1_cache: L1CacheVisualizer,
    l2_cache: L2CacheVisualizer,
    l3_cache: L3CacheVisualizer,
    coherency: CoherencyVisualizer,
    prefetcher: PrefetcherVisualizer,
}

impl CacheHierarchyVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        let layout = CacheHierarchyLayout::new(position, size);
        
        Self {
            position,
            size,
            l1_cache: L1CacheVisualizer::new(layout.l1_region),
            l2_cache: L2CacheVisualizer::new(layout.l2_region),
            l3_cache: L3CacheVisualizer::new(layout.l3_region),
            coherency: CoherencyVisualizer::new(layout.coherency_region),
            prefetcher: PrefetcherVisualizer::new(layout.prefetcher_region),
        }
    }

    pub fn update(&mut self, l1: &L1Cache, l2: &L2Cache, l3: &L3Cache) {
        self.l1_cache.update(l1);
        self.l2_cache.update(l2);
        self.l3_cache.update(l3);
        
        // Update coherency state
        self.coherency.update_states(l1, l2, l3);
        
        // Update prefetcher
        self.prefetcher.update(l1.prefetcher(), l2.prefetcher(), l3.prefetcher());
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw cache hierarchy structure
        self.draw_hierarchy_structure(frame);
        
        // Render individual caches
        self.l1_cache.render(frame);
        self.l2_cache.render(frame);
        self.l3_cache.render(frame);
        
        // Render coherency state
        self.coherency.render(frame);
        
        // Render prefetcher
        self.prefetcher.render(frame);
        
        // Draw data flow animations
        self.render_data_flows(frame);
        
        // Draw performance metrics
        self.draw_cache_metrics(frame);
    }

    fn render_data_flows(&self, frame: &mut Frame) {
        // Draw cache line transfers between levels
        for transfer in &self.data_flows {
            let path = match transfer.direction {
                CacheDirection::L1ToL2 => self.get_l1_to_l2_path(),
                CacheDirection::L2ToL3 => self.get_l2_to_l3_path(),
                CacheDirection::L3ToMemory => self.get_l3_to_memory_path(),
            };
            
            frame.draw_animated_path(path, transfer.progress, self.colors.data_flow);
        }
    }
} 