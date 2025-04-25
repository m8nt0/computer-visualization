// Specific laptop module - contains logic for specific laptop models
// This is a placeholder for future implementation

use crate::{Computer, InputEvent, Event};
use crate::laptop::{Laptop, ViewMode};

// Placeholder implementation - to be expanded later
pub struct SpecificLaptop {
    base: Laptop,
    model: String,
}

impl SpecificLaptop {
    pub fn new(model: &str) -> Self {
        SpecificLaptop {
            base: Laptop::new(),
            model: model.to_string(),
        }
    }
    
    pub fn model(&self) -> &str {
        &self.model
    }
}

impl Computer for SpecificLaptop {
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