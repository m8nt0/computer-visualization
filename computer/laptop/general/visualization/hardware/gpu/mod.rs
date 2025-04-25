use super::common::{Point, Size, Color, Rect};
use crate::hardware::gpu::{GPU, CommandProcessor, ComputeUnit, DisplayController};

pub mod compute;
pub mod memory;
pub mod display;
pub mod command;
pub mod scheduler;

pub struct GpuVisualizer {
    position: Point,
    size: Size,
    
    // Core components
    compute_units: compute::ComputeUnitsVisualizer,
    memory_system: memory::GpuMemoryVisualizer,
    display_controller: display::DisplayVisualizer,
    command_processor: command::CommandProcessorVisualizer,
    scheduler: scheduler::SchedulerVisualizer,
    
    // Animation state
    workload_flows: Vec<WorkloadAnimation>,
    memory_transfers: Vec<MemoryTransferAnimation>,
}

impl GpuVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        let layout = GpuLayout::new(position, size);
        
        Self {
            position,
            size,
            compute_units: compute::ComputeUnitsVisualizer::new(layout.compute_region),
            memory_system: memory::GpuMemoryVisualizer::new(layout.memory_region),
            display_controller: display::DisplayVisualizer::new(layout.display_region),
            command_processor: command::CommandProcessorVisualizer::new(layout.command_region),
            scheduler: scheduler::SchedulerVisualizer::new(layout.scheduler_region),
            workload_flows: Vec::new(),
            memory_transfers: Vec::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw GPU die outline
        self.draw_gpu_outline(frame);
        
        // Render core components
        self.compute_units.render(frame);
        self.memory_system.render(frame);
        self.display_controller.render(frame);
        self.command_processor.render(frame);
        self.scheduler.render(frame);
        
        // Render workload flows
        for flow in &self.workload_flows {
            flow.render(frame);
        }
        
        // Render memory transfers
        for transfer in &self.memory_transfers {
            transfer.render(frame);
        }
        
        // Draw performance metrics
        self.draw_gpu_metrics(frame);
    }
} 