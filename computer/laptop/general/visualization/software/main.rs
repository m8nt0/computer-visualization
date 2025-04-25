// visualize the software components going through the hardware components like a soul going through the body.
//   

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use crate::hardware::main::Hardware;
use crate::software::main::Software;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

// Colors for software flow visualization
const INSTRUCTION_COLOR: u32 = 0x00FFFF;      // Cyan
const DATA_COLOR: u32 = 0xFFFF00;            // Yellow
const SIGNAL_COLOR: u32 = 0xFF00FF;          // Magenta
const OS_COLOR: u32 = 0x0088FF;              // Blue
const APP_COLOR: u32 = 0x00FF88;             // Teal
const DRIVER_COLOR: u32 = 0xFF8800;          // Orange
const UI_COLOR: u32 = 0x88FF00;              // Light Green
const BACKGROUND_COLOR: u32 = 0x101010;      // Very dark gray

// Hardware component positions (must match the hardware visualization)
struct ComponentPosition {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

pub struct SoftwareVisualization {
    buffer: Vec<u32>,
    window: Window,
    animation_time: f32,
    hardware: Option<Hardware>,
    software: Option<Software>,
    particle_system: ParticleSystem,
    cpu_pos: ComponentPosition,
    gpu_pos: ComponentPosition,
    memory_pos: ComponentPosition,
    storage_pos: ComponentPosition,
    display_pos: ComponentPosition,
    keyboard_pos: ComponentPosition,
    touchpad_pos: ComponentPosition,
    network_pos: ComponentPosition,
}

// Particle system to represent software "flowing" through hardware
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: u32,
    lifetime: f32,
    max_lifetime: f32,
    size: f32,
    type_id: u32, // 0 = instruction, 1 = data, 2 = signal, etc.
}

struct ParticleSystem {
    particles: Vec<Particle>,
    spawn_timer: f32,
}

impl ParticleSystem {
    fn new() -> Self {
        ParticleSystem {
            particles: Vec::with_capacity(1000),
            spawn_timer: 0.0,
        }
    }
    
    fn update(&mut self, dt: f32) {
        // Update existing particles
        let mut i = 0;
        while i < self.particles.len() {
            let particle = &mut self.particles[i];
            
            // Update position
            particle.x += particle.vx * dt;
            particle.y += particle.vy * dt;
            
            // Update lifetime
            particle.lifetime -= dt;
            
            // Remove dead particles
            if particle.lifetime <= 0.0 {
                self.particles.swap_remove(i);
            } else {
                i += 1;
            }
        }
        
        // Spawn timer for controlling flow rate
        self.spawn_timer -= dt;
    }
    
    fn spawn_particle(&mut self, x: f32, y: f32, vx: f32, vy: f32, color: u32, type_id: u32) {
        if self.spawn_timer <= 0.0 {
            let size = match type_id {
                0 => 2.0, // instruction
                1 => 3.0, // data
                _ => 2.5, // other
            };
            
            let lifetime = 4.0 + (rand() * 2.0); // 4-6 seconds
            
            self.particles.push(Particle {
                x,
                y,
                vx,
                vy,
                color,
                lifetime,
                max_lifetime: lifetime,
                size,
                type_id,
            });
            
            // Reset spawn timer
            self.spawn_timer = 0.01; // Adjust to control particle density
        }
    }
    
    fn render(&self, buffer: &mut [u32], width: usize, height: usize) {
        for particle in &self.particles {
            let x = particle.x as usize;
            let y = particle.y as usize;
            let size = particle.size as usize;
            
            // Adjust color based on lifetime (fade out)
            let alpha = particle.lifetime / particle.max_lifetime;
            let r = ((particle.color >> 16) & 0xFF) as f32 * alpha;
            let g = ((particle.color >> 8) & 0xFF) as f32 * alpha;
            let b = (particle.color & 0xFF) as f32 * alpha;
            
            let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            
            // Draw particle
            for dy in 0..size {
                for dx in 0..size {
                    if x + dx < width && y + dy < height {
                        buffer[(y + dy) * width + (x + dx)] = color;
                    }
                }
            }
        }
    }
    
    // Generate software flow from os to components
    fn generate_os_flow(&mut self, cpu_pos: &ComponentPosition) {
        // OS to CPU
        let os_x = WIDTH / 2;
        let os_y = 50;
        
        let cpu_center_x = cpu_pos.x + cpu_pos.width / 2;
        let cpu_center_y = cpu_pos.y + cpu_pos.height / 2;
        
        let dx = cpu_center_x as f32 - os_x as f32;
        let dy = cpu_center_y as f32 - os_y as f32;
        let dist = (dx * dx + dy * dy).sqrt();
        
        let vx = dx / dist * 100.0;
        let vy = dy / dist * 100.0;
        
        // Spawn OS instruction particles
        self.spawn_particle(os_x as f32, os_y as f32, vx, vy, OS_COLOR, 0);
    }
    
    // Generate flow between components
    fn generate_component_flows(&mut self, 
                               cpu_pos: &ComponentPosition, 
                               gpu_pos: &ComponentPosition,
                               memory_pos: &ComponentPosition,
                               storage_pos: &ComponentPosition) {
        // CPU to GPU
        let cpu_x = cpu_pos.x + cpu_pos.width / 2;
        let cpu_y = cpu_pos.y + cpu_pos.height / 2;
        
        let gpu_x = gpu_pos.x + gpu_pos.width / 2;
        let gpu_y = gpu_pos.y + gpu_pos.height / 2;
        
        let dx = gpu_x as f32 - cpu_x as f32;
        let dy = gpu_y as f32 - cpu_y as f32;
        let dist = (dx * dx + dy * dy).sqrt();
        
        let vx = dx / dist * 150.0;
        let vy = dy / dist * 150.0;
        
        if rand() < 0.3 {
            self.spawn_particle(cpu_x as f32, cpu_y as f32, vx, vy, INSTRUCTION_COLOR, 0);
        }
        
        // CPU to Memory
        let memory_x = memory_pos.x + memory_pos.width / 2;
        let memory_y = memory_pos.y + memory_pos.height / 2;
        
        let dx = memory_x as f32 - cpu_x as f32;
        let dy = memory_y as f32 - cpu_y as f32;
        let dist = (dx * dx + dy * dy).sqrt();
        
        let vx = dx / dist * 200.0;
        let vy = dy / dist * 200.0;
        
        if rand() < 0.4 {
            self.spawn_particle(cpu_x as f32, cpu_y as f32, vx, vy, DATA_COLOR, 1);
        }
        
        // CPU to Storage
        let storage_x = storage_pos.x + storage_pos.width / 2;
        let storage_y = storage_pos.y + storage_pos.height / 2;
        
        let dx = storage_x as f32 - cpu_x as f32;
        let dy = storage_y as f32 - cpu_y as f32;
        let dist = (dx * dx + dy * dy).sqrt();
        
        let vx = dx / dist * 120.0;
        let vy = dy / dist * 120.0;
        
        if rand() < 0.2 {
            self.spawn_particle(cpu_x as f32, cpu_y as f32, vx, vy, APP_COLOR, 2);
        }
    }
}

impl SoftwareVisualization {
    pub fn new() -> Result<Self, String> {
        let mut window = Window::new(
            "Software Flowing Through Hardware Visualization",
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        )
        .map_err(|e| format!("Failed to create window: {}", e))?;

        // Limit to max ~60 fps update rate
        window.limit_update_rate(Some(Duration::from_micros(16600)));

        // Component positions (same as in hardware visualization)
        let cpu_pos = ComponentPosition {
            x: (WIDTH / 2) - 100,
            y: HEIGHT - 120,
            width: 80,
            height: 80,
        };
        
        let gpu_pos = ComponentPosition {
            x: (WIDTH / 2) + 50,
            y: HEIGHT - 120,
            width: 100,
            height: 60,
        };
        
        let memory_pos = ComponentPosition {
            x: (WIDTH / 2) - 180,
            y: HEIGHT - 130,
            width: 30,
            height: 80,
        };
        
        let storage_pos = ComponentPosition {
            x: (WIDTH / 2) + 200,
            y: HEIGHT - 130,
            width: 70,
            height: 70,
        };
        
        let display_pos = ComponentPosition {
            x: (WIDTH / 2) - 220,
            y: HEIGHT - 220,
            width: 40,
            height: 40,
        };
        
        let keyboard_pos = ComponentPosition {
            x: (WIDTH / 2) - 50,
            y: HEIGHT - 230,
            width: 40,
            height: 30,
        };
        
        let touchpad_pos = ComponentPosition {
            x: (WIDTH / 2) + 10,
            y: HEIGHT - 230,
            width: 30,
            height: 30,
        };
        
        let network_pos = ComponentPosition {
            x: (WIDTH / 2) + 150,
            y: HEIGHT - 230,
            width: 50,
            height: 40,
        };

        Ok(SoftwareVisualization {
            buffer: vec![0; WIDTH * HEIGHT],
            window,
            animation_time: 0.0,
            hardware: None,
            software: None,
            particle_system: ParticleSystem::new(),
            cpu_pos,
            gpu_pos,
            memory_pos,
            storage_pos,
            display_pos,
            keyboard_pos,
            touchpad_pos,
            network_pos,
        })
    }

    pub fn connect_systems(&mut self, hardware: Hardware, software: Software) {
        println!("Connecting hardware and software to visualization...");
        self.hardware = Some(hardware);
        self.software = Some(software);
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut last_time = std::time::Instant::now();
        
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            // Calculate delta time
            let current_time = std::time::Instant::now();
            let dt = current_time.duration_since(last_time).as_secs_f32();
            last_time = current_time;
            
            // Update animation time
            self.animation_time += dt;
            
            // Clear buffer with background
            self.buffer.iter_mut().for_each(|pixel| *pixel = BACKGROUND_COLOR);
            
            // Render the hardware components (simplified version)
            self.render_hardware_outline();
            
            // Update particle system
            self.particle_system.update(dt);
            
            // Generate new particles
            self.generate_software_flow();
            
            // Render particles
            self.particle_system.render(&mut self.buffer, WIDTH, HEIGHT);
            
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

    fn render_hardware_outline(&mut self) {
        // Simplified outline of hardware components
        let component_color = 0x303030; // Dark gray
        
        // Draw CPU
        self.draw_component_outline(&self.cpu_pos, component_color);
        
        // Draw GPU
        self.draw_component_outline(&self.gpu_pos, component_color);
        
        // Draw Memory
        self.draw_component_outline(&self.memory_pos, component_color);
        
        // Draw Storage
        self.draw_component_outline(&self.storage_pos, component_color);
        
        // Draw Display controller
        self.draw_component_outline(&self.display_pos, component_color);
        
        // Draw Keyboard controller
        self.draw_component_outline(&self.keyboard_pos, component_color);
        
        // Draw Touchpad controller
        self.draw_component_outline(&self.touchpad_pos, component_color);
        
        // Draw Network
        self.draw_component_outline(&self.network_pos, component_color);
    }
    
    fn draw_component_outline(&mut self, component: &ComponentPosition, color: u32) {
        let x = component.x;
        let y = component.y;
        let width = component.width;
        let height = component.height;
        
        // Draw outline (just the edges)
        for i in 0..width {
            if x + i < WIDTH {
                // Top edge
                if y < HEIGHT {
                    self.buffer[y * WIDTH + x + i] = color;
                }
                // Bottom edge
                if y + height - 1 < HEIGHT {
                    self.buffer[(y + height - 1) * WIDTH + x + i] = color;
                }
            }
        }
        
        for i in 0..height {
            if y + i < HEIGHT {
                // Left edge
                if x < WIDTH {
                    self.buffer[(y + i) * WIDTH + x] = color;
                }
                // Right edge
                if x + width - 1 < WIDTH {
                    self.buffer[(y + i) * WIDTH + x + width - 1] = color;
                }
            }
        }
    }
    
    fn generate_software_flow(&mut self) {
        // OS to CPU flow
        self.particle_system.generate_os_flow(&self.cpu_pos);
        
        // Component to component flow
        self.particle_system.generate_component_flows(
            &self.cpu_pos, 
            &self.gpu_pos, 
            &self.memory_pos,
            &self.storage_pos
        );
        
        // Additional flows based on animation time
        let time_factor = self.animation_time % 10.0;
        
        if time_factor < 3.0 {
            // Heavy UI updates during this phase
            let cpu_x = self.cpu_pos.x + self.cpu_pos.width / 2;
            let cpu_y = self.cpu_pos.y + self.cpu_pos.height / 2;
            
            let display_x = self.display_pos.x + self.display_pos.width / 2;
            let display_y = self.display_pos.y + self.display_pos.height / 2;
            
            let dx = display_x as f32 - cpu_x as f32;
            let dy = display_y as f32 - cpu_y as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            
            let vx = dx / dist * 180.0;
            let vy = dy / dist * 180.0;
            
            if rand() < 0.5 {
                self.particle_system.spawn_particle(
                    cpu_x as f32, 
                    cpu_y as f32, 
                    vx, 
                    vy, 
                    UI_COLOR, 
                    2
                );
            }
        } else if time_factor >= 3.0 && time_factor < 6.0 {
            // Network activity during this phase
            let cpu_x = self.cpu_pos.x + self.cpu_pos.width / 2;
            let cpu_y = self.cpu_pos.y + self.cpu_pos.height / 2;
            
            let network_x = self.network_pos.x + self.network_pos.width / 2;
            let network_y = self.network_pos.y + self.network_pos.height / 2;
            
            let dx = network_x as f32 - cpu_x as f32;
            let dy = network_y as f32 - cpu_y as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            
            let vx = dx / dist * 200.0;
            let vy = dy / dist * 200.0;
            
            if rand() < 0.4 {
                self.particle_system.spawn_particle(
                    cpu_x as f32, 
                    cpu_y as f32, 
                    vx, 
                    vy, 
                    SIGNAL_COLOR, 
                    1
                );
            }
        } else {
            // Input device activity
            let cpu_x = self.cpu_pos.x + self.cpu_pos.width / 2;
            let cpu_y = self.cpu_pos.y + self.cpu_pos.height / 2;
            
            let keyboard_x = self.keyboard_pos.x + self.keyboard_pos.width / 2;
            let keyboard_y = self.keyboard_pos.y + self.keyboard_pos.height / 2;
            
            let dx = cpu_x as f32 - keyboard_x as f32;
            let dy = cpu_y as f32 - keyboard_y as f32;
            let dist = (dx * dx + dy * dy).sqrt();
            
            let vx = dx / dist * 150.0;
            let vy = dy / dist * 150.0;
            
            if rand() < 0.3 {
                self.particle_system.spawn_particle(
                    keyboard_x as f32, 
                    keyboard_y as f32, 
                    vx, 
                    vy, 
                    DRIVER_COLOR, 
                    0
                );
            }
        }
    }
}

// Simple random number function (0.0 to 1.0)
fn rand() -> f32 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (now % 10000) as f32 / 10000.0
}

pub fn run(hardware: Option<Hardware>, software: Option<Software>) -> Result<(), String> {
    println!("Starting software flow visualization...");
    let mut visualization = SoftwareVisualization::new()?;
    
    if let (Some(hw), Some(sw)) = (hardware, software) {
        visualization.connect_systems(hw, sw);
    }
    
    visualization.run()
}

// Entry point for standalone execution
pub fn main() -> Result<(), String> {
    // When run standalone, create new instances
    let hardware = Hardware::new();
    let mut software = Software::new();
    software.connect_to_hardware(hardware.clone());
    
    run(Some(hardware), Some(software))
}
