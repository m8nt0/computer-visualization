use super::common::{Point, Size, Color, Rect};

// pub mod bus;
// pub mod cache;
// pub mod clock;
// pub mod cpu;
// pub mod dma;
// pub mod gpu;
// pub mod interrupt;
// pub mod io;
// pub mod memory;
// pub mod mmu;
// pub mod power;
// pub mod storage;

pub mod cooling;
pub mod memory;
pub mod peripherals;
pub mod power;
pub mod processing;
pub mod storage;
pub mod firmware;

pub struct HardwareVisualizer {
    // Main components
    cpu_region: Rect,
    gpu_region: Rect,
    memory_region: Rect,
    storage_region: Rect,
    
    // Interconnects
    bus_visualizer: bus::BusVisualizer,
    
    // Core visualizers
    cpu_visualizer: cpu::CpuVisualizer,
    gpu_visualizer: gpu::GpuVisualizer,
    memory_visualizer: memory::MemoryVisualizer,
    storage_visualizer: storage::StorageVisualizer,
    
    // Support systems
    power_visualizer: power::PowerVisualizer,
    clock_visualizer: clock::ClockVisualizer,
    interrupt_visualizer: interrupt::InterruptVisualizer,
    
    // Current view state
    zoom_level: f32,
    focused_component: Option<ComponentId>,
}

impl HardwareVisualizer {
    pub fn new(viewport_size: Size) -> Self {
        // Layout the main component regions
        let layout = ComponentLayout::new(viewport_size);
        
        Self {
            cpu_region: layout.cpu_region,
            gpu_region: layout.gpu_region,
            memory_region: layout.memory_region,
            storage_region: layout.storage_region,
            
            bus_visualizer: bus::BusVisualizer::new(&layout),
            cpu_visualizer: cpu::CpuVisualizer::new(layout.cpu_region),
            gpu_visualizer: gpu::GpuVisualizer::new(layout.gpu_region),
            memory_visualizer: memory::MemoryVisualizer::new(layout.memory_region),
            storage_visualizer: storage::StorageVisualizer::new(layout.storage_region),
            
            power_visualizer: power::PowerVisualizer::new(&layout),
            clock_visualizer: clock::ClockVisualizer::new(&layout),
            interrupt_visualizer: interrupt::InterruptVisualizer::new(&layout),
            
            zoom_level: 1.0,
            focused_component: None,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw background and system outline
        self.draw_background(frame);
        
        // Draw interconnects (buses)
        self.bus_visualizer.render(frame);
        
        // Draw main components
        self.cpu_visualizer.render(frame);
        self.gpu_visualizer.render(frame);
        self.memory_visualizer.render(frame);
        self.storage_visualizer.render(frame);
        
        // Draw support systems
        self.power_visualizer.render(frame);
        self.clock_visualizer.render(frame);
        self.interrupt_visualizer.render(frame);
        
        // Draw data flow animations
        self.render_data_flows(frame);
        
        // Draw focused component overlay
        if let Some(component) = self.focused_component {
            self.render_focused_component(component, frame);
        }
    }

    pub fn handle_interaction(&mut self, event: &InteractionEvent) -> bool {
        match event.kind {
            InteractionKind::Zoom(delta) => {
                self.handle_zoom(delta);
                true
            }
            InteractionKind::Click(position) => {
                if let Some(component) = self.get_component_at(position) {
                    self.focused_component = Some(component);
                    true
                } else {
                    false
                }
            }
            InteractionKind::Hover(position) => {
                self.handle_hover(position);
                true
            }
        }
    }

    fn get_component_at(&self, position: Point) -> Option<ComponentId> {
        // Check each region to find which component was clicked
        if self.cpu_region.contains(position) {
            Some(ComponentId::Cpu)
        } else if self.gpu_region.contains(position) {
            Some(ComponentId::Gpu)
        } else if self.memory_region.contains(position) {
            Some(ComponentId::Memory)
        } else if self.storage_region.contains(position) {
            Some(ComponentId::Storage)
        } else {
            None
        }
    }
} 