//here you create visualiaztion that allows users to select which type of sub computer they want. 

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::process::Command;


const WIDTH: usize = 800;
const HEIGHT: usize = 600;

enum ComputerType {
    Desktop,
    Laptop,
    Server,
    Smartphone,
    Tablet,
}

struct ComputerSelection {
    selected: ComputerType,
}

fn main() {
    let mut window = Window::new(
        "Computer Architecture Visualization",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut selection = ComputerSelection {
        selected: ComputerType::Laptop, // Default selection
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer
        for i in buffer.iter_mut() {
            *i = 0x00101030; // Dark blue background
        }

        // Draw the selection interface
        draw_interface(&mut buffer, &selection);

        // Handle input
        if window.is_key_pressed(Key::Right, minifb::KeyRepeat::No) {
            selection.selected = match selection.selected {
                ComputerType::Desktop => ComputerType::Laptop,
                ComputerType::Laptop => ComputerType::Server,
                ComputerType::Server => ComputerType::Smartphone,
                ComputerType::Smartphone => ComputerType::Tablet,
                ComputerType::Tablet => ComputerType::Desktop,
            };
        }

        if window.is_key_pressed(Key::Left, minifb::KeyRepeat::No) {
            selection.selected = match selection.selected {
                ComputerType::Desktop => ComputerType::Tablet,
                ComputerType::Laptop => ComputerType::Desktop,
                ComputerType::Server => ComputerType::Laptop,
                ComputerType::Smartphone => ComputerType::Server,
                ComputerType::Tablet => ComputerType::Smartphone,
            };
        }

        if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
            match selection.selected {
                ComputerType::Laptop => {
                    // Launch the laptop visualization
                    launch_laptop_visualization();
                    break;
                }
                _ => {
                    // Other computer types (not implemented yet)
                    draw_not_implemented_message(&mut buffer);
                }
            }
        }

        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_interface(buffer: &mut Vec<u32>, selection: &ComputerSelection) {
    // Draw title
    draw_text(buffer, "Select Computer Type", 250, 100, 0xFFFFFFFF);

    // Draw computer options in a circular arrangement
    let center_x = WIDTH as i32 / 2;
    let center_y = HEIGHT as i32 / 2;
    let radius = 150;

    // Draw Desktop option
    let desktop_x = center_x;
    let desktop_y = center_y - radius;
    let desktop_color = if matches!(selection.selected, ComputerType::Desktop) {
        0xFFFFFFFF // White if selected
    } else {
        0xFFAAAAAA // Light gray if not selected
    };
    draw_text(buffer, "Desktop", desktop_x - 40, desktop_y, desktop_color);

    // Draw Laptop option
    let laptop_x = center_x + radius;
    let laptop_y = center_y;
    let laptop_color = if matches!(selection.selected, ComputerType::Laptop) {
        0xFFFFFFFF
    } else {
        0xFFAAAAAA
    };
    draw_text(buffer, "Laptop", laptop_x - 30, laptop_y, laptop_color);

    // Draw Server option
    let server_x = center_x;
    let server_y = center_y + radius;
    let server_color = if matches!(selection.selected, ComputerType::Server) {
        0xFFFFFFFF
    } else {
        0xFFAAAAAA
    };
    draw_text(buffer, "Server", server_x - 30, server_y, server_color);

    // Draw Smartphone option
    let smartphone_x = center_x - radius;
    let smartphone_y = center_y;
    let smartphone_color = if matches!(selection.selected, ComputerType::Smartphone) {
        0xFFFFFFFF
    } else {
        0xFFAAAAAA
    };
    draw_text(buffer, "Smartphone", smartphone_x - 60, smartphone_y, smartphone_color);

    // Draw Tablet option
    let tablet_x = center_x - (radius * 707 / 1000);
    let tablet_y = center_y - (radius * 707 / 1000);
    let tablet_color = if matches!(selection.selected, ComputerType::Tablet) {
        0xFFFFFFFF
    } else {
        0xFFAAAAAA
    };
    draw_text(buffer, "Tablet", tablet_x - 30, tablet_y, tablet_color);

    // Draw instructions
    draw_text(buffer, "Use Left/Right arrows to select, Enter to confirm", 180, (HEIGHT - 50) as i32, 0xFFFFFFFF);
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
        '-' => y == 6 && x >= 1 && x <= 7,
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

fn draw_not_implemented_message(buffer: &mut Vec<u32>) {
    draw_text(buffer, "This computer type is not implemented yet.", 200, HEIGHT as i32 / 2, 0xFFFFFFFF);
    std::thread::sleep(Duration::from_secs(2));
}

fn launch_laptop_visualization() {
    println!("Launching laptop visualization...");
    
    // Use cargo run to launch the laptop visualization
    if cfg!(target_os = "windows") {
        Command::new("cargo.exe")
            .args(["run", "--bin", "laptop-visualization"])
            .spawn()
            .expect("Failed to launch laptop visualization");
    } else {
        Command::new("cargo")
            .args(["run", "--bin", "laptop-visualization"])
            .spawn()
            .expect("Failed to launch laptop visualization");
    }
}

