// Main entry point for the personal laptop visualization
// This file delegates to the visualization module that uses the core laptop logic

use computer_visual::laptop::personal::visualization::{WIDTH, HEIGHT, run};

fn main() {
    // This is the main entry point for the personal laptop visualization.
    // We delegate to the visualization module's run function.
    run();
} 

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

// This function can be used to run the visualization directly
pub fn run_visualization() {
    let mut window = Window::new(
        "Your Personal Laptop Visualization",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut power_on = false;
    let mut view_mode = ViewMode::External;
    let mut requesting_access = true;
    let mut access_granted = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer
        for i in buffer.iter_mut() {
            *i = 0x00202020; // Dark gray background
        }

        if requesting_access {
            draw_access_request(&mut buffer);
            
            // Check for key press to grant/deny access
            if window.is_key_pressed(Key::Y, minifb::KeyRepeat::No) {
                requesting_access = false;
                access_granted = true;
            }
            
            if window.is_key_pressed(Key::N, minifb::KeyRepeat::No) {
                requesting_access = false;
                access_granted = false;
            }
        } else if !access_granted {
            draw_access_denied(&mut buffer);
            
            // Exit the visualization after 3 seconds
            std::thread::sleep(Duration::from_secs(3));
            break;
        } else if !power_on {
            draw_laptop_powered_off(&mut buffer);
            
            // Check for power button press
            if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
                power_on = true;
            }
        } else {
            match view_mode {
                ViewMode::External => draw_laptop_external(&mut buffer),
                ViewMode::Hardware => draw_laptop_hardware(&mut buffer),
                ViewMode::Software => draw_laptop_software(&mut buffer),
            }

            // Toggle view mode
            if window.is_key_pressed(Key::Tab, minifb::KeyRepeat::No) {
                view_mode = match view_mode {
                    ViewMode::External => ViewMode::Hardware,
                    ViewMode::Hardware => ViewMode::Software,
                    ViewMode::Software => ViewMode::External,
                };
            }
        }

        // Draw instructions
        let instructions = if requesting_access {
            "Press 'Y' to grant access or 'N' to deny access"
        } else if !access_granted {
            "Access Denied"
        } else if !power_on {
            "Press 'P' to power on your laptop"
        } else {
            "Press 'Tab' to switch views, 'Escape' to exit"
        };
        draw_text(&mut buffer, instructions, 20, (HEIGHT - 30) as i32, 0xFFFFFFFF);

        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

enum ViewMode {
    External,  // The laptop from outside (keyboard, screen, etc.)
    Hardware,  // Inside the laptop (components)
    Software,  // The laptop's operating system
}

fn draw_text(buffer: &mut Vec<u32>, text: &str, x: i32, y: i32, color: u32) {
    // This is a simple text rendering function
    // In a real application, you would use a proper font rendering library
    let char_width = 8;
    let char_height = 12;

    for (i, c) in text.chars().enumerate() {
        if c == ' ' {
            continue;
        }

        let char_x = x + (i as i32 * char_width);
        
        // Very basic character rendering - create a bitmap pattern for each character
        for cy in 0..char_height {
            for cx in 0..char_width {
                let px = char_x + cx;
                let py = y + cy;
                
                if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                    // Create a unique pattern for each character
                    // This doesn't create readable text but makes each character visually distinct
                    if is_pixel_set(c, cx, cy) {
                        let idx = py as usize * WIDTH + px as usize;
                        buffer[idx] = color;
                    }
                }
            }
        }
    }
}

// A function to determine if a pixel should be set for a given character
fn is_pixel_set(c: char, x: i32, y: i32, ) -> bool {
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

fn draw_access_request(buffer: &mut Vec<u32>) {
    // Draw a centered message asking for access
    let message = "This visualization needs access to your laptop";
    let message2 = "to show details specific to your hardware and software.";
    let message3 = "Do you want to grant access?";
    
    // Draw a dialog box
    let box_width = 600;
    let box_height = 200;
    let box_x = (WIDTH - box_width) / 2;
    let box_y = (HEIGHT - box_height) / 2;
    
    // Draw dialog background
    for y in box_y..box_y + box_height {
        for x in box_x..box_x + box_width {
            buffer[y * WIDTH + x] = 0x00404040;
        }
    }
    
    // Draw dialog border
    for y in box_y..box_y + box_height {
        for x in box_x..box_x + box_width {
            if y == box_y || y == box_y + box_height - 1 ||
               x == box_x || x == box_x + box_width - 1 {
                buffer[y * WIDTH + x] = 0xFFFFFFFF;
            }
        }
    }
    
    // Draw message
    draw_text(buffer, message, (box_x + 50) as i32, (box_y + 60) as i32, 0xFFFFFFFF);
    draw_text(buffer, message2, (box_x + 50) as i32, (box_y + 85) as i32, 0xFFFFFFFF);
    draw_text(buffer, message3, (box_x + 50) as i32, (box_y + 120) as i32, 0xFFFFFFFF);
    
    // Draw Yes/No buttons
    draw_text(buffer, "Yes (Y)", (box_x + 200) as i32, (box_y + 150) as i32, 0xFFFFFF00);
    draw_text(buffer, "No (N)", (box_x + 350) as i32, (box_y + 150) as i32, 0xFFFFFF00);
}

fn draw_access_denied(buffer: &mut Vec<u32>) {
    // Draw a centered message indicating access is denied
    let message = "Access Denied";
    draw_text(buffer, message, ((WIDTH / 2) - 80) as i32, (HEIGHT / 2) as i32, 0xFF0000FF);
}

fn draw_laptop_powered_off(buffer: &mut Vec<u32>) {
    // Draw a closed laptop
    let laptop_width = 400;
    let laptop_height = 20;
    let laptop_x = (WIDTH - laptop_width) / 2;
    let laptop_y = (HEIGHT - laptop_height) / 2;
    
    // Draw laptop base (closed)
    for y in laptop_y..laptop_y + laptop_height {
        for x in laptop_x..laptop_x + laptop_width {
            buffer[y * WIDTH + x] = 0x00777777;
        }
    }
    
    // Draw power button
    let button_x = laptop_x + laptop_width / 2;
    let button_y = laptop_y - 10;
    for y in button_y-5..button_y + 5 {
        for x in button_x - 5..button_x + 5 {
            if y >= 0 && y < HEIGHT && x >= 0 && x < WIDTH {
                let dx = x as i32 - button_x as i32;
                let dy = y as i32 - button_y as i32;
                // Draw a circle
                if dx*dx + dy*dy <= 25 {
                    buffer[y * WIDTH + x] = 0x00AAAAAA;
                }
            }
        }
    }
    
    draw_text(buffer, "Power", (button_x - 20) as i32, (button_y - 15) as i32, 0xFFFFFFFF);
}

fn draw_laptop_external(buffer: &mut Vec<u32>) {
    // Draw an open laptop with screen and keyboard
    let base_width = 400;
    let base_height = 30;
    let base_x = (WIDTH - base_width) / 2;
    let base_y = HEIGHT / 2 + 50;
    
    let screen_width = 360;
    let screen_height = 240;
    let screen_x = (WIDTH - screen_width) / 2;
    let screen_y = base_y - screen_height - 10;
    
    // Draw laptop base (keyboard area)
    for y in base_y..base_y + base_height {
        for x in base_x..base_x + base_width {
            buffer[y * WIDTH + x] = 0x00777777;
        }
    }
    
    // Draw keyboard
    for row in 0..4 {
        for key in 0..10 {
            let key_width = 30;
            let key_height = 15;
            let key_x = base_x + 40 + key * (key_width + 5);
            let key_y = base_y + 5 + row * (key_height + 2);
            
            for y in key_y..key_y + key_height {
                for x in key_x..key_x + key_width {
                    if y < HEIGHT && x < WIDTH {
                        buffer[y * WIDTH + x] = 0x00AAAAAA;
                    }
                }
            }
        }
    }
    
    // Draw laptop screen
    for y in screen_y..screen_y + screen_height {
        for x in screen_x..screen_x + screen_width {
            // Screen border
            if y == screen_y || y == screen_y + screen_height - 1 ||
               x == screen_x || x == screen_x + screen_width - 1 {
                buffer[y * WIDTH + x] = 0x00777777;
            } else {
                // Screen content - blue desktop
                buffer[y * WIDTH + x] = 0x000000FF;
            }
        }
    }
    
    // Draw desktop icons
    draw_text(buffer, "My Files", (screen_x + 20) as i32, (screen_y + 30) as i32, 0xFFFFFFFF);
    draw_text(buffer, "Browser", (screen_x + 20) as i32, (screen_y + 60) as i32, 0xFFFFFFFF);
    draw_text(buffer, "Settings", (screen_x + 20) as i32, (screen_y + 90) as i32, 0xFFFFFFFF);
    
    draw_text(buffer, "External View", ((WIDTH / 2) - 60) as i32, 30, 0xFFFFFFFF);
}

fn draw_laptop_hardware(buffer: &mut Vec<u32>) {
    // Draw the internal hardware components
    
    // Draw the motherboard background
    let mb_width = 500;
    let mb_height = 350;
    let mb_x = (WIDTH - mb_width) / 2;
    let mb_y = (HEIGHT - mb_height) / 2;
    
    for y in mb_y..mb_y + mb_height {
        for x in mb_x..mb_x + mb_width {
            buffer[y * WIDTH + x] = 0x0000AA00; // Green motherboard
        }
    }
    
    // Draw CPU
    let cpu_width = 80;
    let cpu_height = 80;
    let cpu_x = mb_x + 50;
    let cpu_y = mb_y + 100;
    
    for y in cpu_y..cpu_y + cpu_height {
        for x in cpu_x..cpu_x + cpu_width {
            buffer[y * WIDTH + x] = 0x00AAAAAA; // Silver CPU
        }
    }
    draw_text(buffer, "CPU", (cpu_x + 25) as i32, (cpu_y + 40) as i32, 0xFF000000);
    
    // Draw RAM
    let ram_width = 200;
    let ram_height = 30;
    let ram_x = mb_x + 200;
    let ram_y = mb_y + 80;
    
    for y in ram_y..ram_y + ram_height {
        for x in ram_x..ram_x + ram_width {
            buffer[y * WIDTH + x] = 0x00222222; // Dark gray RAM
        }
    }
    draw_text(buffer, "RAM", (ram_x + 80) as i32, (ram_y + 15) as i32, 0xFFFFFFFF);
    
    // Draw SSD
    let ssd_width = 100;
    let ssd_height = 70;
    let ssd_x = mb_x + 300;
    let ssd_y = mb_y + 200;
    
    for y in ssd_y..ssd_y + ssd_height {
        for x in ssd_x..ssd_x + ssd_width {
            buffer[y * WIDTH + x] = 0x00444444; // Gray SSD
        }
    }
    draw_text(buffer, "SSD", (ssd_x + 35) as i32, (ssd_y + 35) as i32, 0xFFFFFFFF);
    
    // Draw Battery
    let bat_width = 150;
    let bat_height = 80;
    let bat_x = mb_x + 50;
    let bat_y = mb_y + 220;
    
    for y in bat_y..bat_y + bat_height {
        for x in bat_x..bat_x + bat_width {
            buffer[y * WIDTH + x] = 0x00FFFF00; // Yellow battery
        }
    }
    draw_text(buffer, "Battery", (bat_x + 45) as i32, (bat_y + 40) as i32, 0xFF000000);
    
    // Draw GPU
    let gpu_width = 120;
    let gpu_height = 60;
    let gpu_x = mb_x + 200;
    let gpu_y = mb_y + 150;
    
    for y in gpu_y..gpu_y + gpu_height {
        for x in gpu_x..gpu_x + gpu_width {
            buffer[y * WIDTH + x] = 0x00FF0000; // Red GPU
        }
    }
    draw_text(buffer, "GPU", (gpu_x + 45) as i32, (gpu_y + 30) as i32, 0xFFFFFFFF);
    
    draw_text(buffer, "Hardware View", ((WIDTH / 2) - 70) as i32, 30, 0xFFFFFFFF);
}

fn draw_laptop_software(buffer: &mut Vec<u32>) {
    // Draw operating system and software layers
    
    // Draw a layered view of the software stack
    let layer_width = 500;
    let layer_height = 40;
    let layer_x = (WIDTH - layer_width) / 2;
    let base_y = 100;
    
    // Hardware layer
    let hw_y = base_y + 5 * layer_height;
    for y in hw_y..hw_y + layer_height {
        for x in layer_x..layer_x + layer_width {
            buffer[y * WIDTH + x] = 0x00333333;
        }
    }
    draw_text(buffer, "Hardware", (layer_x + 200) as i32, (hw_y + 25) as i32, 0xFFFFFFFF);
    
    // Firmware/BIOS layer
    let fw_y = base_y + 4 * layer_height;
    for y in fw_y..fw_y + layer_height {
        for x in layer_x..layer_x + layer_width {
            buffer[y * WIDTH + x] = 0x00666666;
        }
    }
    draw_text(buffer, "Firmware/BIOS", (layer_x + 180) as i32, (fw_y + 25) as i32, 0xFFFFFFFF);
    
    // Kernel layer
    let kernel_y = base_y + 3 * layer_height;
    for y in kernel_y..kernel_y + layer_height {
        for x in layer_x..layer_x + layer_width {
            buffer[y * WIDTH + x] = 0x00996633;
        }
    }
    draw_text(buffer, "Operating System Kernel", (layer_x + 150) as i32, (kernel_y + 25) as i32, 0xFFFFFFFF);
    
    // System Services layer
    let sys_y = base_y + 2 * layer_height;
    for y in sys_y..sys_y + layer_height {
        for x in layer_x..layer_x + layer_width {
            buffer[y * WIDTH + x] = 0x00CC9966;
        }
    }
    draw_text(buffer, "System Services", (layer_x + 180) as i32, (sys_y + 25) as i32, 0xFFFFFFFF);
    
    // Applications layer
    let app_y = base_y + layer_height;
    for y in app_y..app_y + layer_height {
        for x in layer_x..layer_x + layer_width {
            buffer[y * WIDTH + x] = 0x0066CCFF;
        }
    }
    draw_text(buffer, "Applications", (layer_x + 190) as i32, (app_y + 25) as i32, 0xFF000000);
    
    // User Interface layer
    let ui_y = base_y;
    for y in ui_y..ui_y + layer_height {
        for x in layer_x..layer_x + layer_width {
            buffer[y * WIDTH + x] = 0x0099CCFF;
        }
    }
    draw_text(buffer, "User Interface", (layer_x + 180) as i32, (ui_y + 25) as i32, 0xFF000000);
    
    // Draw app icons in the Application layer
    draw_text(buffer, "Browser", (layer_x + 50) as i32, (app_y + 25) as i32, 0xFF000000);
    draw_text(buffer, "Email", (layer_x + 120) as i32, (app_y + 25) as i32, 0xFF000000);
    draw_text(buffer, "Photos", (layer_x + 260) as i32, (app_y + 25) as i32, 0xFF000000);
    draw_text(buffer, "Music", (layer_x + 320) as i32, (app_y + 25) as i32, 0xFF000000);
    draw_text(buffer, "Documents", (layer_x + 380) as i32, (app_y + 25) as i32, 0xFF000000);
    
    // Draw processes
    draw_text(buffer, "Running Processes:", 50, 300, 0xFFFFFFFF);
    draw_text(buffer, "- System", 70, 330, 0xFFAAAAAA);
    draw_text(buffer, "- User Interface", 70, 350, 0xFFAAAAAA);
    draw_text(buffer, "- Browser", 70, 370, 0xFFAAAAAA);
    draw_text(buffer, "- Background Services", 70, 390, 0xFFAAAAAA);
    
    // Draw memory usage
    draw_text(buffer, "Memory Usage:", 400, 300, 0xFFFFFFFF);
    
    // Draw memory bar
    let mem_width = 200;
    let mem_height = 20;
    let mem_x = 400;
    let mem_y = 330;
    
    for y in mem_y..mem_y + mem_height {
        for x in mem_x..mem_x + mem_width {
            buffer[y * WIDTH + x] = 0x00333333;
        }
    }
    
    // Draw used memory (60%)
    let used_width = (mem_width as f32 * 0.6) as usize;
    for y in mem_y..mem_y + mem_height {
        for x in mem_x..mem_x + used_width {
            buffer[y * WIDTH + x] = 0x0000CCFF;
        }
    }
    draw_text(buffer, "60% Used", (mem_x + 70) as i32, (mem_y + 15) as i32, 0xFF000000);
    
    draw_text(buffer, "Software View", ((WIDTH / 2) - 70) as i32, 30, 0xFFFFFFFF);
}