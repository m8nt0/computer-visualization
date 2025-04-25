// basically create a space where a user can select what type of laptop they want to visualize (general, specifc, your-personal).

//if they select general, it shows them a general way laptops work (that is general across all laptop exisitng, and better). 

//if they select specific (specific type of laptops, premade basically), they get options of speicic laptop to select from, like macbook air something, windows 11 something and so on.

//if they select your-personal, it shows how your currently using laptop works. 

//here you create visualiaztion that allows users to select which type of sub computer they want. 

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

enum LaptopVisualizationType {
    General,
    Specific,
    Personal,
}

enum SpecificLaptopType {
    MacbookAir,
    Windows11,
    LinuxLaptop,
}

struct LaptopSelection {
    visualization_type: LaptopVisualizationType,
    specific_type: Option<SpecificLaptopType>,
}

pub fn main() {
    let mut window = Window::new(
        "Laptop Visualization Selection",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut selection = LaptopSelection {
        visualization_type: LaptopVisualizationType::General,
        specific_type: None,
    };
    let mut specific_selection_active = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Clear the buffer
        for i in buffer.iter_mut() {
            *i = 0x00000000; // Black color
        }

        if !specific_selection_active {
            // Draw the visualization type selection interface
            draw_visualization_interface(&mut buffer, &selection);

            // Handle input for visualization type
            if window.is_key_pressed(Key::Down, minifb::KeyRepeat::No) {
                selection.visualization_type = match selection.visualization_type {
                    LaptopVisualizationType::General => LaptopVisualizationType::Specific,
                    LaptopVisualizationType::Specific => LaptopVisualizationType::YourPersonal,
                    LaptopVisualizationType::YourPersonal => LaptopVisualizationType::General,
                };
            }

            if window.is_key_pressed(Key::Up, minifb::KeyRepeat::No) {
                selection.visualization_type = match selection.visualization_type {
                    LaptopVisualizationType::General => LaptopVisualizationType::YourPersonal,
                    LaptopVisualizationType::Specific => LaptopVisualizationType::General,
                    LaptopVisualizationType::YourPersonal => LaptopVisualizationType::Specific,
                };
            }

            if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
                match selection.visualization_type {
                    LaptopVisualizationType::General => {
                        // Launch general laptop visualization
                        launch_general_visualization();
                        break;
                    }
                    LaptopVisualizationType::Specific => {
                        // Show specific laptop selection
                        specific_selection_active = true;
                        selection.specific_type = Some(SpecificLaptopType::MacbookAir);
                    }
                    LaptopVisualizationType::YourPersonal => {
                        // Launch personal laptop visualization
                        launch_personal_visualization();
                        break;
                    }
                }
            }
        } else {
            // Draw the specific laptop selection interface
            draw_specific_interface(&mut buffer, &selection);

            // Handle input for specific laptop type
            if window.is_key_pressed(Key::Right, minifb::KeyRepeat::No) {
                selection.specific_type = match selection.specific_type {
                    Some(SpecificLaptopType::MacbookAir) => Some(SpecificLaptopType::Windows11),
                    Some(SpecificLaptopType::Windows11) => Some(SpecificLaptopType::LinuxLaptop),
                    Some(SpecificLaptopType::LinuxLaptop) => Some(SpecificLaptopType::MacbookAir),
                    None => Some(SpecificLaptopType::MacbookAir),
                };
            }

            if window.is_key_pressed(Key::Left, minifb::KeyRepeat::No) {
                selection.specific_type = match selection.specific_type {
                    Some(SpecificLaptopType::MacbookAir) => Some(SpecificLaptopType::LinuxLaptop),
                    Some(SpecificLaptopType::Windows11) => Some(SpecificLaptopType::MacbookAir),
                    Some(SpecificLaptopType::LinuxLaptop) => Some(SpecificLaptopType::Windows11),
                    None => Some(SpecificLaptopType::MacbookAir),
                };
            }

            if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
                match selection.specific_type {
                    Some(SpecificLaptopType::MacbookAir) => {
                        launch_specific_visualization("Macbook Air");
                        break;
                    }
                    Some(SpecificLaptopType::Windows11) => {
                        launch_specific_visualization("Windows 11");
                        break;
                    }
                    Some(SpecificLaptopType::LinuxLaptop) => {
                        launch_specific_visualization("Linux Laptop");
                        break;
                    }
                    None => {}
                }
            }

            if window.is_key_pressed(Key::Backspace, minifb::KeyRepeat::No) {
                specific_selection_active = false;
            }
        }

        // Update the window
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_visualization_interface(buffer: &mut Vec<u32>, selection: &LaptopSelection) {
    // Set background color (dark green)
    for i in buffer.iter_mut() {
        *i = 0x00103010;
    }

    // Draw title
    draw_text(buffer, "Select Laptop Visualization Type", 200, 100, 0xFFFFFFFF);

    // Calculate positions
    let center_x = WIDTH / 2;
    let spacing = 50;
    let start_y = HEIGHT / 2 - spacing;

    // Draw General option
    let general_color = match selection.visualization_type {
        LaptopVisualizationType::General => 0xFFFFFFFF, // White if selected
        _ => 0xFFAAAAAA,                              // Light gray if not selected
    };
    draw_text(buffer, "General Laptop", center_x - 60, start_y, general_color);

    // Draw Specific option
    let specific_color = match selection.visualization_type {
        LaptopVisualizationType::Specific => 0xFFFFFFFF,
        _ => 0xFFAAAAAA,
    };
    draw_text(buffer, "Specific Laptop Models", center_x - 100, start_y + spacing, specific_color);

    // Draw Your Personal option
    let personal_color = match selection.visualization_type {
        LaptopVisualizationType::YourPersonal => 0xFFFFFFFF,
        _ => 0xFFAAAAAA,
    };
    draw_text(buffer, "Your Personal Laptop", center_x - 90, start_y + spacing * 2, personal_color);

    // Draw instructions
    draw_text(buffer, "Use Up/Down arrows to select, Enter to confirm", 180, HEIGHT - 50, 0xFFFFFFFF);
}

fn draw_specific_interface(buffer: &mut Vec<u32>, selection: &LaptopSelection) {
    // Set background color (dark blue-green)
    for i in buffer.iter_mut() {
        *i = 0x00103040;
    }

    // Draw title
    draw_text(buffer, "Select Specific Laptop Model", 200, 100, 0xFFFFFFFF);

    // Calculate positions
    let center_x = WIDTH / 2;
    let spacing = 100;
    let start_x = center_x - spacing;
    let y = HEIGHT / 2;

    // Draw Macbook Air option
    let macbook_color = match selection.specific_type {
        Some(SpecificLaptopType::MacbookAir) => 0xFFFFFFFF,
        _ => 0xFFAAAAAA,
    };
    draw_text(buffer, "Macbook Air", start_x - 50, y, macbook_color);

    // Draw Windows 11 option
    let windows_color = match selection.specific_type {
        Some(SpecificLaptopType::Windows11) => 0xFFFFFFFF,
        _ => 0xFFAAAAAA,
    };
    draw_text(buffer, "Windows 11", center_x - 50, y, windows_color);

    // Draw Linux Laptop option
    let linux_color = match selection.specific_type {
        Some(SpecificLaptopType::LinuxLaptop) => 0xFFFFFFFF,
        _ => 0xFFAAAAAA,
    };
    draw_text(buffer, "Linux Laptop", start_x + spacing * 2 - 50, y, linux_color);

    // Draw instructions
    draw_text(buffer, "Use Left/Right arrows to select, Enter to confirm, Backspace to go back", 100, HEIGHT - 50, 0xFFFFFFFF);
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

fn launch_general_visualization() {
    println!("Launching general laptop visualization...");
    // Here we would call into the general laptop visualization module
    // general::main();
}

fn launch_specific_visualization(laptop_type: &str) {
    println!("Launching specific laptop visualization for: {}", laptop_type);
    // Here we would call into the specific laptop visualization module
    // specific::main(laptop_type);
}

fn launch_personal_visualization() {
    println!("Launching your personal laptop visualization...");
    // Here we would call into the personal laptop visualization module
    // your_personal::main();
}

