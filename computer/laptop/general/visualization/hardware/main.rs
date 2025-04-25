// visualize the hardware components body of the laptop from ../src/hardware

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use crate::hardware::main::Hardware;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// Colors for different hardware components
const CPU_COLOR: u32 = 0x00A5A5;    // Teal
const GPU_COLOR: u32 = 0x4CAF50;    // Green
const MEMORY_COLOR: u32 = 0x3F51B5; // Indigo
const STORAGE_COLOR: u32 = 0x673AB7; // Deep Purple
const DISPLAY_COLOR: u32 = 0x2196F3; // Blue
const KEYBOARD_COLOR: u32 = 0xFFC107; // Amber
const TOUCHPAD_COLOR: u32 = 0xFF9800; // Orange
const BATTERY_COLOR: u32 = 0xF44336; // Red
const NETWORK_COLOR: u32 = 0x9C27B0; // Purple
const PORTS_COLOR: u32 = 0x795548;   // Brown

pub struct HardwareVisualization {
    buffer: Vec<u32>,
    window: Window,
    rotation_angle: f32,
    hardware: Option<Hardware>,
}

impl HardwareVisualization {
    pub fn new() -> Result<Self, String> {
        let mut window = Window::new(
            "Hardware Components Visualization",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .map_err(|e| format!("Failed to create window: {}", e))?;

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(Duration::from_micros(16600)));

        Ok(HardwareVisualization {
            buffer: vec![0; WIDTH * HEIGHT],
            window,
            rotation_angle: 0.0,
            hardware: None,
        })
    }

    pub fn connect_hardware(&mut self, hardware: Hardware) {
        println!("Connecting hardware to visualization...");
        self.hardware = Some(hardware);
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // Clear buffer
            self.buffer.iter_mut().for_each(|pixel| *pixel = 0x000000);
            
            // Render laptop hardware components
            self.render_laptop_hardware();
            
            // Update rotation for 3D effect
            self.rotation_angle += 0.01;
            if self.rotation_angle > std::f32::consts::PI * 2.0 {
                self.rotation_angle = 0.0;
            }
            
            // Update window with new buffer
            self.window
                .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
                .map_err(|e| format!("Failed to update window: {}", e))?;
                
            // Handle keyboard inputs
            if self.window.is_key_down(Key::Space) {
                println!("Space pressed - Return to main view");
                break;
            }
        }

        Ok(())
    }

    fn render_laptop_hardware(&mut self) {
        // Draw the laptop outline (transparent/wireframe to see inside)
        self.draw_laptop_outline();
        
        // Draw all internal components
        self.draw_cpu();
        self.draw_gpu();
        self.draw_memory();
        self.draw_storage();
        self.draw_battery();
        self.draw_display_controller();
        self.draw_keyboard_controller();
        self.draw_touchpad_controller();
        self.draw_network_card();
        self.draw_ports();
        
        // Draw the connections between components
        self.draw_connections();
        
        // Draw component labels
        self.draw_component_labels();
    }

    fn draw_laptop_outline(&mut self) {
        // Draw a wireframe outline of the laptop
        let outline_color = 0x303030; // Dark gray
        
        // Base outline
        let base_width = 600;
        let base_height = 40;
        let base_x = (WIDTH - base_width) / 2;
        let base_y = HEIGHT - 150;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 50.0) as usize;
        let corrected_base_x = base_x + perspective_x;
        
        // Draw base outline (just the edges, not filled)
        for x in 0..base_width {
            if corrected_base_x + x < WIDTH {
                // Top edge
                if base_y < HEIGHT {
                    self.buffer[base_y * WIDTH + corrected_base_x + x] = outline_color;
                }
                // Bottom edge
                if base_y + base_height < HEIGHT {
                    self.buffer[(base_y + base_height) * WIDTH + corrected_base_x + x] = outline_color;
                }
            }
        }
        
        for y in 0..base_height {
            if base_y + y < HEIGHT {
                // Left edge
                if corrected_base_x < WIDTH {
                    self.buffer[(base_y + y) * WIDTH + corrected_base_x] = outline_color;
                }
                // Right edge
                if corrected_base_x + base_width < WIDTH {
                    self.buffer[(base_y + y) * WIDTH + corrected_base_x + base_width - 1] = outline_color;
                }
            }
        }
    }
    
    fn draw_cpu(&mut self) {
        // Draw the CPU component
        let cpu_x = (WIDTH / 2) - 100;
        let cpu_y = HEIGHT - 120;
        let cpu_width = 80;
        let cpu_height = 80;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        self.draw_component(cpu_x + perspective_x, cpu_y, cpu_width, cpu_height, CPU_COLOR);
    }
    
    fn draw_gpu(&mut self) {
        // Draw the GPU component
        let gpu_x = (WIDTH / 2) + 50;
        let gpu_y = HEIGHT - 120;
        let gpu_width = 100;
        let gpu_height = 60;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        self.draw_component(gpu_x + perspective_x, gpu_y, gpu_width, gpu_height, GPU_COLOR);
    }
    
    fn draw_memory(&mut self) {
        // Draw the memory modules (RAM)
        let memory_count = 2; // Two RAM sticks
        let memory_width = 30;
        let memory_height = 80;
        let memory_gap = 10;
        
        let memory_start_x = (WIDTH / 2) - 180;
        let memory_y = HEIGHT - 130;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        for i in 0..memory_count {
            let memory_x = memory_start_x + i * (memory_width + memory_gap);
            self.draw_component(memory_x + perspective_x, memory_y, memory_width, memory_height, MEMORY_COLOR);
        }
    }
    
    fn draw_storage(&mut self) {
        // Draw the storage component (SSD/HDD)
        let storage_x = (WIDTH / 2) + 200;
        let storage_y = HEIGHT - 130;
        let storage_width = 70;
        let storage_height = 70;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        self.draw_component(storage_x + perspective_x, storage_y, storage_width, storage_height, STORAGE_COLOR);
    }
    
    fn draw_battery(&mut self) {
        // Draw the battery
        let battery_x = (WIDTH / 2) - 50;
        let battery_y = HEIGHT - 180;
        let battery_width = 100;
        let battery_height = 50;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        self.draw_component(battery_x + perspective_x, battery_y, battery_width, battery_height, BATTERY_COLOR);
    }
    
    fn draw_display_controller(&mut self) {
        // Draw the display controller
        let controller_x = (WIDTH / 2) - 220;
        let controller_y = HEIGHT - 220;
        let controller_width = 40;
        let controller_height = 40;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        self.draw_component(controller_x + perspective_x, controller_y, controller_width, controller_height, DISPLAY_COLOR);
    }
    
    fn draw_keyboard_controller(&mut self) {
        // Draw the keyboard controller
        let controller_x = (WIDTH / 2) - 50;
        let controller_y = HEIGHT - 230;
        let controller_width = 40;
        let controller_height = 30;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        self.draw_component(controller_x + perspective_x, controller_y, controller_width, controller_height, KEYBOARD_COLOR);
    }
    
    fn draw_touchpad_controller(&mut self) {
        // Draw the touchpad controller
        let controller_x = (WIDTH / 2) + 10;
        let controller_y = HEIGHT - 230;
        let controller_width = 30;
        let controller_height = 30;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        self.draw_component(controller_x + perspective_x, controller_y, controller_width, controller_height, TOUCHPAD_COLOR);
    }
    
    fn draw_network_card(&mut self) {
        // Draw the network card (WiFi/Bluetooth)
        let network_x = (WIDTH / 2) + 150;
        let network_y = HEIGHT - 230;
        let network_width = 50;
        let network_height = 40;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        self.draw_component(network_x + perspective_x, network_y, network_width, network_height, NETWORK_COLOR);
    }
    
    fn draw_ports(&mut self) {
        // Draw the ports on the sides of the laptop
        // USB, HDMI, etc.
        let port_width = 15;
        let port_height = 10;
        let port_gap = 20;
        let port_count = 5;
        
        // Left side ports
        let left_port_x = (WIDTH / 2) - 300;
        let port_start_y = HEIGHT - 150;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        for i in 0..port_count {
            let port_y = port_start_y + i * port_gap;
            self.draw_component(left_port_x + perspective_x, port_y, port_width, port_height, PORTS_COLOR);
        }
        
        // Right side ports
        let right_port_x = (WIDTH / 2) + 300 - port_width;
        
        for i in 0..port_count {
            let port_y = port_start_y + i * port_gap;
            self.draw_component(right_port_x + perspective_x, port_y, port_width, port_height, PORTS_COLOR);
        }
    }
    
    fn draw_connections(&mut self) {
        // Draw connections between components (simplified)
        let connection_color = 0x808080; // Gray
        
        // Example: CPU to RAM connection
        let cpu_x = (WIDTH / 2) - 100 + 40; // center of CPU
        let cpu_y = HEIGHT - 120 + 40;      // center of CPU
        let ram_x = (WIDTH / 2) - 180 + 15;  // center of first RAM stick
        let ram_y = HEIGHT - 130 + 40;      // center of RAM
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        
        self.draw_line(
            cpu_x + perspective_x, 
            cpu_y, 
            ram_x + perspective_x, 
            ram_y, 
            connection_color
        );
        
        // CPU to GPU connection
        let gpu_x = (WIDTH / 2) + 50 + 50; // center of GPU
        let gpu_y = HEIGHT - 120 + 30;     // center of GPU
        
        self.draw_line(
            cpu_x + perspective_x, 
            cpu_y, 
            gpu_x + perspective_x, 
            gpu_y, 
            connection_color
        );
        
        // CPU to Storage connection
        let storage_x = (WIDTH / 2) + 200 + 35; // center of Storage
        let storage_y = HEIGHT - 130 + 35;      // center of Storage
        
        self.draw_line(
            cpu_x + perspective_x, 
            cpu_y, 
            storage_x + perspective_x, 
            storage_y, 
            connection_color
        );
        
        // More connections can be added as needed
    }
    
    fn draw_component_labels(&mut self) {
        // This would be more complex in a real implementation
        // For simplicity, we'll skip actual text rendering
        // In a real app, you'd use a font rendering library
    }
    
    // Helper function to draw a component rectangle
    fn draw_component(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for dy in 0..height {
            for dx in 0..width {
                if x + dx < WIDTH && y + dy < HEIGHT {
                    // Add some 3D effect with shading
                    let shade = ((dx as f32 / width as f32 + dy as f32 / height as f32) * 30.0) as u32;
                    let component_color = if shade > 30 { color } else { color - shade };
                    
                    self.buffer[(y + dy) * WIDTH + x + dx] = component_color;
                }
            }
        }
    }
    
    // Helper function to draw a line between two points (simple Bresenham's algorithm)
    fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u32) {
        let dx = if x0 < x1 { x1 - x0 } else { x0 - x1 };
        let dy = if y0 < y1 { y1 - y0 } else { y0 - y1 };
        
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        
        let mut err = if dx > dy { dx } else { -dy } as i32 / 2;
        let mut x = x0 as i32;
        let mut y = y0 as i32;
        
        loop {
            if x >= 0 && y >= 0 && x < WIDTH as i32 && y < HEIGHT as i32 {
                self.buffer[y as usize * WIDTH + x as usize] = color;
            }
            
            if x == x1 as i32 && y == y1 as i32 {
                break;
            }
            
            let e2 = err;
            if e2 > -(dx as i32) {
                err -= dy as i32;
                x += sx;
            }
            if e2 < dy as i32 {
                err += dx as i32;
                y += sy;
            }
        }
    }
}

pub fn run(hardware: Option<Hardware>) -> Result<(), String> {
    println!("Starting hardware components visualization...");
    let mut visualization = HardwareVisualization::new()?;
    
    if let Some(hw) = hardware {
        visualization.connect_hardware(hw);
    }
    
    visualization.run()
}

// Entry point for standalone execution
pub fn main() -> Result<(), String> {
    // When run standalone, create a new hardware instance
    let hardware = Hardware::new();
    run(Some(hardware))
}