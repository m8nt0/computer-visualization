// Personal laptop visualization module
// Adapts the core personal laptop logic to the minifb UI framework

use crate::{ComputerRenderer, Color, InputEvent, Key, MouseButton, Event, Computer};
use crate::laptop::personal::PersonalLaptop;
use crate::laptop::ViewMode;
use crate::common::components::Component;
use minifb::{Window, WindowOptions, KeyRepeat};
use std::time::Duration;

// Constants for the visualization
pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

// MinifbRenderer adapts our generic rendering interface to minifb
pub struct MinifbRenderer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl MinifbRenderer {
    pub fn new(width: usize, height: usize) -> Self {
        MinifbRenderer {
            buffer: vec![0; width * height],
            width,
            height,
        }
    }
    
    pub fn buffer(&self) -> &Vec<u32> {
        &self.buffer
    }
}

impl ComputerRenderer for MinifbRenderer {
    fn clear(&mut self, color: Color) {
        for i in self.buffer.iter_mut() {
            *i = color.to_rgba();
        }
    }
    
    fn draw_text(&mut self, text: &str, x: f32, y: f32, color: Color) {
        // This is a simple text rendering implementation
        // In a real application, you would use a proper font rendering library
        let char_width = 8;
        let char_height = 12;
        let x = x as i32;
        let y = y as i32;
        let color_value = color.to_rgba();

        for (i, c) in text.chars().enumerate() {
            if c == ' ' {
                continue;
            }

            let char_x = x + (i as i32 * char_width);
            
            for cy in 0..char_height {
                for cx in 0..char_width {
                    let px = char_x + cx;
                    let py = y + cy;
                    
                    if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                        if is_pixel_set(c, cx, cy) {
                            let idx = py as usize * self.width + px as usize;
                            self.buffer[idx] = color_value;
                        }
                    }
                }
            }
        }
    }
    
    fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color) {
        let x_start = x.max(0.0) as usize;
        let y_start = y.max(0.0) as usize;
        let x_end = (x + width).min(self.width as f32) as usize;
        let y_end = (y + height).min(self.height as f32) as usize;
        let color_value = color.to_rgba();
        
        for y in y_start..y_end {
            for x in x_start..x_end {
                let idx = y * self.width + x;
                self.buffer[idx] = color_value;
            }
        }
    }
    
    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color) {
        // Bresenham's line algorithm
        let x1 = x1 as i32;
        let y1 = y1 as i32;
        let x2 = x2 as i32;
        let y2 = y2 as i32;
        let color_value = color.to_rgba();
        
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };
        let mut err = dx + dy;
        
        let mut x = x1;
        let mut y = y1;
        
        loop {
            if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
                let idx = y as usize * self.width + x as usize;
                self.buffer[idx] = color_value;
            }
            
            if x == x2 && y == y2 {
                break;
            }
            
            let e2 = 2 * err;
            if e2 >= dy {
                if x == x2 {
                    break;
                }
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                if y == y2 {
                    break;
                }
                err += dx;
                y += sy;
            }
        }
    }
    
    fn present(&mut self) {
        // No-op for minifb, as we update the window separately
    }
}

// Helper function for character rendering
fn is_pixel_set(c: char, x: i32, y: i32) -> bool {
    match c {
        'A'..='Z' | 'a'..='z' => {
            // For letters, create patterns based on the character value
            let char_val = c as u8;
            let pattern = char_val % 16;
            
            // Create different patterns for each character
            match pattern {
                0 => (x + y) % 2 == 0,
                1 => x % 2 == 0,
                2 => y % 2 == 0,
                3 => (x * y) % 2 == 0,
                4 => (x + y) % 3 == 0,
                5 => x % 3 == 0,
                6 => y % 3 == 0,
                7 => (x * y) % 3 == 0,
                8 => (x + y + 1) % 2 == 0,
                9 => (x + 1) % 2 == 0,
                10 => (y + 1) % 2 == 0,
                11 => ((x + 1) * (y + 1)) % 2 == 0,
                12 => (x + y + 1) % 3 == 0,
                13 => (x + 1) % 3 == 0,
                14 => (y + 1) % 3 == 0,
                _ => ((x + 1) * (y + 1)) % 3 == 0,
            }
        },
        '0'..='9' => {
            // For numbers, use a simple pattern
            let digit = c as u8 - b'0';
            (x + y + digit as i32) % 3 == 0
        },
        '.' => x == 3 && y == 9,
        ',' => x == 3 && (y == 9 || y == 10),
        ':' => x == 3 && (y == 3 || y == 9),
        ';' => x == 3 && (y == 3 || y == 9 || y == 10),
        '!' => x == 3 && (y != 8),
        '?' => {
            (x >= 2 && x <= 6 && y == 1) || // top
            (x == 6 && y >= 1 && y <= 5) || // right side
            (x == 4 && y == 7) || // dot
            (x >= 4 && x <= 6 && y == 5)
        }
        '(' => x == 7 - y/2,
        ')' => x == y/2,
        '[' => x == 2 || (y == 1 || y == 11) && x < 5,
        ']' => x == 6 || (y == 1 || y == 11) && x > 3,
        '{' => (x == 2 && (y < 5 || y > 7)) || (x == 4 && y >= 5 && y <= 7) || ((y == 1 || y == 11) && x > 2 && x < 6),
        '}' => (x == 6 && (y < 5 || y > 7)) || (x == 4 && y >= 5 && y <= 7) || ((y == 1 || y == 11) && x > 2 && x < 6),
        '+' => (x == 4 && y >= 3 && y <= 9) || (y == 6 && x >= 1 && x <= 7),
        '-' => (y == 6 && x >= 1 && x <= 7),
        '*' => (y == x || y == 8-x) && x >= 2 && x <= 6,
        '/' => y == 11 - x,
        '\\' => y == x,
        '=' => (y == 4 || y == 8) && x >= 1 && x <= 7,
        '_' => y == 11 && x >= 1 && x <= 7,
        '<' => x == 10 - y || x == y - 2,
        '>' => x == y || x == 12 - y,
        '|' => x == 4,
        '&' => ((x == 3 || x == 5) && (y == 2 || y == 5)) || ((x == 2 || x == 6) && (y == 3 || y == 4)) || (x == 3 && y == 6) || (x == 4 && y == 7) || (x == 5 && y >= 8),
        '@' => ((y == 2 || y == 8) && x >= 3 && x <= 5) || ((x == 2 || x == 6) && y >= 3 && y <= 7) || (x == 5 && y >= 5) || (x == 4 && y == 5),
        '#' => ((x == 2 || x == 6) && y >= 2 && y <= 10) || (y == 4 || y == 8) && x >= 1 && x <= 7,
        '$' => (x == 4 && y >= 1 && y <= 11) || ((y == 2 || y == 10) && x >= 2 && x <= 6) || ((y == 6) && x >= 2 && x <= 6) || (x == 2 && y == 3) || (x == 6 && y == 9),
        '%' => (y == x && x != 4) || ((x == 2 || x == 6) && (y == 2 || y == 10)),
        '^' => (y == 2 && x >= 3 && x <= 5) || ((x == 2 || x == 6) && y == 3),
        '~' => ((x == 2 || x == 6) && y == 5) || ((x == 3 || x == 5) && y == 4) || (x == 4 && y == 5),
        _ => false,
    }
}

// Convert minifb key to our generic key
fn convert_key(key: minifb::Key) -> Option<Key> {
    match key {
        minifb::Key::Escape => Some(Key::Escape),
        minifb::Key::Enter => Some(Key::Enter),
        minifb::Key::Tab => Some(Key::Tab),
        minifb::Key::Space => Some(Key::Space),
        minifb::Key::Up => Some(Key::Up),
        minifb::Key::Down => Some(Key::Down),
        minifb::Key::Left => Some(Key::Left),
        minifb::Key::Right => Some(Key::Right),
        minifb::Key::Y => Some(Key::Char('y')),
        minifb::Key::N => Some(Key::Char('n')),
        minifb::Key::P => Some(Key::Char('p')),
        _ => None,
    }
}

// Draw external view of the laptop
fn draw_laptop_external(renderer: &mut MinifbRenderer, laptop: &PersonalLaptop) {
    // Draw the laptop case
    let case_color = Color::new(80, 80, 80, 255);
    renderer.draw_rectangle(100.0, 100.0, 600.0, 400.0, case_color);
    
    // Draw the screen (black if powered off, blue if powered on)
    let screen_color = if laptop.is_powered_on() {
        Color::new(20, 40, 80, 255)
    } else {
        Color::new(10, 10, 10, 255)
    };
    renderer.draw_rectangle(150.0, 150.0, 500.0, 300.0, screen_color);
    
    // Draw the keyboard
    let keyboard_color = Color::new(60, 60, 60, 255);
    renderer.draw_rectangle(200.0, 480.0, 400.0, 100.0, keyboard_color);
    
    // Draw some keyboard keys
    let key_color = Color::new(40, 40, 40, 255);
    for row in 0..4 {
        for col in 0..10 {
            renderer.draw_rectangle(
                220.0 + col as f32 * 36.0,
                500.0 + row as f32 * 20.0,
                30.0,
                16.0,
                key_color
            );
        }
    }
    
    // Draw touchpad
    renderer.draw_rectangle(350.0, 560.0, 100.0, 70.0, key_color);
}

// Draw hardware view of the laptop
fn draw_laptop_hardware(renderer: &mut MinifbRenderer, laptop: &PersonalLaptop) {
    // Draw the laptop internals
    let case_color = Color::new(40, 40, 40, 255);
    renderer.draw_rectangle(100.0, 100.0, 600.0, 400.0, case_color);
    
    // Draw the motherboard
    let mb_color = Color::new(0, 120, 0, 255);
    renderer.draw_rectangle(150.0, 150.0, 500.0, 300.0, mb_color);
    
    // Draw CPU
    let cpu_color = Color::new(120, 120, 120, 255);
    renderer.draw_rectangle(300.0, 200.0, 100.0, 100.0, cpu_color);
    renderer.draw_text("CPU", 335.0, 240.0, Color::new(0, 0, 0, 255));
    
    // Draw RAM
    let ram_color = Color::new(0, 100, 0, 255);
    renderer.draw_rectangle(450.0, 200.0, 150.0, 40.0, ram_color);
    renderer.draw_text("RAM", 510.0, 215.0, Color::new(0, 0, 0, 255));
    
    // Draw SSD
    let ssd_color = Color::new(100, 100, 0, 255);
    renderer.draw_rectangle(200.0, 350.0, 200.0, 50.0, ssd_color);
    renderer.draw_text("SSD", 285.0, 365.0, Color::new(0, 0, 0, 255));
    
    // Draw component details
    let text_color = Color::new(255, 255, 255, 255);
    renderer.draw_text(&format!("CPU: {}", laptop.cpu().description()), 150.0, 430.0, text_color);
    renderer.draw_text(&format!("RAM: {}", laptop.memory().description()), 150.0, 450.0, text_color);
    renderer.draw_text(&format!("Storage: {}", laptop.storage().description()), 150.0, 470.0, text_color);
}

// Draw software view of the laptop
fn draw_laptop_software(renderer: &mut MinifbRenderer, laptop: &PersonalLaptop) {
    // Draw operating system interface
    let desktop_color = Color::new(0, 80, 120, 255);
    renderer.draw_rectangle(100.0, 100.0, 600.0, 400.0, desktop_color);
    
    // Draw taskbar
    let taskbar_color = Color::new(40, 40, 40, 255);
    renderer.draw_rectangle(100.0, 480.0, 600.0, 20.0, taskbar_color);
    
    // Draw start button
    let start_color = Color::new(0, 120, 0, 255);
    renderer.draw_rectangle(110.0, 480.0, 50.0, 20.0, start_color);
    
    // Draw clock
    let text_color = Color::new(255, 255, 255, 255);
    renderer.draw_text("10:30 AM", 600.0, 485.0, text_color);
    
    // Draw some desktop icons
    let icon_color = Color::new(255, 255, 255, 255);
    renderer.draw_rectangle(150.0, 150.0, 40.0, 40.0, icon_color);
    renderer.draw_rectangle(150.0, 220.0, 40.0, 40.0, icon_color);
    renderer.draw_rectangle(150.0, 290.0, 40.0, 40.0, icon_color);
    
    // Draw a window
    let window_color = Color::new(240, 240, 240, 255);
    renderer.draw_rectangle(250.0, 200.0, 300.0, 200.0, window_color);
    
    // Draw window title bar
    let title_color = Color::new(0, 0, 120, 255);
    renderer.draw_rectangle(250.0, 200.0, 300.0, 20.0, title_color);
    renderer.draw_text("Explorer", 270.0, 205.0, Color::new(255, 255, 255, 255));
    
    // Draw close button
    let close_color = Color::new(255, 0, 0, 255);
    renderer.draw_rectangle(530.0, 200.0, 20.0, 20.0, close_color);
    
    // Draw user name
    renderer.draw_text(&format!("User: {}", laptop.user_name()), 400.0, 150.0, text_color);
}

// Draw access request dialog
fn draw_access_request(renderer: &mut MinifbRenderer) {
    // Draw a dialog box
    let dialog_color = Color::new(200, 200, 200, 255);
    renderer.draw_rectangle(200.0, 200.0, 400.0, 200.0, dialog_color);
    
    // Draw border
    let border_color = Color::new(100, 100, 100, 255);
    renderer.draw_rectangle(200.0, 200.0, 400.0, 2.0, border_color);
    renderer.draw_rectangle(200.0, 200.0, 2.0, 200.0, border_color);
    renderer.draw_rectangle(200.0, 398.0, 400.0, 2.0, border_color);
    renderer.draw_rectangle(598.0, 200.0, 2.0, 200.0, border_color);
    
    // Draw title bar
    let title_color = Color::new(0, 0, 120, 255);
    renderer.draw_rectangle(200.0, 200.0, 400.0, 20.0, title_color);
    
    // Draw message
    let text_color = Color::new(0, 0, 0, 255);
    renderer.draw_text("This visualization needs access to your laptop", 220.0, 240.0, text_color);
    renderer.draw_text("to show details specific to your hardware and software.", 220.0, 260.0, text_color);
    renderer.draw_text("Press 'Y' to grant access or 'N' to deny access", 220.0, 300.0, text_color);
    
    // Draw buttons
    let button_color = Color::new(220, 220, 220, 255);
    renderer.draw_rectangle(250.0, 350.0, 100.0, 30.0, button_color);
    renderer.draw_rectangle(450.0, 350.0, 100.0, 30.0, button_color);
    
    renderer.draw_text("Yes (Y)", 270.0, 360.0, text_color);
    renderer.draw_text("No (N)", 470.0, 360.0, text_color);
}

// Draw access denied message
fn draw_access_denied(renderer: &mut MinifbRenderer) {
    // Draw a dialog box
    let dialog_color = Color::new(200, 200, 200, 255);
    renderer.draw_rectangle(200.0, 200.0, 400.0, 200.0, dialog_color);
    
    // Draw border
    let border_color = Color::new(100, 100, 100, 255);
    renderer.draw_rectangle(200.0, 200.0, 400.0, 2.0, border_color);
    renderer.draw_rectangle(200.0, 200.0, 2.0, 200.0, border_color);
    renderer.draw_rectangle(200.0, 398.0, 400.0, 2.0, border_color);
    renderer.draw_rectangle(598.0, 200.0, 2.0, 200.0, border_color);
    
    // Draw title bar
    let title_color = Color::new(120, 0, 0, 255);
    renderer.draw_rectangle(200.0, 200.0, 400.0, 20.0, title_color);
    
    // Draw message
    let text_color = Color::new(0, 0, 0, 255);
    renderer.draw_text("Access Denied", 350.0, 280.0, text_color);
    renderer.draw_text("Exiting visualization...", 330.0, 320.0, text_color);
}

// Run the personal laptop visualization
pub fn run() {
    let mut window = Window::new(
        "Your Personal Laptop Visualization",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut renderer = MinifbRenderer::new(WIDTH, HEIGHT);
    let mut laptop = PersonalLaptop::new();

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // Clear the renderer
        renderer.clear(Color::new(32, 32, 32, 255));
        
        // Check for key presses and convert to our generic input events
        for key in [
            minifb::Key::Y, 
            minifb::Key::N, 
            minifb::Key::P, 
            minifb::Key::Tab,
            minifb::Key::Escape,
            minifb::Key::Enter,
        ].iter() {
            if window.is_key_pressed(*key, KeyRepeat::No) {
                if let Some(generic_key) = convert_key(*key) {
                    let input_event = InputEvent::KeyPressed(generic_key);
                    laptop.process_input(&input_event);
                }
            }
        }
        
        // Update laptop state
        laptop.update();
        
        // Render the appropriate view
        if laptop.is_requesting_access() {
            draw_access_request(&mut renderer);
        } else if !laptop.is_access_granted() {
            draw_access_denied(&mut renderer);
            
            // Exit the visualization after 3 seconds
            std::thread::sleep(Duration::from_secs(3));
            break;
        } else if !laptop.is_powered_on() {
            // Draw powered off laptop
            draw_laptop_external(&mut renderer, &laptop);
            
            // Draw instructions
            let text_color = Color::new(255, 255, 255, 255);
            renderer.draw_text("Press 'P' to power on your laptop", 20.0, (HEIGHT - 30) as f32, text_color);
        } else {
            // Draw the appropriate view based on the current view mode
            match laptop.view_mode() {
                ViewMode::External => draw_laptop_external(&mut renderer, &laptop),
                ViewMode::Hardware => draw_laptop_hardware(&mut renderer, &laptop),
                ViewMode::Software => draw_laptop_software(&mut renderer, &laptop),
            }
            
            // Draw instructions
            let text_color = Color::new(255, 255, 255, 255);
            renderer.draw_text(
                "Press 'Tab' to switch views, 'Escape' to exit", 
                20.0, 
                (HEIGHT - 30) as f32, 
                text_color
            );
        }
        
        // Update the window
        window.update_with_buffer(renderer.buffer(), WIDTH, HEIGHT).unwrap();
    }
}
