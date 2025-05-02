use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

// Import our components
use crate::hardware::main::Hardware;
use crate::software::main::Software;
use crate::visualization::computer::main as computer_viz;
use crate::visualization::hardware::main as hardware_viz;
use crate::visualization::software::main as software_viz;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

enum ViewMode {
    External,  // The laptop from outside (keyboard, screen, etc.)
    Hardware,  // Inside the laptop (components)
    Software,  // Software flowing through hardware
}

pub fn main() -> Result<(), String> {
    // Initialize the hardware and software components
    println!("Initializing hardware components...");
    let mut hardware = Hardware::new();
    hardware.initialize();
    
    println!("Initializing software components...");
    let mut software = Software::new();
    software.connect_to_hardware(hardware.clone());
    software.boot();
    
    println!("Ready to visualize the laptop. Use the following keys:");
    println!("  '1' - View external laptop");
    println!("  '2' - View hardware components");
    println!("  '3' - View software flowing through hardware");
    println!("  'Escape' - Exit");
    
    let mut window = Window::new(
        "General Laptop Visualization - Main Menu",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).map_err(|e| format!("Failed to create window: {}", e))?;

    // Limit to max ~60 fps
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut view_mode = ViewMode::External;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer
        for i in buffer.iter_mut() {
            *i = 0x202020; // Dark gray background
        }

        // Draw menu text
        draw_text(&mut buffer, "Laptop Visualization System", 300, 100, 0xFFFFFF);
        draw_text(&mut buffer, "Press '1' to view external laptop", 300, 150, 0xFFFFFF);
        draw_text(&mut buffer, "Press '2' to view hardware components", 300, 200, 0xFFFFFF);
        draw_text(&mut buffer, "Press '3' to view software flowing through hardware", 300, 250, 0xFFFFFF);
        draw_text(&mut buffer, "Press 'Escape' to exit", 300, 300, 0xFFFFFF);
        
        // Handle key presses
        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            println!("Launching external laptop view...");
            window.set_title("Loading External View...");
            // Run the external laptop visualization
            computer_viz::run()?;
            window.set_title("General Laptop Visualization - Main Menu");
        }
        
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
            println!("Launching hardware components view...");
            window.set_title("Loading Hardware View...");
            // Run the hardware visualization with our initialized hardware
            hardware_viz::run(Some(hardware.clone()))?;
            window.set_title("General Laptop Visualization - Main Menu");
        }
        
        if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
            println!("Launching software flow visualization...");
            window.set_title("Loading Software Flow View...");
            // Run the software flow visualization with our initialized components
            software_viz::run(Some(hardware.clone()), Some(software.clone()))?;
            window.set_title("General Laptop Visualization - Main Menu");
        }

        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)
            .map_err(|e| format!("Failed to update window: {}", e))?;
    }
    
    // Clean shutdown
    println!("Shutting down software components...");
    software.shutdown();
    
    println!("Shutting down hardware components...");
    hardware.shutdown();
    
    println!("Visualization system terminated.");
    Ok(())
}

// Simple text rendering function
fn draw_text(buffer: &mut Vec<u32>, text: &str, x: usize, y: usize, color: u32) {
    // In a real implementation, this would use a font rendering library
    // For this simple version, we'll just draw a placeholder rectangle
    let text_width = text.len() * 8;
    let text_height = 12;
    
    for dy in 0..text_height {
        for dx in 0..text_width {
            if x + dx < WIDTH && y + dy < HEIGHT {
                buffer[(y + dy) * WIDTH + (x + dx)] = color;
            }
        }
    }
}