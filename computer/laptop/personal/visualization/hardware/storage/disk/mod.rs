use super::common::{Point, Size, Color, Rect};
use crate::hardware::storage::disk::{DiskController, Platter, Head};

pub struct DiskVisualizer {
    position: Point,
    size: Size,
    platters: Vec<PlatterView>,
    heads: Vec<HeadView>,
    controller: DiskControllerView,
    cache: DiskCacheView,
}

impl DiskVisualizer {
    pub fn render(&self, frame: &mut Frame) {
        // Draw disk structure
        self.draw_disk_structure(frame);
        
        // Render platters
        for platter in &self.platters {
            platter.render(frame);
        }
        
        // Render heads
        for head in &self.heads {
            head.render(frame);
        }
        
        // Render controller
        self.controller.render(frame);
        
        // Render cache
        self.cache.render(frame);
        
        // Draw access patterns
        self.draw_access_patterns(frame);
        
        // Draw performance metrics
        self.draw_disk_metrics(frame);
    }
} 