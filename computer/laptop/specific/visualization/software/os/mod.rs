use crate::src::os::{kernel::Kernel, process::Process, fs::VirtualFileSystem};
use super::{processes::ProcessVisualizer, filesystem::FilesystemVisualizer};
use super::common::{DataFlow, blend_colors, ACTIVE_COLOR, INACTIVE_COLOR};

pub struct OSVisualizer {
    process_vis: ProcessVisualizer,
    filesystem_vis: FilesystemVisualizer,
    desktop_icons: Vec<DesktopIcon>,
    start_menu: StartMenu,
    taskbar: Taskbar,
}

struct DesktopIcon {
    name: String,
    icon_type: IconType,
    position: (usize, usize),
    selected: bool,
}

enum IconType {
    Folder,
    Application,
    Document,
    System,
}

struct StartMenu {
    visible: bool,
    items: Vec<MenuItem>,
    position: (usize, usize),
}

struct Taskbar {
    running_apps: Vec<TaskbarItem>,
    system_tray: Vec<SystemTrayIcon>,
    height: usize,
}

impl OSVisualizer {
    pub fn new() -> Self {
        Self {
            process_vis: ProcessVisualizer::new(),
            filesystem_vis: FilesystemVisualizer::new(),
            desktop_icons: Self::create_default_icons(),
            start_menu: StartMenu::new(),
            taskbar: Taskbar::new(),
        }
    }

    pub fn render(&mut self, buffer: &mut Vec<u32>, width: usize, height: usize, 
                 kernel: &Kernel, fs: &VirtualFileSystem) {
        // Draw desktop background
        self.draw_desktop_background(buffer, width, height);
        
        // Draw desktop icons
        self.draw_desktop_icons(buffer, width, height);
        
        // Draw taskbar
        self.taskbar.render(buffer, width, height);
        
        // Draw start menu if visible
        if self.start_menu.visible {
            self.start_menu.render(buffer, width, height);
        }

        // Draw process and filesystem visualizations in windows
        self.process_vis.render(buffer, width, height, kernel);
        self.filesystem_vis.render(buffer, width, height, fs);
    }

    fn draw_desktop_background(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw gradient background
        for y in 0..height {
            let gradient = (y as f32 / height as f32 * 40.0) as u32;
            let color = 0x000040 + gradient;
            
            for x in 0..width {
                let pos = y * width + x;
                if pos < buffer.len() {
                    buffer[pos] = color;
                }
            }
        }
    }

    fn draw_desktop_icons(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        for icon in &self.desktop_icons {
            self.draw_icon(buffer, width, icon);
        }
    }

    fn draw_icon(&self, buffer: &mut Vec<u32>, width: usize, icon: &DesktopIcon) {
        let (x, y) = icon.position;
        let icon_size = 32;
        
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

        // Draw icon based on type
        let icon_color = match icon.icon_type {
            IconType::Folder => 0xFFFF00,
            IconType::Application => 0x00FF00,
            IconType::Document => 0x0000FF,
            IconType::System => 0xFF0000,
        };

        // Draw icon shape
        // ... implementation
    }

    fn create_default_icons() -> Vec<DesktopIcon> {
        vec![
            DesktopIcon {
                name: "My Computer".to_string(),
                icon_type: IconType::System,
                position: (20, 20),
                selected: false,
            },
            DesktopIcon {
                name: "Documents".to_string(),
                icon_type: IconType::Folder,
                position: (20, 80),
                selected: false,
            },
            // Add more default icons
        ]
    }

    pub fn handle_click(&mut self, x: f32, y: f32) {
        // Handle clicks on desktop icons
        for icon in &mut self.desktop_icons {
            // Check if click is within icon bounds
            // ... implementation
        }

        // Handle taskbar clicks
        self.taskbar.handle_click(x, y);

        // Handle start menu clicks if visible
        if self.start_menu.visible {
            self.start_menu.handle_click(x, y);
        }
    }
}
