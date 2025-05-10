// CPU/processor abstraction
use super::super::traits::powerable::Powerable;

#[derive(Debug, Clone)]
pub struct Processor {
    name: String,
    cores: u32,
    clock_speed: f32, // in GHz
    power_state: bool,
}

impl Processor {
    pub fn new(name: &str, cores: u32, clock_speed: f32) -> Self {
        Self {
            name: name.to_string(),
            cores,
            clock_speed,
            power_state: false,
        }
    }
    
    pub fn cores(&self) -> u32 {
        self.cores
    }
    
    pub fn clock_speed(&self) -> f32 {
        self.clock_speed
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Powerable for Processor {
    fn power_on(&mut self) -> bool {
        self.power_state = true;
        true
    }
    
    fn power_off(&mut self) -> bool {
        self.power_state = false;
        true
    }
    
    fn is_powered(&self) -> bool {
        self.power_state
    }
} 