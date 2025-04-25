use crate::src::os::{kernel::Kernel, fs::VirtualFileSystem};
use super::{window::WindowManager, taskbar::Taskbar};
use super::common::{blend_colors, ACTIVE_COLOR, INACTIVE_COLOR};

pub struct Desktop {
    window_manager: WindowManager,
    taskbar: Taskbar,
    icons: Vec<DesktopIcon>,
    wallpaper: WallpaperConfig,
    start_menu: StartMenu,
}

struct WallpaperConfig {
    base_color: u32,
    gradient: bool,
    gradient_color: u32,
}

pub struct DesktopIcon {
    name: String,
    icon_type: IconType,
    position: (usize, usize),
    selected: bool,
    double_clicked: bool,
}

#[derive(Clone, Copy)]
pub enum IconType {
    MyComputer,
    Documents,
    Settings,
    Terminal,
    ProcessManager,
    FileExplorer,
}

impl Desktop {
    pub fn new() -> Self {
        Self {
            window_manager: WindowManager::new(),
            taskbar: Taskbar::new(),
            icons: Self::create_default_icons(),
            wallpaper: WallpaperConfig {
                base_color: 0x000040,
                gradient: true,
                gradient_color: 0x000080,
            },
            start_menu: StartMenu::new(),
        }
    }

    pub fn render(&mut self, buffer: &mut Vec<u32>, width: usize, height: usize, 
                 kernel: &Kernel, fs: &VirtualFileSystem) {
        // Draw wallpaper
        self.draw_wallpaper(buffer, width, height);
        
        // Draw desktop icons
        self.draw_icons(buffer, width, height);
        
        // Draw windows
        self.window_manager.render(buffer, width, height);
        
        // Draw taskbar
        self.taskbar.render(buffer, width, height);
        
        // Draw start menu if open
        if self.start_menu.is_open() {
            self.start_menu.render(buffer, width, height);
        }
    }

    fn draw_wallpaper(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        if self.wallpaper.gradient {
            for y in 0..height {
                let progress = y as f32 / height as f32;
                let color = blend_colors(
                    self.wallpaper.base_color,
                    self.wallpaper.gradient_color,
                    progress
                );
                
                for x in 0..width {
                    let pos = y * width + x;
                    buffer[pos] = color;
                }
            }
        } else {
            for pixel in buffer.iter_mut() {
                *pixel = self.wallpaper.base_color;
            }
        }
    }

    fn draw_icons(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        for icon in &self.icons {
            self.draw_icon(buffer, width, icon);
        }
    }

    fn draw_icon(&self, buffer: &mut Vec<u32>, width: usize, icon: &DesktopIcon) {
        let (x, y) = icon.position;
        let icon_size = 32;
        let icon_color = self.get_icon_color(icon.icon_type);
        
        // Draw icon background if selected
        if icon.selected {
            for dy in 0..icon_size {
                for dx in 0..icon_size {
                    let pos = (y + dy) * width + (x + dx);
                    if pos < buffer.len() {
                        buffer[pos] = blend_colors(buffer[pos], ACTIVE_COLOR, 0.3);
                    }
                }
            }
        }

        // Draw icon
        for dy in 4..icon_size-4 {
            for dx in 4..icon_size-4 {
                let pos = (y + dy) * width + (x + dx);
                if pos < buffer.len() {
                    buffer[pos] = icon_color;
                }
            }
        }

        // Draw icon label
        self.draw_icon_label(buffer, width, x, y + icon_size + 4, &icon.name);
    }

    fn get_icon_color(&self, icon_type: IconType) -> u32 {
        match icon_type {
            IconType::MyComputer => 0x4040FF,
            IconType::Documents => 0xFFFF40,
            IconType::Settings => 0x40FF40,
            IconType::Terminal => 0x404040,
            IconType::ProcessManager => 0xFF4040,
            IconType::FileExplorer => 0x40FFFF,
        }
    }

    pub fn handle_click(&mut self, x: f32, y: f32) -> bool {
        let x = x as usize;
        let y = y as usize;

        // Check start menu first
        if self.start_menu.is_open() && self.start_menu.handle_click(x, y) {
            return true;
        }

        // Check taskbar
        if self.taskbar.handle_click(x, y) {
            return true;
        }

        // Check windows
        if self.window_manager.handle_click(x as f32, y as f32) {
            return true;
        }

        // Check desktop icons
        self.handle_icon_click(x, y)
    }

    fn handle_icon_click(&mut self, x: usize, y: usize) -> bool {
        for icon in &mut self.icons {
            let (ix, iy) = icon.position;
            if x >= ix && x < ix + 32 && y >= iy && y < iy + 32 {
                if icon.selected && !icon.double_clicked {
                    // Handle double click
                    self.open_icon(icon.icon_type);
                    icon.double_clicked = true;
                } else {
                    icon.selected = true;
                    icon.double_clicked = false;
                }
                return true;
            }
        }
        false
    }

    fn open_icon(&mut self, icon_type: IconType) {
        match icon_type {
            IconType::ProcessManager => {
                self.window_manager.open_process_manager();
            }
            IconType::FileExplorer => {
                self.window_manager.open_file_explorer();
            }
            IconType::Terminal => {
                self.window_manager.open_terminal();
            }
            // Handle other icon types...
            _ => {}
        }
    }
}
