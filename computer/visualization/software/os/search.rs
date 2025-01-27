use crate::src::os::{process::Process, fs::VirtualFileSystem};
use super::common::{blend_colors, ACTIVE_COLOR, INACTIVE_COLOR};

pub struct SearchBar {
    query: String,
    results: Vec<SearchResult>,
    active_result: Option<usize>,
    cursor_position: usize,
    cursor_blink: bool,
}

struct SearchResult {
    name: String,
    result_type: ResultType,
    path: String,
    relevance: f32,
}

enum ResultType {
    Program,
    File,
    Setting,
    Command,
}

impl SearchBar {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
            active_result: None,
            cursor_position: 0,
            cursor_blink: false,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw search box
        self.draw_search_box(buffer, width);
        
        // Draw search results if any
        if !self.results.is_empty() {
            self.draw_results(buffer, width);
        }
    }

    fn draw_search_box(&self, buffer: &mut Vec<u32>, width: usize) {
        let box_width = 280;
        let box_height = 30;
        let x = 10;
        let y = 10;

        // Draw box background
        for dy in 0..box_height {
            for dx in 0..box_width {
                let pos = (y + dy) * width + (x + dx);
                buffer[pos] = 0xFFFFFF;
            }
        }

        // Draw search text
        // ... text rendering implementation

        // Draw cursor if active
        if self.cursor_blink {
            let cursor_x = x + 8 + self.cursor_position * 8;
            for dy in 5..25 {
                let pos = (y + dy) * width + cursor_x;
                buffer[pos] = 0x000000;
            }
        }
    }

    fn draw_results(&self, buffer: &mut Vec<u32>, width: usize) {
        let start_y = 50;
        let result_height = 30;

        for (i, result) in self.results.iter().enumerate() {
            let y = start_y + i * result_height;
            let is_active = Some(i) == self.active_result;
            
            // Draw result background
            let bg_color = if is_active { ACTIVE_COLOR } else { 0x303030 };
            for dy in 0..result_height {
                for dx in 0..280 {
                    let pos = (y + dy) * width + (10 + dx);
                    buffer[pos] = bg_color;
                }
            }

            // Draw result icon based on type
            let icon_color = match result.result_type {
                ResultType::Program => 0x40FF40,
                ResultType::File => 0xFFFF40,
                ResultType::Setting => 0x4040FF,
                ResultType::Command => 0xFF4040,
            };
            self.draw_result_icon(buffer, width, 15, y + 7, icon_color);

            // Draw result text
            // ... text rendering implementation
        }
    }

    pub fn update_search(&mut self, fs: &VirtualFileSystem, processes: &[Process]) {
        // Clear old results
        self.results.clear();

        if self.query.is_empty() {
            return;
        }

        // Search through programs
        self.search_programs(processes);
        
        // Search through files
        self.search_files(fs);
        
        // Search through settings
        self.search_settings();

        // Sort results by relevance
        self.results.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
    }

    fn search_programs(&mut self, processes: &[Process]) {
        for process in processes {
            if process.name.to_lowercase().contains(&self.query.to_lowercase()) {
                self.results.push(SearchResult {
                    name: process.name.clone(),
                    result_type: ResultType::Program,
                    path: String::new(),
                    relevance: self.calculate_relevance(&process.name),
                });
            }
        }
    }

    fn search_files(&mut self, fs: &VirtualFileSystem) {
        // Search through filesystem
        // ... implementation
    }

    fn search_settings(&mut self) {
        // Search through system settings
        // ... implementation
    }

    fn calculate_relevance(&self, text: &str) -> f32 {
        let query = self.query.to_lowercase();
        let text = text.to_lowercase();

        if text == query {
            return 1.0;
        }

        if text.starts_with(&query) {
            return 0.8;
        }

        if text.contains(&query) {
            return 0.5;
        }

        0.0
    }

    pub fn handle_input(&mut self, c: char) {
        match c {
            '\x08' => { // Backspace
                if self.cursor_position > 0 {
                    self.query.remove(self.cursor_position - 1);
                    self.cursor_position -= 1;
                }
            }
            '\r' => { // Enter
                if let Some(active) = self.active_result {
                    self.execute_result(&self.results[active]);
                }
            }
            c if c.is_ascii_graphic() => {
                self.query.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
            _ => {}
        }
    }

    fn execute_result(&mut self, result: &SearchResult) {
        match result.result_type {
            ResultType::Program => {
                // Launch program
            }
            ResultType::File => {
                // Open file
            }
            ResultType::Setting => {
                // Open setting
            }
            ResultType::Command => {
                // Execute command
            }
        }
    }
} 