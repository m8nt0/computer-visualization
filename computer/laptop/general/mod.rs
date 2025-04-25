// General laptop module - contains logic for general laptop features
// This is a placeholder for future implementation

pub mod src;
pub mod visualization;

use crate::{Computer, InputEvent, Event};
use crate::laptop::{Laptop, ViewMode};

// Placeholder implementation - to be expanded later
pub struct GeneralLaptop {
    base: Laptop,
}

impl GeneralLaptop {
    pub fn new() -> Self {
        GeneralLaptop {
            base: Laptop::new(),
        }
    }
}

impl Computer for GeneralLaptop {
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
        self.base.process_input(input)
    }
    
    fn update(&mut self) -> Vec<Event> {
        self.base.update()
    }
} 