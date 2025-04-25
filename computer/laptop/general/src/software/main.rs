////////////////////////////////////////////////////////////
// connect all the software modules/folders basically what make up the laptop work, into a coherent whole.
//this is just the overarching LOGIC that connects all the inidividual software components LOGIC's (within this dir) together.
// like how in a typical computer, the software runs through the hardware, i want to do the same here, basically send it through the hardware dir ../hardware/main.rs.

use crate::hardware::main::Hardware;

pub mod operating_system;
pub mod applications;
pub mod drivers;
pub mod services;
pub mod user_interface;

pub struct Software {
    pub os: operating_system::OperatingSystem,
    pub apps: applications::Applications,
    pub drivers: drivers::Drivers,
    pub services: services::Services,
    pub ui: user_interface::UserInterface,
    hardware_interface: Option<Hardware>,
}

impl Software {
    pub fn new() -> Self {
        Software {
            os: operating_system::OperatingSystem::new(),
            apps: applications::Applications::new(),
            drivers: drivers::Drivers::new(),
            services: services::Services::new(),
            ui: user_interface::UserInterface::new(),
            hardware_interface: None,
        }
    }
    
    // Connect software to hardware system
    pub fn connect_to_hardware(&mut self, hardware: Hardware) {
        println!("Connecting software to hardware interface...");
        self.hardware_interface = Some(hardware);
        println!("Software-hardware interface established.");
    }
    
    pub fn boot(&mut self) -> bool {
        if self.hardware_interface.is_none() {
            println!("ERROR: Cannot boot software without hardware!");
            return false;
        }
        
        println!("Booting software components...");
        
        // Boot sequence
        self.os.initialize();
        
        // Load drivers to interface with hardware
        self.drivers.load_all();
        
        // Start system services
        self.services.start_all();
        
        // Initialize user interface
        self.ui.initialize();
        
        // Load default applications
        self.apps.load_default_apps();
        
        println!("Software boot complete. System is ready.");
        true
    }
    
    pub fn shutdown(&mut self) -> bool {
        println!("Initiating software shutdown sequence...");
        
        // Shutdown in reverse order of initialization
        self.apps.close_all();
        self.ui.terminate();
        self.services.stop_all();
        self.drivers.unload_all();
        self.os.shutdown();
        
        println!("Software shutdown complete.");
        true
    }
    
    // Process user commands by sending instructions to hardware
    pub fn process_user_command(&mut self, command: &str) -> String {
        println!("Processing user command: {}", command);
        
        // Convert command to binary instructions (simplified example)
        let instructions = self.os.parse_command(command);
        
        // Send instructions to hardware and get response
        if let Some(ref mut hardware) = self.hardware_interface {
            let hardware_response = hardware.process_software_instructions(&instructions);
            
            // Process hardware response through OS
            let result = self.os.interpret_hardware_response(&hardware_response);
            
            // Update UI with the result
            self.ui.update_display(&result);
            
            result
        } else {
            "ERROR: Hardware interface not connected".to_string()
        }
    }
    
    pub fn get_system_status(&mut self) -> String {
        let software_status = format!(
            "OS: {} | Services Running: {} | Apps Running: {}",
            self.os.get_version(),
            self.services.get_running_count(),
            self.apps.get_running_count()
        );
        
        let hardware_status = if let Some(ref hw) = self.hardware_interface {
            hw.get_hardware_status()
        } else {
            "Hardware not connected".to_string()
        };
        
        format!("{}\n{}", software_status, hardware_status)
    }
    
    // Main interaction loop between software and hardware
    pub fn run_cycle(&mut self) {
        if let Some(ref mut hardware) = self.hardware_interface {
            // 1. Collect system events
            let system_events = self.os.collect_system_events();
            
            // 2. Process system events
            let instructions = self.os.process_system_events(&system_events);
            
            // 3. Send instructions to hardware
            let hardware_response = hardware.process_software_instructions(&instructions);
            
            // 4. Update software components based on hardware response
            self.os.update_from_hardware_response(&hardware_response);
            self.services.process_hardware_events(&hardware_response);
            self.ui.refresh();
            
            // 5. Process application requests
            self.apps.process_pending_tasks();
        }
    }
}

