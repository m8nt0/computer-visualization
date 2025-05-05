// Common functionality shared across all computer types

// Constants that might be used across different computer types
pub const DEFAULT_SCREEN_WIDTH: usize = 800;
pub const DEFAULT_SCREEN_HEIGHT: usize = 600;

// Common utility functions
pub mod utils {
    // Utility function to calculate if a point is inside a rectangle (OPINION: the rectangle referring to might be rect_x and rect_y)
    pub fn point_in_rect(x: f32, y: f32, rect_x: f32, rect_y: f32, rect_width: f32, rect_height: f32) -> bool {
        x >= rect_x && x <= rect_x + rect_width && y >= rect_y && y <= rect_y + rect_height
    }
    
    // Utility function to convert RGB components to a single u32 value
    pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
        ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
    }
    
    // Utility function to convert RGBA components to a single u32 value
    pub fn rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
        ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | (a as u32)
    }
}

// Common components that might be shared across different computer types
pub mod components {
    // Base component trait that can be used for different hardware components
    pub trait Component {
        fn name(&self) -> &str;
        fn description(&self) -> &str;
        fn update(&mut self);
    }
    
    // CPU component with common properties
    pub struct CPU {
        name: String,
        cores: u32,
        clock_speed: f32, // in GHz
        description: String,
    }
    
    impl CPU {
        pub fn new(name: &str, cores: u32, clock_speed: f32) -> Self {
            CPU {
                name: name.to_string(),
                cores,
                clock_speed,
                description: format!("{} with {} cores @ {:.2} GHz", name, cores, clock_speed),
            }
        }
        
        pub fn cores(&self) -> u32 {
            self.cores
        }
        
        pub fn clock_speed(&self) -> f32 {
            self.clock_speed
        }
    }
    
    impl Component for CPU {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn description(&self) -> &str {
            &self.description
        }
        
        fn update(&mut self) {
            // CPU update logic common to all computer types
        }
    }
    
    // Memory component with common properties
    pub struct Memory {
        name: String,
        capacity: u32, // in GB
        type_: String, // e.g., DDR4, DDR5
        description: String,
    }
    
    impl Memory {
        pub fn new(name: &str, capacity: u32, type_: &str) -> Self {
            Memory {
                name: name.to_string(),
                capacity,
                type_: type_.to_string(),
                description: format!("{} {} GB {}", name, capacity, type_),
            }
        }
        
        pub fn capacity(&self) -> u32 {
            self.capacity
        }
        
        pub fn type_(&self) -> &str {
            &self.type_
        }
    }
    
    impl Component for Memory {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn description(&self) -> &str {
            &self.description
        }
        
        fn update(&mut self) {
            // Memory update logic common to all computer types
        }
    }
    
    // Storage component with common properties
    pub struct Storage {
        name: String,
        capacity: u32, // in GB
        type_: String, // e.g., SSD, HDD
        description: String,
    }
    
    impl Storage {
        pub fn new(name: &str, capacity: u32, type_: &str) -> Self {
            Storage {
                name: name.to_string(),
                capacity,
                type_: type_.to_string(),
                description: format!("{} {} GB {}", name, capacity, type_),
            }
        }
        
        pub fn capacity(&self) -> u32 {
            self.capacity
        }
        
        pub fn type_(&self) -> &str {
            &self.type_
        }
    }
    
    impl Component for Storage {
        fn name(&self) -> &str {
            &self.name
        }
        
        fn description(&self) -> &str {
            &self.description
        }
        
        fn update(&mut self) {
            // Storage update logic common to all computer types
        }
    }
} 