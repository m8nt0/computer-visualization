use crate::src::os::fs::{VirtualFileSystem, FileType};
use super::common::{blend_colors, ACTIVE_COLOR, INACTIVE_COLOR};

pub struct FileExplorer {
    current_path: String,
    selected_item: Option<String>,
    scroll_offset: usize,
    items: Vec<FileItem>,
    view_mode: ViewMode,
}

struct FileItem {
    name: String,
    file_type: FileType,
    size: usize,
    modified: u64,
    selected: bool,
}

enum ViewMode {
    List,
    Grid,
    Details,
}

impl FileExplorer {
    pub fn new() -> Self {
        Self {
            current_path: "/".to_string(),
            selected_item: None,
            scroll_offset: 0,
            items: Vec::new(),
            view_mode: ViewMode::Details,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw explorer layout
        self.draw_layout(buffer, width, height);
        
        // Draw navigation bar
        self.draw_navigation(buffer, width);
        
        // Draw file list/grid
        match self.view_mode {
            ViewMode::List => self.draw_list_view(buffer, width, height),
            ViewMode::Grid => self.draw_grid_view(buffer, width, height),
            ViewMode::Details => self.draw_details_view(buffer, width, height),
        }
        
        // Draw status bar
        self.draw_status_bar(buffer, width, height);
    }

    fn draw_layout(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw background
        for y in 0..height {
            for x in 0..width {
                let pos = y * width + x;
                buffer[pos] = 0xFFFFFF;
            }
        }

        // Draw borders
        for x in 0..width {
            let top = x;
            let bottom = (height - 1) * width + x;
            buffer[top] = 0xC0C0C0;
            buffer[bottom] = 0xC0C0C0;
        }

        for y in 0..height {
            let left = y * width;
            let right = y * width + width - 1;
            buffer[left] = 0xC0C0C0;
            buffer[right] = 0xC0C0C0;
        }
    }

    fn draw_navigation(&self, buffer: &mut Vec<u32>, width: usize) {
        // Draw path bar
        let nav_height = 30;
        for y in 0..nav_height {
            for x in 0..width {
                let pos = y * width + x;
                buffer[pos] = 0xE0E0E0;
            }
        }

        // Draw navigation buttons (back, forward, up)
        self.draw_nav_buttons(buffer, width);
        
        // Draw current path
        // ... text rendering implementation
    }

    fn draw_details_view(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        let start_y = 40;
        let item_height = 25;
        
        // Draw column headers
        self.draw_column_headers(buffer, width);

        // Draw items
        for (i, item) in self.items.iter().enumerate().skip(self.scroll_offset) {
            let y = start_y + (i - self.scroll_offset) * item_height;
            if y + item_height > height {
                break;
            }

            self.draw_file_item(buffer, width, y, item);
        }
    }

    fn draw_file_item(&self, buffer: &mut Vec<u32>, width: usize, y: usize, item: &FileItem) {
        let bg_color = if item.selected { ACTIVE_COLOR } else { 0xFFFFFF };
        
        // Draw item background
        for dy in 0..25 {
            for x in 0..width {
                let pos = (y + dy) * width + x;
                buffer[pos] = bg_color;
            }
        }

        // Draw icon
        let icon_color = match item.file_type {
            FileType::Directory => 0xFFD700,
            FileType::File => 0x4169E1,
            FileType::Symlink => 0x32CD32,
        };
        self.draw_file_icon(buffer, width, y + 4, icon_color);

        // Draw file information
        // ... text rendering implementation
    }

    pub fn update(&mut self, fs: &VirtualFileSystem) {
        // Update file listing from actual filesystem
        self.items = fs.read_dir(&self.current_path)
            .unwrap_or_default()
            .into_iter()
            .map(|entry| FileItem {
                name: entry.name,
                file_type: entry.file_type,
                size: entry.size,
                modified: entry.modified,
                selected: false,
            })
            .collect();
    }

    pub fn handle_click(&mut self, x: f32, y: f32) -> bool {
        let x = x as usize;
        let y = y as usize;

        // Handle navigation clicks
        if y < 30 {
            return self.handle_nav_click(x, y);
        }

        // Handle item clicks
        self.handle_item_click(x, y)
    }

    fn handle_nav_click(&mut self, x: usize, y: usize) -> bool {
        // Handle back button
        if x < 30 {
            self.navigate_back();
            return true;
        }

        // Handle forward button
        if x >= 30 && x < 60 {
            self.navigate_forward();
            return true;
        }

        // Handle up button
        if x >= 60 && x < 90 {
            self.navigate_up();
            return true;
        }

        false
    }

    fn handle_item_click(&mut self, x: usize, y: usize) -> bool {
        let item_index = self.get_item_at(x, y);
        if let Some(index) = item_index {
            if let Some(item) = self.items.get_mut(index) {
                item.selected = !item.selected;
                self.selected_item = Some(item.name.clone());
                return true;
            }
        }
        false
    }

    fn get_item_at(&self, x: usize, y: usize) -> Option<usize> {
        let start_y = 40;
        let item_height = 25;
        
        if y < start_y {
            return None;
        }

        let index = (y - start_y) / item_height + self.scroll_offset;
        if index < self.items.len() {
            Some(index)
        } else {
            None
        }
    }
}
