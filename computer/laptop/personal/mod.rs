// Personal laptop module - contains logic specific to the user's personal laptop
use crate::{Computer, InputEvent, Event};
use crate::laptop::{Laptop, ViewMode};
use crate::common::components::{CPU, Memory, Storage};

pub mod visualization; // Will contain visualization-specific code

// PersonalLaptop extends the base Laptop with personal customizations
pub struct PersonalLaptop {
    base: Laptop,
    user_name: String,
    // Add personal laptop specific properties here
}

impl PersonalLaptop {
    pub fn new() -> Self {
        PersonalLaptop {
            base: Laptop::with_components(
                CPU::new("Intel Core i7", 8, 3.2),
                Memory::new("Crucial", 16, "DDR4"),
                Storage::new("Samsung 970 EVO", 1000, "NVMe SSD"),
            ),
            user_name: "User".to_string(),
        }
    }
    
    pub fn with_user_name(user_name: &str) -> Self {
        PersonalLaptop {
            base: Laptop::with_components(
                CPU::new("Intel Core i7", 8, 3.2),
                Memory::new("Crucial", 16, "DDR4"),
                Storage::new("Samsung 970 EVO", 1000, "NVMe SSD"),
            ),
            user_name: user_name.to_string(),
        }
    }
    
    pub fn user_name(&self) -> &str {
        &self.user_name
    }
    
    // Add personal laptop specific methods here
}

// Delegate Computer trait implementation to the base Laptop
impl Computer for PersonalLaptop {
    fn power_on(&mut self) {
        self.base.power_on();
    }
    
    fn power_off(&mut self) {
        self.base.power_off();
    }
    
    fn is_powered_on(&self) -> bool {
        self.base.is_powered_on()
    }
    
    fn process_input(&mut self, input: &InputEvent) -> Vec<Event> {
        // Forward input to base laptop and process any personal-specific input
        let mut events = self.base.process_input(input);
        
        // Add personal-specific input handling here
        
        events
    }
    
    fn update(&mut self) -> Vec<Event> {
        // Forward update to base laptop and process any personal-specific updates
        let mut events = self.base.update();
        
        // Add personal-specific update logic here
        
        events
    }
}

// Delegate accessor methods to the base Laptop
impl std::ops::Deref for PersonalLaptop {
    type Target = Laptop;
    
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl std::ops::DerefMut for PersonalLaptop {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
} 