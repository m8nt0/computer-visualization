//! Visualization system for computer hardware simulation
//! This module provides visual representations of computer components
//! for educational purposes

use crate::src::hardware::{bus::Bus, cpu::CPU, gpu::GPU, memory::{MainMemory, Cache, DRAM}, power::Power};
use crate::src::software::{Process, ProcessState};
use computer::ComputerView;
use hardware::HardwareView;
use software::SoftwareView;
use super::hardware::{Hardware, HardwareStats};
use super::software::SoftwareState;

mod computer;
mod hardware;
mod software;
mod text;

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Computer,    // Physical laptop view
    Hardware,    // Internal components view
    Software,    // OS/Software view
}

pub struct VisualizationSystem {
    current_view: ViewMode,
    computer_view: ComputerView,
    hardware_view: HardwareView,
    software_view: SoftwareView,
    powered: bool,

    // References to actual hardware components
    cpu: *const CPU,
    gpu: *const GPU,
    memory: *const MainMemory,
    cache: *const Cache,
    dram: *const DRAM,
}

impl VisualizationSystem {
    pub fn new(
        cpu: *const CPU,
        gpu: *const GPU,
        memory: *const MainMemory,
        cache: *const Cache,
        dram: *const DRAM,
    ) -> Self {
        Self {
            current_view: ViewMode::Computer,
            computer_view: ComputerView::new(),
            hardware_view: HardwareView::new(cpu, gpu, memory, cache, dram),
            software_view: SoftwareView::new(),
            powered: false,
            cpu,
            gpu,
            memory,
            cache,
            dram,
        }
    }

    pub fn render(&mut self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        match self.current_view {
            ViewMode::Computer => {
                self.computer_view.render(buffer, width, height, self.powered);
            }
            ViewMode::Hardware => {
                self.hardware_view.render(buffer, width, height, self.powered);
            }
            ViewMode::Software => {
                self.software_view.render(buffer, width, height, self.powered);
            }
        }
    }

    pub fn switch_view(&mut self, mode: ViewMode) {
        self.current_view = mode;
    }

    pub fn toggle_power(&mut self) {
        self.powered = !self.powered;
    }

    pub fn handle_click(&mut self, x: f32, y: f32) {
        match self.current_view {
            ViewMode::Computer => self.computer_view.handle_click(x, y),
            ViewMode::Hardware => self.hardware_view.handle_click(x, y),
            ViewMode::Software => self.software_view.handle_click(x, y),
        }
    }

    pub fn get_component_info(&self) -> Vec<String> {
        // Get real-time info from actual hardware components
        let cpu = unsafe { &*self.cpu };
        let gpu = unsafe { &*self.gpu };
        let memory = unsafe { &*self.memory };

        match self.current_view {
            ViewMode::Computer => vec![
                format!("Power: {}", if self.powered { "ON" } else { "OFF" }),
                "Click to interact with components".to_string(),
            ],
            ViewMode::Hardware => vec![
                format!("CPU Status: Active"),
                format!("Memory Usage: {}%", self.get_memory_usage(memory)),
                format!("GPU Temperature: {}°C", self.get_gpu_temp(gpu)),
            ],
            ViewMode::Software => vec![
                format!("OS Status: Running"),
                format!("Active Processes: {}", self.get_process_count()),
            ],
        }
    }

    fn get_memory_usage(&self, memory: &MainMemory) -> u32 {
        // Calculate actual memory usage from hardware state
        0 // Placeholder
    }

    fn get_gpu_temp(&self, gpu: &GPU) -> u32 {
        // Get actual GPU temperature
        45 // Placeholder
    }

    fn get_process_count(&self) -> usize {
        // Get actual process count from OS
        2 // Placeholder
    }
}
