use super::common::{Point, Size, Color, Rect};
use crate::hardware::gpu::compute::{
    ComputeUnit,
    ShaderCore,
    RayCore,
    TensorCore
};

pub struct ComputeUnitsVisualizer {
    position: Point,
    size: Size,
    shader_cores: Vec<ShaderCoreView>,
    ray_cores: Vec<RayCoreView>,
    tensor_cores: Vec<TensorCoreView>,
    utilization: UtilizationVisualizer,
}

impl ComputeUnitsVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        Self {
            position,
            size,
            shader_cores: create_shader_core_views(position, size),
            ray_cores: create_ray_core_views(position, size),
            tensor_cores: create_tensor_core_views(position, size),
            utilization: UtilizationVisualizer::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw compute unit layout
        self.draw_compute_layout(frame);
        
        // Render cores
        for core in &self.shader_cores {
            core.render(frame);
        }
        for core in &self.ray_cores {
            core.render(frame);
        }
        for core in &self.tensor_cores {
            core.render(frame);
        }
        
        // Draw utilization heatmap
        self.utilization.render(frame);
        
        // Draw workload distribution
        self.draw_workload_distribution(frame);
        
        // Draw performance metrics
        self.draw_compute_metrics(frame);
    }

    fn draw_compute_layout(&self, frame: &mut Frame) {
        // Draw compute unit grid
        let grid = ComputeGrid::new(self.position, self.size);
        grid.draw(frame);
        
        // Draw interconnects between cores
        self.draw_core_interconnects(frame);
        
        // Draw cache hierarchy
        self.draw_cache_hierarchy(frame);
    }
} 