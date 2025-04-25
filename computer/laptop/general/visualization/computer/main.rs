// essentially the 3d view of the general laptop in this case, without before entering the screen mode or hardware mode.

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub struct LaptopVisualization {
    buffer: Vec<u32>,
    window: Window,
    rotation_angle: f32,
}

impl LaptopVisualization {
    pub fn new() -> Result<Self, String> {
        let mut window = Window::new(
            "Laptop External Visualization",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .map_err(|e| format!("Failed to create window: {}", e))?;

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(Duration::from_micros(16600)));

        Ok(LaptopVisualization {
            buffer: vec![0; WIDTH * HEIGHT],
            window,
            rotation_angle: 0.0,
        })
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // Clear buffer
            self.buffer.iter_mut().for_each(|pixel| *pixel = 0x000000);
            
            // Render laptop exterior
            self.render_laptop_exterior();
            
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

    fn render_laptop_exterior(&mut self) {
        // Simplified 3D rendering of laptop exterior
        self.draw_base();
        self.draw_screen();
        self.draw_keyboard();
        self.draw_touchpad();
        self.draw_logo();
    }

    fn draw_base(&mut self) {
        // Draw the base of the laptop (bottom part)
        let base_color = 0x303030; // Dark gray
        let base_width = 600;
        let base_height = 300;
        let base_x = (WIDTH - base_width) / 2;
        let base_y = HEIGHT - 150;
        
        // Apply perspective based on rotation
        let perspective_x = (self.rotation_angle.sin() * 50.0) as usize;
        let corrected_base_x = base_x + perspective_x;
        
        // Draw the base with perspective
        for y in 0..base_height {
            for x in 0..base_width {
                if corrected_base_x + x < WIDTH && base_y + y < HEIGHT {
                    // Add some shading based on the distance from the center
                    let shade = ((x as f32 / base_width as f32) * 30.0) as u32;
                    let color = base_color + shade;
                    
                    self.buffer[(base_y + y) * WIDTH + corrected_base_x + x] = color;
                }
            }
        }
    }

    fn draw_screen(&mut self) {
        // Draw the laptop screen (top part)
        let screen_color = 0x303030; // Dark gray
        let screen_width = 580;
        let screen_height = 350;
        let screen_x = (WIDTH - screen_width) / 2 + 10;
        let screen_y = HEIGHT - 450;
        
        // Adjust for perspective
        let perspective_x = (self.rotation_angle.sin() * 30.0) as usize;
        let corrected_screen_x = screen_x + perspective_x;
        
        // Draw the screen outer case
        for y in 0..screen_height {
            for x in 0..screen_width {
                if corrected_screen_x + x < WIDTH && screen_y + y < HEIGHT {
                    // Add some shading
                    let shade = ((y as f32 / screen_height as f32) * 20.0) as u32;
                    let color = screen_color + shade;
                    
                    self.buffer[(screen_y + y) * WIDTH + corrected_screen_x + x] = color;
                }
            }
        }
        
        // Draw the actual display (inner part of the screen)
        let display_color = 0x000000; // Black
        let display_width = 520;
        let display_height = 300;
        let display_x = corrected_screen_x + (screen_width - display_width) / 2;
        let display_y = screen_y + 25;
        
        for y in 0..display_height {
            for x in 0..display_width {
                if display_x + x < WIDTH && display_y + y < HEIGHT {
                    self.buffer[(display_y + y) * WIDTH + display_x + x] = display_color;
                }
            }
        }
    }
    
    fn draw_keyboard(&mut self) {
        // Draw a simplified keyboard on the base
        let keyboard_color = 0x202020; // Darker gray
        let keyboard_width = 500;
        let keyboard_height = 150;
        let keyboard_x = (WIDTH - keyboard_width) / 2 + 50;
        let keyboard_y = HEIGHT - 290;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 40.0) as usize;
        let corrected_keyboard_x = keyboard_x + perspective_x;
        
        // Draw the main keyboard area
        for y in 0..keyboard_height {
            for x in 0..keyboard_width {
                if corrected_keyboard_x + x < WIDTH && keyboard_y + y < HEIGHT {
                    self.buffer[(keyboard_y + y) * WIDTH + corrected_keyboard_x + x] = keyboard_color;
                }
            }
        }
        
        // Draw keyboard keys (simplified grid)
        let key_color = 0x383838; // Light gray
        let key_size = 20;
        let key_gap = 5;
        let keys_per_row = 12;
        let num_rows = 5;
        
        for row in 0..num_rows {
            for col in 0..keys_per_row {
                let key_x = corrected_keyboard_x + 30 + col * (key_size + key_gap);
                let key_y = keyboard_y + 20 + row * (key_size + key_gap);
                
                for y in 0..key_size {
                    for x in 0..key_size {
                        if key_x + x < WIDTH && key_y + y < HEIGHT {
                            self.buffer[(key_y + y) * WIDTH + key_x + x] = key_color;
                        }
                    }
                }
            }
        }
    }
    
    fn draw_touchpad(&mut self) {
        // Draw the touchpad below the keyboard
        let touchpad_color = 0x505050; // Gray
        let touchpad_width = 150;
        let touchpad_height = 100;
        let touchpad_x = (WIDTH - touchpad_width) / 2;
        let touchpad_y = HEIGHT - 130;
        
        // Apply perspective
        let perspective_x = (self.rotation_angle.sin() * 40.0) as usize;
        let corrected_touchpad_x = touchpad_x + perspective_x;
        
        for y in 0..touchpad_height {
            for x in 0..touchpad_width {
                if corrected_touchpad_x + x < WIDTH && touchpad_y + y < HEIGHT {
                    self.buffer[(touchpad_y + y) * WIDTH + corrected_touchpad_x + x] = touchpad_color;
                }
            }
        }
    }
    
    fn draw_logo(&mut self) {
        // Draw a simplified logo on the back of the screen
        let logo_color = 0xA0A0A0; // Silver
        let logo_size = 50;
        let logo_x = (WIDTH - logo_size) / 2;
        let logo_y = HEIGHT - 350;
        
        // Apply perspective for the logo on the back of the screen
        let perspective_x = (self.rotation_angle.sin() * 20.0) as usize;
        let corrected_logo_x = logo_x + perspective_x;
        
        // Draw a simplified circle logo
        for y in 0..logo_size {
            for x in 0..logo_size {
                let dx = x as f32 - logo_size as f32 / 2.0;
                let dy = y as f32 - logo_size as f32 / 2.0;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= logo_size as f32 / 2.0 {
                    if corrected_logo_x + x < WIDTH && logo_y + y < HEIGHT {
                        self.buffer[(logo_y + y) * WIDTH + corrected_logo_x + x] = logo_color;
                    }
                }
            }
        }
    }
}

pub fn run() -> Result<(), String> {
    println!("Starting laptop exterior visualization...");
    let mut visualization = LaptopVisualization::new()?;
    visualization.run()
}

// Entry point for standalone execution
pub fn main() -> Result<(), String> {
    run()
}