// connect all the hardware modules/folders basically what make up the laptop work, into a coherent whole.
// This is the motherboard that connects all components together
// and provides a cohesive hardware platform for software to run on.

use crate::hardware::bus::Bus;
use crate::hardware::cooling::CoolingSystem;
use crate::hardware::memory::ram::RAM;
use crate::hardware::peripherals::input::keyboard::Keyboard;
use crate::hardware::peripherals::input::touchpad::Touchpad;
use crate::hardware::peripherals::output::display::Display;
use crate::hardware::power::PowerManagement;
use crate::hardware::processing::cpu::CPU;
use crate::hardware::processing::gpu::GPU;
use crate::hardware::storage::ssd::SSD;

// The Hardware struct represents a complete computer system
// It's essentially the motherboard that connects all components
pub struct Hardware {
    // System bus
    bus: Bus,
    
    // Processing units
    cpu: CPU,
    gpu: GPU,
    
    // Memory subsystem
    ram: RAM,
    
    // Storage
    storage: SSD,
    
    // Input/Output devices
    display: Display,
    keyboard: Keyboard,
    touchpad: Touchpad,
    
    // Power management
    power: PowerManagement,
    
    // Cooling system
    cooling: CoolingSystem,
    
    // System state
    is_powered_on: bool,
}

impl Hardware {
    pub fn new() -> Self {
        let bus = Bus::new();
        
        Self {
            bus,
            cpu: CPU::new(),
            gpu: GPU::new(),
            ram: RAM::new(),
            storage: SSD::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
            touchpad: Touchpad::new(),
            power: PowerManagement::new(),
            cooling: CoolingSystem::new(),
            is_powered_on: false,
        }
    }
    
    pub fn initialize(&mut self) -> bool {
        if self.is_powered_on {
            println!("Hardware is already powered on.");
            return true;
        }
        
        println!("Initializing hardware components...");
        
        // Power management must be initialized first
        if !self.power.initialize() {
            println!("Failed to initialize power management system!");
            return false;
        }
        
        // Memory subsystem
        if !self.ram.initialize() {
            println!("Failed to initialize memory!");
            self.power.emergency_shutdown();
            return false;
        }
        
        // Processing units
        if !self.cpu.initialize() {
            println!("Failed to initialize CPU!");
            self.power.emergency_shutdown();
            return false;
        }
        
        if !self.gpu.initialize() {
            println!("Failed to initialize GPU!");
            // GPU failure isn't critical, we can continue
            println!("Continuing with integrated graphics...");
        }
        
        // Storage
        if !self.storage.initialize() {
            println!("Failed to initialize storage!");
            self.power.emergency_shutdown();
            return false;
        }
        
        // I/O devices
        self.display.initialize();
        self.keyboard.initialize();
        self.touchpad.initialize();
        
        // Cooling system
        self.cooling.initialize();
        
        self.is_powered_on = true;
        println!("All hardware components initialized successfully.");
        
        true
    }
    
    pub fn shutdown(&mut self) -> bool {
        if !self.is_powered_on {
            println!("Hardware is already powered off.");
            return true;
        }
        
        println!("Shutting down hardware components...");
        
        // Shutdown in reverse order of initialization
        // I/O devices first
        self.display.shutdown();
        self.keyboard.shutdown();
        self.touchpad.shutdown();
        
        // Processing units
        self.gpu.shutdown();
        self.cpu.shutdown();
        
        // Storage
        self.storage.shutdown();
        
        // Memory
        self.ram.shutdown();
        
        // Cooling system
        self.cooling.shutdown();
        
        // Power management last
        self.power.shutdown();
        
        self.is_powered_on = false;
        println!("All hardware components shut down successfully.");
        
        true
    }
    
    // Process software instructions that flow through the hardware
    pub fn process_instructions(&mut self, instructions: &[u8]) -> Vec<u8> {
        if !self.is_powered_on {
            return vec![0xFF]; // Error code - hardware not initialized
        }
        
        // First, let the CPU process the instruction
        let processed_data = self.cpu.process(instructions);
        
        // Based on the instruction type, route to other components
        // In a real system, this would be handled by the memory controller and bus
        let response = match instructions.first() {
            // Memory operation
            Some(0x01) => self.handle_memory_operation(&processed_data),
            
            // Storage operation
            Some(0x02) => self.handle_storage_operation(&processed_data),
            
            // Display operation
            Some(0x03) => self.handle_display_operation(&processed_data),
            
            // Input device query
            Some(0x04) => self.handle_input_device_query(&processed_data),
            
            // GPU operation (rendering, compute)
            Some(0x05) => self.handle_gpu_operation(&processed_data),
            
            // Power state change
            Some(0x06) => self.handle_power_state_change(&processed_data),
            
            // System status query
            Some(0x07) => self.get_system_status_data(),
            
            // Unknown instruction
            _ => vec![0xFF], // Error code - unknown instruction
        };
        
        // Update thermal state
        self.update_thermal_state();
        
        response
    }
    
    // Handle different types of operations
    fn handle_memory_operation(&mut self, data: &[u8]) -> Vec<u8> {
        if data.len() < 3 {
            return vec![0xFF]; // Error - insufficient data
        }
        
        let operation = data[1];
        let address = data[2] as usize;
        
        match operation {
            // Read operation
            0x01 => {
                if data.len() < 4 {
                    return vec![0xFF]; // Error - insufficient data
                }
                let size = data[3] as usize;
                self.ram.read(address, size)
            },
            
            // Write operation
            0x02 => {
                if data.len() < 4 {
                    return vec![0xFF]; // Error - insufficient data
                }
                
                if self.ram.write(address, &data[3..]) {
                    vec![0x00] // Success
                } else {
                    vec![0xFF] // Error
                }
            },
            
            // Allocate memory
            0x03 => {
                if data.len() < 4 {
                    return vec![0xFF]; // Error - insufficient data
                }
                
                let size = ((data[2] as usize) << 8) | (data[3] as usize);
                match self.ram.allocate(size) {
                    Some(addr) => {
                        let mut response = vec![0x00]; // Success
                        // Return the allocated address
                        response.extend_from_slice(&(addr as u32).to_be_bytes());
                        response
                    },
                    None => vec![0xFF], // Error - allocation failed
                }
            },
            
            // Free memory
            0x04 => {
                if data.len() < 5 {
                    return vec![0xFF]; // Error - insufficient data
                }
                
                let address = ((data[2] as usize) << 8) | (data[3] as usize);
                let size = data[4] as usize;
                
                if self.ram.free(address, size) {
                    vec![0x00] // Success
                } else {
                    vec![0xFF] // Error
                }
            },
            
            // Unknown memory operation
            _ => vec![0xFF], // Error - unknown operation
        }
    }
    
    fn handle_storage_operation(&mut self, data: &[u8]) -> Vec<u8> {
        // Process storage operations (simplified)
        // In a real system, this would involve complex disk controllers
        vec![0x00] // Placeholder
    }
    
    fn handle_display_operation(&mut self, data: &[u8]) -> Vec<u8> {
        // Process display operations (simplified)
        // In a real system, this would involve the GPU and display controllers
        vec![0x00] // Placeholder
    }
    
    fn handle_input_device_query(&mut self, data: &[u8]) -> Vec<u8> {
        // Process input device queries (simplified)
        // In a real system, this would involve various input controllers
        vec![0x00] // Placeholder
    }
    
    fn handle_gpu_operation(&mut self, data: &[u8]) -> Vec<u8> {
        // Process GPU operations (simplified)
        // In a real system, this would involve complex GPU processing
        self.gpu.process(data)
    }
    
    fn handle_power_state_change(&mut self, data: &[u8]) -> Vec<u8> {
        // Process power state changes (simplified)
        // In a real system, this would involve ACPI and power management
        vec![0x00] // Placeholder
    }
    
    fn update_thermal_state(&mut self) {
        // Get temperatures from components
        let cpu_temp = self.cpu.get_temperature();
        let gpu_temp = self.gpu.get_temperature();
        
        // Update cooling system
        let max_temp = cpu_temp.max(gpu_temp) as u8;
        if max_temp > 70 {
            self.cooling.increase_cooling(max_temp);
        } else {
            self.cooling.decrease_cooling();
        }
    }
    
    fn get_system_status_data(&self) -> Vec<u8> {
        let mut response = Vec::new();
        
        // CPU utilization (0-100%)
        response.push(self.cpu.get_utilization() as u8);
        
        // Memory usage (0-100%)
        response.push((self.ram.get_usage() * 100.0) as u8);
        
        // Temperature (0-100°C)
        response.push(self.cpu.get_temperature() as u8);
        
        // Battery percentage
        response.push(self.power.get_battery_percentage());
        
        // Power source (0 = Battery, 1 = AC, 2 = External)
        let power_source = match self.power.get_power_source() {
            crate::hardware::power::PowerSource::Battery => 0,
            crate::hardware::power::PowerSource::AC => 1,
            crate::hardware::power::PowerSource::External => 2,
        };
        response.push(power_source);
        
        response
    }
    
    pub fn get_hardware_status(&self) -> String {
        if !self.is_powered_on {
            return "Hardware powered off".to_string();
        }
        
        format!(
            "CPU: {:.1}% @ {:.1}°C | RAM: {:.1}% | Storage: {:.1}% | Power: {} {:.1}%",
            self.cpu.get_utilization(),
            self.cpu.get_temperature(),
            self.ram.get_usage() * 100.0,
            self.storage.get_usage(),
            if self.power.is_on_battery() { "Battery" } else { "AC" },
            self.power.get_battery_percentage()
        )
    }
    
    // Method to allow software components to access the hardware
    // This simulates the hardware interface exposed to software
    pub fn clone(&self) -> Self {
        if !self.is_powered_on {
            panic!("Cannot clone hardware when powered off");
        }
        
        // In a real system, we'd return a handle/reference to the hardware
        // For simulation, we return a simple clone
        Self::new()
    }
}
