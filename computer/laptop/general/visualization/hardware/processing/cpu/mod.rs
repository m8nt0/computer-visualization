use super::super::super::src::hardware::visualization::{HardwareVisualizer, HardwareComponent};
use super::super::super::src::hardware::processing::cpu::CPU;
use crate::common::{Point, Size, Color};

pub mod alu;
pub mod cache;
pub mod core;
pub mod pipeline;

pub struct CpuVisualizer {
    position: Point,
    size: Size,
    components: CpuComponents,
    colors: CpuColors,
}

struct CpuComponents {
    alu: alu::AluVisualizer,
    cache: cache::CacheVisualizer,
    core: core::CoreVisualizer,
    pipeline: pipeline::PipelineVisualizer,
}

struct CpuColors {
    background: Color,
    active: Color,
    inactive: Color,
    highlight: Color,
    error: Color,
}

impl CpuVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        let colors = CpuColors {
            background: Color::new(0.2, 0.2, 0.2, 1.0),
            active: Color::new(0.0, 1.0, 0.0, 1.0),
            inactive: Color::new(0.5, 0.5, 0.5, 1.0),
            highlight: Color::new(0.0, 0.7, 1.0, 1.0),
            error: Color::new(1.0, 0.0, 0.0, 1.0),
        };

        // Calculate component positions and sizes
        let alu_size = Size::new(size.width * 0.3, size.height * 0.4);
        let cache_size = Size::new(size.width * 0.2, size.height * 0.3);
        let core_size = Size::new(size.width * 0.4, size.height * 0.5);
        let pipeline_size = Size::new(size.width * 0.9, size.height * 0.2);

        let components = CpuComponents {
            alu: alu::AluVisualizer::new(
                Point::new(position.x + size.width * 0.1, position.y + size.height * 0.1),
                alu_size
            ),
            cache: cache::CacheVisualizer::new(
                Point::new(position.x + size.width * 0.6, position.y + size.height * 0.1),
                cache_size
            ),
            core: core::CoreVisualizer::new(
                Point::new(position.x + size.width * 0.3, position.y + size.height * 0.3),
                core_size
            ),
            pipeline: pipeline::PipelineVisualizer::new(
                Point::new(position.x + size.width * 0.05, position.y + size.height * 0.7),
                pipeline_size
            ),
        };

        Self {
            position,
            size,
            components,
            colors,
        }
    }

    fn draw_background(&self) {
        let rect = crate::common::Rect::new(self.position, self.size);
        rect.fill(self.colors.background);
    }

    fn draw_connections(&self) {
        // Draw connections between components
        // This would be implemented based on the actual CPU architecture
    }
}

impl HardwareVisualizer for CpuVisualizer {
    fn update(&mut self, component: &dyn HardwareComponent) {
        if let Some(cpu) = component.as_any().downcast_ref::<CPU>() {
            // Update all CPU components
            self.components.alu.update(cpu.get_alu());
            self.components.cache.update(cpu.get_cache());
            self.components.core.update(cpu.get_core());
            self.components.pipeline.update(cpu.get_pipeline());
        }
    }

    fn render(&self) {
        // Draw CPU background
        self.draw_background();
        
        // Draw connections between components
        self.draw_connections();
        
        // Render all components
        self.components.alu.render();
        self.components.cache.render();
        self.components.core.render();
        self.components.pipeline.render();
    }
} 