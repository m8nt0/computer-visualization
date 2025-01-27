use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

use crate::src::hardware::{CPU, GPU, MainMemory, Cache, DRAM, Bus};
use visualization::{VisualizationSystem, ViewMode};
use crate::visualization::hardware::cpu::CPU;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    // Initialize actual hardware components
    let mut bus = Bus::new();
    let mut cpu = CPU::new(&mut bus as *mut Bus);
    let mut gpu = GPU::new();
    let mut memory = MainMemory::new();
    let mut cache = Cache::new();
    let mut dram = DRAM::new();

    // Create visualization system with references to hardware
    let mut vis_system = VisualizationSystem::new(
        &cpu as *const CPU,
        &gpu as *const GPU,
        &memory as *const MainMemory,
        &cache as *const Cache,
        &dram as *const DRAM,
    );

    let mut window = Window::new(
        "Computer Hardware Visualization",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update hardware state
        if vis_system.is_powered() {
            cpu.tick();
            // Update other hardware components...
        }

        // Handle input
        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            vis_system.toggle_power();
        }
        if window.is_key_pressed(Key::Tab, minifb::KeyRepeat::No) {
            vis_system.switch_view(match vis_system.current_view() {
                ViewMode::Computer => ViewMode::Hardware,
                ViewMode::Hardware => ViewMode::Software,
                ViewMode::Software => ViewMode::Computer,
            });
        }

        // Handle mouse clicks
        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            if window.get_mouse_down(minifb::MouseButton::Left) {
                vis_system.handle_click(x as f32, y as f32);
            }
        }

        // Render visualization
        vis_system.render(&mut buffer, WIDTH, HEIGHT);

        // Update window
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(Duration::from_millis(16));
    }
}      