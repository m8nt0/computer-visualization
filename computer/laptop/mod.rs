// Laptop module - contains the core logic for laptop computers
use crate::{Computer, InputEvent, Event, Key};
use crate::common::components::{CPU, Memory, Storage, Component};

pub mod general;
pub mod specific;
pub mod personal;

// Shared traits and structures for all laptop types

// Laptop view modes
#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    External,  // The laptop from outside (keyboard, screen, etc.)
    Hardware,  // Inside the laptop (components)
    Software,  // The laptop's operating system
}

// Base laptop struct with common properties
pub struct Laptop {
    powered_on: bool,
    view_mode: ViewMode,
    requesting_access: bool,
    access_granted: bool,
    cpu: CPU,
    memory: Memory,
    storage: Storage,
    // Add more common laptop properties here
}

impl Laptop {
    pub fn new() -> Self {
        Laptop {
            powered_on: false,
            view_mode: ViewMode::External,
            requesting_access: true,
            access_granted: false,
            cpu: CPU::new("Generic CPU", 4, 2.5),
            memory: Memory::new("Generic RAM", 8, "DDR4"),
            storage: Storage::new("Generic Storage", 512, "SSD"),
        }
    }
    
    pub fn with_components(cpu: CPU, memory: Memory, storage: Storage) -> Self {
        Laptop {
            powered_on: false,
            view_mode: ViewMode::External,
            requesting_access: true,
            access_granted: false,
            cpu,
            memory,
            storage,
        }
    }
    
    pub fn view_mode(&self) -> &ViewMode {
        &self.view_mode
    }
    
    pub fn set_view_mode(&mut self, mode: ViewMode) {
        self.view_mode = mode;
    }
    
    pub fn is_requesting_access(&self) -> bool {
        self.requesting_access
    }
    
    pub fn grant_access(&mut self) {
        self.requesting_access = false;
        self.access_granted = true;
    }
    
    pub fn deny_access(&mut self) {
        self.requesting_access = false;
        self.access_granted = false;
    }
    
    pub fn is_access_granted(&self) -> bool {
        self.access_granted
    }
    
    pub fn cpu(&self) -> &CPU {
        &self.cpu
    }
    
    pub fn memory(&self) -> &Memory {
        &self.memory
    }
    
    pub fn storage(&self) -> &Storage {
        &self.storage
    }
}

impl Computer for Laptop {
    fn power_on(&mut self) {
        self.powered_on = true;
    }
    
    fn power_off(&mut self) {
        self.powered_on = false;
    }
    
    fn is_powered_on(&self) -> bool {
        self.powered_on
    }
    
    fn process_input(&mut self, input: &InputEvent) -> Vec<Event> {
        let mut events = Vec::new();
        
        match input {
            InputEvent::KeyPressed(key) => {
                match key {
                    Key::Char('p') | Key::Char('P') => {
                        if !self.powered_on && self.access_granted {
                            self.power_on();
                            events.push(Event::PowerStateChanged(true));
                        }
                    },
                    Key::Char('y') | Key::Char('Y') => {
                        if self.requesting_access {
                            self.grant_access();
                            events.push(Event::AccessGranted);
                        }
                    },
                    Key::Char('n') | Key::Char('N') => {
                        if self.requesting_access {
                            self.deny_access();
                            events.push(Event::AccessDenied);
                        }
                    },
                    Key::Tab => {
                        if self.powered_on && self.access_granted {
                            // Cycle through view modes
                            self.view_mode = match self.view_mode {
                                ViewMode::External => ViewMode::Hardware,
                                ViewMode::Hardware => ViewMode::Software,
                                ViewMode::Software => ViewMode::External,
                            };
                            events.push(Event::ViewChanged(format!("{:?}", self.view_mode)));
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
        
        events
    }
    
    fn update(&mut self) -> Vec<Event> {
        // Update laptop state
        Vec::new()
    }
} 