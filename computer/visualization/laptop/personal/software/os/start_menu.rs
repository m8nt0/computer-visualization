use crate::src::os::process::Process;
use super::common::{blend_colors, ACTIVE_COLOR, INACTIVE_COLOR};

pub struct StartMenu {
    visible: bool,
    items: Vec<MenuItem>,
    recent_programs: Vec<RecentProgram>,
    power_options: Vec<PowerOption>,
    hover_item: Option<usize>,
}

struct MenuItem {
    name: String,
    icon: MenuIcon,
    action: MenuAction,
    position: (usize, usize),
    size: (usize, usize),
}

struct RecentProgram {
    name: String,
    icon: MenuIcon,
    last_used: u64,
}

#[derive(Clone, Copy)]
enum MenuIcon {
    Programs,
    Documents,
    Settings,
    Search,
    Power,
    User,
}

enum MenuAction {
    OpenPrograms,
    OpenDocuments,
    OpenSettings,
    StartSearch,
    ShowPowerOptions,
    ShowUserProfile,
    LaunchProgram(String),
}

enum PowerOption {
    Sleep,
    Restart,
    Shutdown,
}

impl StartMenu {
    pub fn new() -> Self {
        Self {
            visible: false,
            items: Self::create_default_items(),
            recent_programs: Vec::new(),
            power_options: vec![
                PowerOption::Sleep,
                PowerOption::Restart,
                PowerOption::Shutdown,
            ],
            hover_item: None,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        if !self.visible {
            return;
        }

        // Draw menu background
        self.draw_background(buffer, width, height);
        
        // Draw menu items
        self.draw_items(buffer, width);
        
        // Draw recent programs
        self.draw_recent_programs(buffer, width);
        
        // Draw power options
        self.draw_power_options(buffer, width);
        
        // Draw search bar
        self.draw_search_bar(buffer, width);
    }

    fn draw_background(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        let menu_width = 300;
        let menu_height = 400;
        let start_x = 0;
        let start_y = height - menu_height;

        // Draw semi-transparent background
        for y in start_y..height {
            for x in start_x..start_x + menu_width {
                let pos = y * width + x;
                if pos < buffer.len() {
                    buffer[pos] = blend_colors(buffer[pos], 0x202020, 0.9);
                }
            }
        }

        // Draw border
        for y in start_y..height {
            let border_pos = y * width + (start_x + menu_width - 1);
            if border_pos < buffer.len() {
                buffer[border_pos] = 0x404040;
            }
        }
    }

    fn draw_items(&self, buffer: &mut Vec<u32>, width: usize) {
        for (i, item) in self.items.iter().enumerate() {
            let is_hovered = self.hover_item == Some(i);
            self.draw_menu_item(buffer, width, item, is_hovered);
        }
    }

    fn draw_menu_item(&self, buffer: &mut Vec<u32>, width: usize, item: &MenuItem, hovered: bool) {
        let (x, y) = item.position;
        let (w, h) = item.size;

        // Draw item background
        let bg_color = if hovered { ACTIVE_COLOR } else { 0x303030 };
        for dy in 0..h {
            for dx in 0..w {
                let pos = (y + dy) * width + (x + dx);
                if pos < buffer.len() {
                    buffer[pos] = bg_color;
                }
            }
        }

        // Draw icon
        self.draw_menu_icon(buffer, width, x + 4, y + 4, item.icon);
        
        // Draw text
        // ... text rendering implementation
    }

    fn draw_menu_icon(&self, buffer: &mut Vec<u32>, width: usize, x: usize, y: usize, icon: MenuIcon) {
        let icon_color = match icon {
            MenuIcon::Programs => 0x40FF40,
            MenuIcon::Documents => 0xFFFF40,
            MenuIcon::Settings => 0x4040FF,
            MenuIcon::Search => 0xFF4040,
            MenuIcon::Power => 0xFF0000,
            MenuIcon::User => 0x40FFFF,
        };

        // Draw 16x16 icon
        for dy in 0..16 {
            for dx in 0..16 {
                let pos = (y + dy) * width + (x + dx);
                if pos < buffer.len() {
                    buffer[pos] = icon_color;
                }
            }
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn is_open(&self) -> bool {
        self.visible
    }

    pub fn handle_click(&mut self, x: usize, y: usize) -> bool {
        if !self.visible {
            return false;
        }

        // Check if click is within menu bounds
        for (i, item) in self.items.iter().enumerate() {
            if self.is_point_in_item(x, y, item) {
                self.handle_menu_action(item.action.clone());
                return true;
            }
        }

        // Check power options
        if self.handle_power_click(x, y) {
            return true;
        }

        false
    }

    fn is_point_in_item(&self, x: usize, y: usize, item: &MenuItem) -> bool {
        let (ix, iy) = item.position;
        let (w, h) = item.size;
        x >= ix && x < ix + w && y >= iy && y < iy + h
    }

    fn handle_menu_action(&mut self, action: MenuAction) {
        match action {
            MenuAction::OpenPrograms => {
                // Open programs window
            }
            MenuAction::StartSearch => {
                // Start search
            }
            MenuAction::ShowPowerOptions => {
                // Show power options
            }
            // Handle other actions...
            _ => {}
        }
        self.visible = false;
    }
} 