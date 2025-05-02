use super::common::{blend_colors, ACTIVE_COLOR, INACTIVE_COLOR};

pub struct Window {
    pub title: String,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub minimized: bool,
    pub maximized: bool,
    pub focused: bool,
    content_type: WindowContent,
    drag_state: Option<DragState>,
}

enum WindowContent {
    ProcessManager,
    FileExplorer,
    Terminal,
    SystemMonitor,
}

struct DragState {
    start_x: usize,
    start_y: usize,
    offset_x: isize,
    offset_y: isize,
}

pub struct WindowManager {
    windows: Vec<Window>,
    active_window: Option<usize>,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            active_window: None,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw windows from back to front
        for (i, window) in self.windows.iter().enumerate() {
            if !window.minimized {
                self.draw_window(buffer, width, height, window, i == self.active_window.unwrap_or(999));
            }
        }
    }

    fn draw_window(&self, buffer: &mut Vec<u32>, width: usize, height: usize, window: &Window, active: bool) {
        // Draw window border
        let border_color = if active { ACTIVE_COLOR } else { INACTIVE_COLOR };
        
        // Draw title bar
        let title_height = 20;
        for y in window.y..window.y + title_height {
            for x in window.x..window.x + window.width {
                if y < height && x < width {
                    let pos = y * width + x;
                    buffer[pos] = border_color;
                }
            }
        }

        // Draw window content
        let content_y = window.y + title_height;
        for y in content_y..window.y + window.height {
            for x in window.x..window.x + window.width {
                if y < height && x < width {
                    let pos = y * width + x;
                    buffer[pos] = 0xFFFFFF; // Window background
                }
            }
        }

        // Draw window controls (minimize, maximize, close)
        self.draw_window_controls(buffer, width, window, active);
    }

    fn draw_window_controls(&self, buffer: &mut Vec<u32>, width: usize, window: &Window, active: bool) {
        let button_size = 16;
        let y = window.y + 2;
        
        // Close button
        let close_x = window.x + window.width - button_size - 2;
        for dy in 0..button_size {
            for dx in 0..button_size {
                let pos = (y + dy) * width + (close_x + dx);
                buffer[pos] = if active { 0xFF0000 } else { 0x800000 };
            }
        }

        // Maximize button
        let max_x = close_x - button_size - 2;
        for dy in 0..button_size {
            for dx in 0..button_size {
                let pos = (y + dy) * width + (max_x + dx);
                buffer[pos] = if active { 0x00FF00 } else { 0x008000 };
            }
        }

        // Minimize button
        let min_x = max_x - button_size - 2;
        for dy in 0..button_size {
            for dx in 0..button_size {
                let pos = (y + dy) * width + (min_x + dx);
                buffer[pos] = if active { 0x0000FF } else { 0x000080 };
            }
        }
    }

    pub fn handle_click(&mut self, x: f32, y: f32) -> bool {
        let x = x as usize;
        let y = y as usize;

        // Check window controls first (close, maximize, minimize)
        if let Some(window_idx) = self.get_window_at(x, y) {
            if self.handle_window_controls(x, y, window_idx) {
                return true;
            }

            // Make clicked window active
            self.active_window = Some(window_idx);
            
            // Start window drag if clicking title bar
            let window = &mut self.windows[window_idx];
            if y >= window.y && y < window.y + 20 {
                window.drag_state = Some(DragState {
                    start_x: x,
                    start_y: y,
                    offset_x: x as isize - window.x as isize,
                    offset_y: y as isize - window.y as isize,
                });
            }
            return true;
        }
        false
    }

    fn get_window_at(&self, x: usize, y: usize) -> Option<usize> {
        // Return index of topmost window containing point
        self.windows.iter().enumerate()
            .rev()
            .find(|(_, window)| {
                !window.minimized &&
                x >= window.x && x < window.x + window.width &&
                y >= window.y && y < window.y + window.height
            })
            .map(|(i, _)| i)
    }
} 