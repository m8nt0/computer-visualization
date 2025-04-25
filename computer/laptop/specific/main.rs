// seeing a specific already premade laptop visulaization for an exisitng laptop. 

// you get to choose out of many premade specific exisitng computer (i have not started one, but once opensourced people can spend time to do them using a common shared framework that i can build) you are going to study now:

// you see it like how you would see it in real life from first person view, its screen keyboard and hardware underneath the keyboard:

// you click power on, to turn it on the top right of the keyboard:

// when its on, you can click on the screen (software) or hardware (like its inside) view. most people would go to the screen, basically you are dealing with an OS, you see premade apps like there would be in typical laptops plus specifc exact things you would see in that laptops from software to hardware.

// you could choose to see when you click on something and you see the result; how you got to that point. 

// you would say teach me; and it would so deep one by one how the data went to the hardware and represenated itself on the screen. for every action, you could do this. 

// so it would have a src and visualization folder. we already have the folders in this dir but not working toghther. the src is for the actual source code, for example, in this case, its for figuring out how a speicifc laptop works and displaying it on the application (its software (os, application), hardware (cpu, and so on).) do you get it?

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

pub fn main(laptop_type: &str) {
    let window_title = format!("Specific Laptop Visualization - {}", laptop_type);
    
    let mut window = Window::new(
        &window_title,
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer
        for i in buffer.iter_mut() {
            *i = 0x00202020; // Dark gray background
        }

        if !power_on {
            draw_laptop_powered_off(&mut buffer, laptop_type);
            
            // Check for power button press
            if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
                power_on = true;
            }
        } else {
            match view_mode {
                ViewMode::External => draw_laptop_external(&mut buffer, laptop_type),
                ViewMode::Hardware => draw_laptop_hardware(&mut buffer, laptop_type),
                ViewMode::Software => draw_laptop_software(&mut buffer, laptop_type),
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
        let instructions = if !power_on {
            "Press 'P' to power on the laptop"
        } else {
            "Press 'Tab' to switch views, 'Escape' to exit"
        };
        draw_text(&mut buffer, instructions, 20, HEIGHT - 30, 0xFFFFFFFF);

        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

enum ViewMode {
    External,  // The laptop from outside (keyboard, screen, etc.)
    Hardware,  // Inside the laptop (components)
    Software,  // The laptop's operating system
}

fn draw_laptop_powered_off(buffer: &mut Vec<u32>, laptop_type: &str) {
    // Draw a simple laptop shape (closed) based on laptop type
    let laptop_width = 400;
    let laptop_height = 30;
    let laptop_x = (WIDTH - laptop_width) / 2;
    let laptop_y = HEIGHT / 2;
    
    // Different colors based on laptop type
    let laptop_color = match laptop_type {
        "Macbook Air" => 0x00C0C0C0, // Silver color
        "Windows 11" => 0x00606060,  // Dark gray
        "Linux Laptop" => 0x00404080, // Bluish
        _ => 0x00C0C0C0, // Default silver
    };
    
    // Draw the laptop base (bottom part)
    for y in laptop_y..laptop_y + laptop_height {
        for x in laptop_x..laptop_x + laptop_width {
            buffer[y * WIDTH + x] = laptop_color;
        }
    }
    
    draw_text(buffer, &format!("{} is powered off. Press 'P' to turn it on.", laptop_type), 
              200, 200, 0xFFFFFFFF);
}

fn draw_laptop_external(buffer: &mut Vec<u32>, laptop_type: &str) {
    // Draw a simple laptop shape (open)
    let base_width = 400;
    let base_height = 30;
    let screen_width = 380;
    let screen_height = 250;
    
    let base_x = (WIDTH - base_width) / 2;
    let base_y = HEIGHT / 2 + 70;
    let screen_x = (WIDTH - screen_width) / 2;
    let screen_y = base_y - screen_height - 10;
    
    // Different colors based on laptop type
    let (base_color, screen_color, keyboard_color) = match laptop_type {
        "Macbook Air" => (0x00C0C0C0, 0x00101030, 0x00B0B0B0), // Silver with dark blue screen
        "Windows 11" => (0x00606060, 0x00103060, 0x00505050),  // Dark gray with blue screen
        "Linux Laptop" => (0x00404080, 0x00080808, 0x00303060), // Bluish with almost black screen
        _ => (0x00C0C0C0, 0x00101040, 0x00B0B0B0), // Default silver
    };
    
    // Draw the laptop base (keyboard part)
    for y in base_y..base_y + base_height {
        for x in base_x..base_x + base_width {
            buffer[y * WIDTH + x] = base_color;
        }
    }
    
    // Draw the laptop screen
    for y in screen_y..screen_y + screen_height {
        for x in screen_x..screen_x + screen_width {
            // Screen bezels
            if y == screen_y || y == screen_y + screen_height - 1 || 
               x == screen_x || x == screen_x + screen_width - 1 {
                buffer[y * WIDTH + x] = base_color; // Bezel color
            } else {
                buffer[y * WIDTH + x] = screen_color; // Screen color
            }
        }
    }
    
    // Draw a power button
    let button_x = base_x + base_width - 30;
    let button_y = base_y + 5;
    for y in button_y..button_y + 10 {
        for x in button_x..button_x + 20 {
            buffer[y * WIDTH + x] = 0x00A0A0A0;
        }
    }
    
    draw_text(buffer, "Power", button_x, button_y + 2, 0x00000000);
    
    // Draw some fake keyboard keys
    for row in 0..4 {
        for col in 0..10 {
            let key_x = base_x + 20 + col * 35;
            let key_y = base_y + 5 + row * 6;
            for y in key_y..key_y + 5 {
                for x in key_x..key_x + 30 {
                    buffer[y * WIDTH + x] = keyboard_color;
                }
            }
        }
    }
    
    let brand_logo_x = screen_x + screen_width / 2 - 20;
    let brand_logo_y = screen_y + screen_height / 2;
    
    // Draw brand logo based on laptop type
    match laptop_type {
        "Macbook Air" => {
            // Draw Apple logo
            for y in brand_logo_y-10..brand_logo_y+10 {
                for x in brand_logo_x-10..brand_logo_x+10 {
                    if (x as i32 - brand_logo_x as i32).pow(2) + (y as i32 - brand_logo_y as i32).pow(2) < 100 {
                        buffer[y * WIDTH + x] = 0x00FFFFFF;
                    }
                }
            }
        },
        "Windows 11" => {
            // Draw Windows logo
            for y in brand_logo_y-10..brand_logo_y+10 {
                for x in brand_logo_x-10..brand_logo_x+10 {
                    if x >= brand_logo_x-10 && x < brand_logo_x && 
                       y >= brand_logo_y-10 && y < brand_logo_y {
                        buffer[y * WIDTH + x] = 0x000000FF; // Blue
                    } else if x >= brand_logo_x && x < brand_logo_x+10 && 
                              y >= brand_logo_y-10 && y < brand_logo_y {
                        buffer[y * WIDTH + x] = 0x0000FF00; // Green
                    } else if x >= brand_logo_x-10 && x < brand_logo_x && 
                              y >= brand_logo_y && y < brand_logo_y+10 {
                        buffer[y * WIDTH + x] = 0x00FF0000; // Red
                    } else if x >= brand_logo_x && x < brand_logo_x+10 && 
                              y >= brand_logo_y && y < brand_logo_y+10 {
                        buffer[y * WIDTH + x] = 0x00FFFF00; // Yellow
                    }
                }
            }
        },
        "Linux Laptop" => {
            // Draw Tux (penguin) logo
            for y in brand_logo_y-10..brand_logo_y+10 {
                for x in brand_logo_x-10..brand_logo_x+10 {
                    if (x as i32 - brand_logo_x as i32).pow(2) + (y as i32 - brand_logo_y as i32).pow(2) < 100 {
                        if y < brand_logo_y {
                            buffer[y * WIDTH + x] = 0x00000000; // Black
                        } else {
                            buffer[y * WIDTH + x] = 0x00FFFFFF; // White
                        }
                    }
                }
            }
        },
        _ => {}
    }
    
    draw_text(buffer, &format!("External View of {} - Press Tab to see Hardware", laptop_type), 
              180, 150, 0xFFFFFFFF);
}

fn draw_laptop_hardware(buffer: &mut Vec<u32>, laptop_type: &str) {
    // Draw a representation of laptop internals based on laptop type
    let motherboard_width = 400;
    let motherboard_height = 250;
    let motherboard_x = (WIDTH - motherboard_width) / 2;
    let motherboard_y = (HEIGHT - motherboard_height) / 2;
    
    // Different motherboard colors based on laptop type
    let motherboard_color = match laptop_type {
        "Macbook Air" => 0x00004000, // Dark green for Apple
        "Windows 11" => 0x00000050,  // Dark blue for Windows
        "Linux Laptop" => 0x00400000, // Dark red for Linux
        _ => 0x00005000, // Default green
    };
    
    // Draw the motherboard
    for y in motherboard_y..motherboard_y + motherboard_height {
        for x in motherboard_x..motherboard_x + motherboard_width {
            buffer[y * WIDTH + x] = motherboard_color;
        }
    }
    
    // Draw CPU with different specs based on laptop type
    let cpu_x = motherboard_x + 50;
    let cpu_y = motherboard_y + 50;
    let cpu_size = 70;
    for y in cpu_y..cpu_y + cpu_size {
        for x in cpu_x..cpu_x + cpu_size {
            buffer[y * WIDTH + x] = 0x00909090; // Gray CPU
        }
    }
    
    let cpu_label = match laptop_type {
        "Macbook Air" => "M1 Chip",
        "Windows 11" => "Intel i7",
        "Linux Laptop" => "AMD Ryzen",
        _ => "CPU",
    };
    draw_text(buffer, cpu_label, cpu_x + 15, cpu_y + 30, 0x00000000);
    
    // Draw GPU
    let gpu_x = motherboard_x + 150;
    let gpu_y = motherboard_y + 50;
    let gpu_width = 100;
    let gpu_height = 70;
    for y in gpu_y..gpu_y + gpu_height {
        for x in gpu_x..gpu_x + gpu_width {
            buffer[y * WIDTH + x] = 0x00707070; // Darker gray GPU
        }
    }
    
    let gpu_label = match laptop_type {
        "Macbook Air" => "Integrated",
        "Windows 11" => "NVIDIA RTX",
        "Linux Laptop" => "AMD Radeon",
        _ => "GPU",
    };
    draw_text(buffer, gpu_label, gpu_x + 15, gpu_y + 30, 0x00000000);
    
    // Draw RAM sticks
    let ram_x = motherboard_x + 50;
    let ram_y = motherboard_y + 150;
    let ram_width = 200;
    let ram_height = 20;
    for y in ram_y..ram_y + ram_height {
        for x in ram_x..ram_x + ram_width {
            buffer[y * WIDTH + x] = 0x00000080; // Blue RAM
        }
    }
    
    let ram_label = match laptop_type {
        "Macbook Air" => "8GB Unified",
        "Windows 11" => "16GB DDR4",
        "Linux Laptop" => "32GB DDR4",
        _ => "RAM",
    };
    draw_text(buffer, ram_label, ram_x + 60, ram_y + 5, 0x00FFFFFF);
    
    // Draw SSD
    let ssd_x = motherboard_x + 280;
    let ssd_y = motherboard_y + 150;
    let ssd_width = 80;
    let ssd_height = 40;
    for y in ssd_y..ssd_y + ssd_height {
        for x in ssd_x..ssd_x + ssd_width {
            buffer[y * WIDTH + x] = 0x00A0A0A0; // Gray SSD
        }
    }
    
    let ssd_label = match laptop_type {
        "Macbook Air" => "256GB",
        "Windows 11" => "1TB",
        "Linux Laptop" => "512GB",
        _ => "SSD",
    };
    draw_text(buffer, ssd_label, ssd_x + 20, ssd_y + 15, 0x00000000);
    
    draw_text(buffer, &format!("Hardware View of {} - Press Tab to see Software", laptop_type), 
              160, 150, 0xFFFFFFFF);
}

fn draw_laptop_software(buffer: &mut Vec<u32>, laptop_type: &str) {
    // Draw OS desktop based on laptop type
    
    // Different desktop appearance based on laptop type
    let (bg_color, taskbar_color, taskbar_position) = match laptop_type {
        "Macbook Air" => (0x00256588, 0x00303030, TaskbarPosition::Top),    // macOS
        "Windows 11" => (0x00103060, 0x00404040, TaskbarPosition::Bottom),  // Windows
        "Linux Laptop" => (0x00333333, 0x00404040, TaskbarPosition::Left),  // Linux
        _ => (0x00103060, 0x00404040, TaskbarPosition::Bottom),             // Default
    };
    
    // Draw desktop background
    for i in buffer.iter_mut() {
        *i = bg_color;
    }
    
    // Draw taskbar
    match taskbar_position {
        TaskbarPosition::Top => {
            // macOS taskbar at top
            for y in 0..40 {
                for x in 0..WIDTH {
                    buffer[y * WIDTH + x] = taskbar_color;
                }
            }
            
            // macOS apple logo
            draw_text(buffer, "", 20, 15, 0xFFFFFFFF);
            
            // macOS menu items
            draw_text(buffer, "Finder   File   Edit   View   Go   Window   Help", 60, 15, 0xFFFFFFFF);
        },
        TaskbarPosition::Bottom => {
            // Windows taskbar at bottom
            for y in HEIGHT - 40..HEIGHT {
                for x in 0..WIDTH {
                    buffer[y * WIDTH + x] = taskbar_color;
                }
            }
            
            // Windows start button
            let start_x = 10;
            let start_y = HEIGHT - 35;
            let start_width = 80;
            let start_height = 30;
            for y in start_y..start_y + start_height {
                for x in start_x..start_x + start_width {
                    buffer[y * WIDTH + x] = 0x00606060;
                }
            }
            draw_text(buffer, "Start", start_x + 20, start_y + 10, 0xFFFFFFFF);
        },
        TaskbarPosition::Left => {
            // Linux taskbar at left
            for y in 0..HEIGHT {
                for x in 0..60 {
                    buffer[y * WIDTH + x] = taskbar_color;
                }
            }
            
            // Linux menu button
            let start_y = 10;
            let start_width = 40;
            let start_height = 40;
            for y in start_y..start_y + start_height {
                for x in 10..10 + start_width {
                    buffer[y * WIDTH + x] = 0x00606060;
                }
            }
            draw_text(buffer, "Menu", 12, start_y + 15, 0xFFFFFFFF);
        },
    }
    
    // Draw desktop icons based on OS
    let icons_start_x = match taskbar_position {
        TaskbarPosition::Left => 80,
        _ => 30,
    };
    
    let icons_start_y = match taskbar_position {
        TaskbarPosition::Top => 60,
        _ => 30,
    };
    
    let icon_size = 50;
    let icon_spacing = 80;
    
    match laptop_type {
        "Macbook Air" => {
            // macOS icons
            // Finder
            for y in icons_start_y..icons_start_y + icon_size {
                for x in icons_start_x..icons_start_x + icon_size {
                    buffer[y * WIDTH + x] = 0x000080FF;
                }
            }
            draw_text(buffer, "Finder", icons_start_x, icons_start_y + icon_size + 5, 0xFFFFFFFF);
            
            // Safari
            for y in icons_start_y..icons_start_y + icon_size {
                for x in icons_start_x + icon_spacing..icons_start_x + icon_spacing + icon_size {
                    buffer[y * WIDTH + x] = 0x004080FF;
                }
            }
            draw_text(buffer, "Safari", icons_start_x + icon_spacing, icons_start_y + icon_size + 5, 0xFFFFFFFF);
            
            // Photos
            for y in icons_start_y..icons_start_y + icon_size {
                for x in icons_start_x + icon_spacing * 2..icons_start_x + icon_spacing * 2 + icon_size {
                    buffer[y * WIDTH + x] = 0x00FF6000;
                }
            }
            draw_text(buffer, "Photos", icons_start_x + icon_spacing * 2, icons_start_y + icon_size + 5, 0xFFFFFFFF);
        },
        "Windows 11" => {
            // Windows icons
            // File Explorer
            for y in icons_start_y..icons_start_y + icon_size {
                for x in icons_start_x..icons_start_x + icon_size {
                    buffer[y * WIDTH + x] = 0x00FFFF00;
                }
            }
            draw_text(buffer, "Explorer", icons_start_x, icons_start_y + icon_size + 5, 0xFFFFFFFF);
            
            // Edge
            for y in icons_start_y..icons_start_y + icon_size {
                for x in icons_start_x + icon_spacing..icons_start_x + icon_spacing + icon_size {
                    buffer[y * WIDTH + x] = 0x000080FF;
                }
            }
            draw_text(buffer, "Edge", icons_start_x + icon_spacing, icons_start_y + icon_size + 5, 0xFFFFFFFF);
            
            // Settings
            for y in icons_start_y..icons_start_y + icon_size {
                for x in icons_start_x + icon_spacing * 2..icons_start_x + icon_spacing * 2 + icon_size {
                    buffer[y * WIDTH + x] = 0x00C0C0C0;
                }
            }
            draw_text(buffer, "Settings", icons_start_x + icon_spacing * 2, icons_start_y + icon_size + 5, 0xFFFFFFFF);
        },
        "Linux Laptop" => {
            // Linux icons
            // Files
            for y in icons_start_y..icons_start_y + icon_size {
                for x in icons_start_x..icons_start_x + icon_size {
                    buffer[y * WIDTH + x] = 0x00FFFF00;
                }
            }
            draw_text(buffer, "Files", icons_start_x, icons_start_y + icon_size + 5, 0xFFFFFFFF);
            
            // Firefox
            for y in icons_start_y..icons_start_y + icon_size {
                for x in icons_start_x + icon_spacing..icons_start_x + icon_spacing + icon_size {
                    buffer[y * WIDTH + x] = 0x00FF4000;
                }
            }
            draw_text(buffer, "Firefox", icons_start_x + icon_spacing, icons_start_y + icon_size + 5, 0xFFFFFFFF);
            
            // Terminal
            for y in icons_start_y..icons_start_y + icon_size {
                for x in icons_start_x + icon_spacing * 2..icons_start_x + icon_spacing * 2 + icon_size {
                    buffer[y * WIDTH + x] = 0x00000000;
                }
            }
            draw_text(buffer, "Terminal", icons_start_x + icon_spacing * 2, icons_start_y + icon_size + 5, 0xFFFFFFFF);
        },
        _ => {
            // Generic icons
            for y in icons_start_y..icons_start_y + icon_size {
                for x in icons_start_x..icons_start_x + icon_size {
                    buffer[y * WIDTH + x] = 0x00FFFFFF;
                }
            }
            draw_text(buffer, "Files", icons_start_x, icons_start_y + icon_size + 5, 0xFFFFFFFF);
        }
    }
    
    draw_text(buffer, &format!("Software View of {} - Press Tab to see External View", laptop_type), 
              160, 300, 0xFFFFFFFF);
}

enum TaskbarPosition {
    Top,
    Bottom,
    Left,
}

fn draw_text(buffer: &mut Vec<u32>, text: &str, x: usize, y: usize, color: u32) {
    // Simple text rendering function
    let char_width = 8;
    let char_height = 12;

    for (i, c) in text.chars().enumerate() {
        if c == ' ' {
            continue;
        }

        let char_x = x + (i * char_width);
        
        // Draw a simple rectangle for each character
        for cy in 0..char_height {
            for cx in 0..char_width {
                let px = char_x + cx;
                let py = y + cy;
                
                if px < WIDTH && py < HEIGHT {
                    let idx = py * WIDTH + px;
                    buffer[idx] = color;
                }
            }
        }
    }
}

