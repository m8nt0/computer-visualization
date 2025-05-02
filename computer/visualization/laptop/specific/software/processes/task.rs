use crate::src::os::process::{Process, ProcessState};
use super::common::{blend_colors, ACTIVE_COLOR, INACTIVE_COLOR};

pub struct TaskManager {
    processes: Vec<ProcessInfo>,
    selected_process: Option<usize>,
    sort_column: SortColumn,
    sort_ascending: bool,
}

struct ProcessInfo {
    pid: u32,
    name: String,
    state: ProcessState,
    cpu_usage: f32,
    memory_usage: usize,
    threads: usize,
    selected: bool,
}

enum SortColumn {
    Name,
    CPU,
    Memory,
    PID,
    Status,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            selected_process: None,
            sort_column: SortColumn::CPU,
            sort_ascending: false,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw task manager layout
        self.draw_layout(buffer, width, height);
        
        // Draw process list
        self.draw_process_list(buffer, width, height);
        
        // Draw performance graphs
        self.draw_performance_graphs(buffer, width, height);
        
        // Draw system information
        self.draw_system_info(buffer, width, height);
    }

    fn draw_layout(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw background
        for y in 0..height {
            for x in 0..width {
                let pos = y * width + x;
                buffer[pos] = 0xFFFFFF;
            }
        }

        // Draw borders and sections
        self.draw_borders(buffer, width, height);
    }

    fn draw_process_list(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw column headers
        self.draw_column_headers(buffer, width);

        // Draw processes
        let start_y = 30;
        let row_height = 25;

        for (i, process) in self.processes.iter().enumerate() {
            let y = start_y + i * row_height;
            if y + row_height > height {
                break;
            }

            self.draw_process_row(buffer, width, y, process);
        }
    }

    fn draw_process_row(&self, buffer: &mut Vec<u32>, width: usize, y: usize, process: &ProcessInfo) {
        let bg_color = if process.selected { ACTIVE_COLOR } else { 0xFFFFFF };
        
        // Draw row background
        for dy in 0..25 {
            for x in 0..width {
                let pos = (y + dy) * width + x;
                if pos < buffer.len() {
                    buffer[pos] = bg_color;
                }
            }
        }

        // Draw process information
        // ... text rendering implementation
    }

    fn draw_performance_graphs(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw CPU usage graph
        self.draw_cpu_graph(buffer, width, height);
        
        // Draw memory usage graph
        self.draw_memory_graph(buffer, width, height);
        
        // Draw disk usage graph
        self.draw_disk_graph(buffer, width, height);
    }

    pub fn update(&mut self, processes: &[Process]) {
        self.processes = processes.iter()
            .map(|p| ProcessInfo {
                pid: p.id,
                name: p.name.clone(),
                state: p.state,
                cpu_usage: p.cpu_usage,
                memory_usage: p.memory_usage,
                threads: p.threads.len(),
                selected: false,
            })
            .collect();

        // Sort processes based on current sort column
        self.sort_processes();
    }

    fn sort_processes(&mut self) {
        self.processes.sort_by(|a, b| {
            let cmp = match self.sort_column {
                SortColumn::Name => a.name.cmp(&b.name),
                SortColumn::CPU => a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap(),
                SortColumn::Memory => a.memory_usage.cmp(&b.memory_usage),
                SortColumn::PID => a.pid.cmp(&b.pid),
                SortColumn::Status => a.state.cmp(&b.state),
            };
            if self.sort_ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });
    }

    pub fn handle_click(&mut self, x: f32, y: f32) -> bool {
        let x = x as usize;
        let y = y as usize;

        // Handle column header clicks
        if y < 30 {
            return self.handle_header_click(x);
        }

        // Handle process row clicks
        self.handle_process_click(x, y)
    }

    fn handle_header_click(&mut self, x: usize) -> bool {
        let new_sort = match x {
            0..=100 => Some(SortColumn::Name),
            101..=200 => Some(SortColumn::CPU),
            201..=300 => Some(SortColumn::Memory),
            301..=400 => Some(SortColumn::PID),
            401..=500 => Some(SortColumn::Status),
            _ => None,
        };

        if let Some(sort) = new_sort {
            if self.sort_column == sort {
                self.sort_ascending = !self.sort_ascending;
            } else {
                self.sort_column = sort;
                self.sort_ascending = true;
            }
            self.sort_processes();
            return true;
        }
        false
    }
}
