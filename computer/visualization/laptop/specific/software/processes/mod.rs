use crate::src::os::{kernel::Kernel, process::{Process, ProcessState}};
use super::common::{DataFlow, blend_colors, ACTIVE_COLOR, INACTIVE_COLOR};

pub struct ProcessVisualizer {
    process_boxes: Vec<ProcessBox>,
    cpu_usage_history: Vec<Vec<f32>>,
    memory_usage_history: Vec<Vec<f32>>,
}

struct ProcessBox {
    pid: u32,
    name: String,
    state: ProcessState,
    position: (usize, usize),
    size: (usize, usize),
    cpu_usage: f32,
    memory_usage: f32,
}

impl ProcessVisualizer {
    pub fn new() -> Self {
        Self {
            process_boxes: Vec::new(),
            cpu_usage_history: Vec::new(),
            memory_usage_history: Vec::new(),
        }
    }

    pub fn render(&mut self, buffer: &mut Vec<u32>, width: usize, height: usize, kernel: &Kernel) {
        // Update process information from actual kernel state
        self.update_process_info(kernel);
        
        // Draw process list
        self.draw_process_list(buffer, width, height);
        
        // Draw CPU usage graph
        self.draw_cpu_graph(buffer, width, height);
        
        // Draw memory usage graph
        self.draw_memory_graph(buffer, width, height);
    }

    fn update_process_info(&mut self, kernel: &Kernel) {
        // Get actual process information from kernel
        let processes = kernel.get_processes();
        
        // Update or create process boxes
        self.process_boxes.clear();
        for (i, process) in processes.iter().enumerate() {
            let box_y = 50 + i * 30;
            self.process_boxes.push(ProcessBox {
                pid: process.id,
                name: process.name.clone(),
                state: process.state,
                position: (20, box_y),
                size: (200, 25),
                cpu_usage: process.cpu_usage,
                memory_usage: process.memory_usage,
            });
        }
    }

    fn draw_process_list(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw header
        self.draw_header(buffer, width);
        
        // Draw each process box
        for process in &self.process_boxes {
            self.draw_process_box(buffer, width, process);
        }
    }

    fn draw_header(&self, buffer: &mut Vec<u32>, width: usize) {
        // Draw column headers
        let headers = ["PID", "Name", "State", "CPU%", "Memory%"];
        let mut x = 20;
        for header in headers {
            // Draw header text
            // ... implementation
            x += 100;
        }
    }

    fn draw_process_box(&self, buffer: &mut Vec<u32>, width: usize, process: &ProcessBox) {
        let (x, y) = process.position;
        let (w, h) = process.size;
        
        // Draw box background based on process state
        let bg_color = match process.state {
            ProcessState::Running => 0x00FF00,
            ProcessState::Ready => 0xFFFF00,
            ProcessState::Blocked => 0xFF0000,
            ProcessState::Terminated => 0x808080,
        };

        // Draw box
        for dy in 0..h {
            for dx in 0..w {
                let pos = (y + dy) * width + (x + dx);
                if pos < buffer.len() {
                    buffer[pos] = bg_color;
                }
            }
        }

        // Draw process information
        // ... draw PID, name, state, CPU%, Memory%
    }

    fn draw_cpu_graph(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw CPU usage history graph
        let graph_x = width - 220;
        let graph_y = 50;
        let graph_width = 200;
        let graph_height = 100;

        // Draw graph background
        for y in graph_y..graph_y + graph_height {
            for x in graph_x..graph_x + graph_width {
                let pos = y * width + x;
                if pos < buffer.len() {
                    buffer[pos] = 0x202020;
                }
            }
        }

        // Draw CPU usage lines
        // ... implementation
    }

    fn draw_memory_graph(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Similar to CPU graph but for memory usage
        // ... implementation
    }
}
