use super::common::{Point, Size, Color};
use crate::hardware::cpu::{CPU, Pipeline, ALU, Registers};

pub mod core;
pub mod cache;
pub mod pipeline;

pub struct CpuVisualizer {
    position: Point,
    size: Size,
    core_visualizer: core::CoreVisualizer,
    pipeline_visualizer: pipeline::PipelineVisualizer,
    cache_visualizer: cache::CacheVisualizer,
    temperature_map: TemperatureMap,
}

impl CpuVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        Self {
            position,
            size,
            core_visualizer: core::CoreVisualizer::new(),
            pipeline_visualizer: pipeline::PipelineVisualizer::new(),
            cache_visualizer: cache::CacheVisualizer::new(),
            temperature_map: TemperatureMap::new(),
        }
    }

    pub fn update(&mut self, cpu: &CPU) {
        self.core_visualizer.update(cpu);
        self.pipeline_visualizer.update(&cpu.pipeline);
        self.cache_visualizer.update(&cpu.cache);
        self.temperature_map.update(&cpu.temperature_sensors);
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw CPU die outline
        self.draw_die_outline(frame);
        
        // Render core components
        self.core_visualizer.render(frame);
        self.pipeline_visualizer.render(frame);
        self.cache_visualizer.render(frame);
        
        // Render temperature overlay
        self.temperature_map.render(frame);
        
        // Draw performance metrics
        self.draw_metrics(frame);
    }

    pub fn handle_interaction(&mut self, event: &InteractionEvent) -> bool {
        // Handle mouse hover/clicks on different CPU components
        if let Some(component) = self.get_component_at(event.position) {
            match event.kind {
                InteractionKind::Hover => {
                    self.show_component_details(component);
                }
                InteractionKind::Click => {
                    self.toggle_component_view(component);
                }
            }
            true
        } else {
            false
        }
    }
} 