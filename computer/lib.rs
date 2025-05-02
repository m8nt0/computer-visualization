// Core library for computer architecture simulation
// This serves as the entry point for all computer types

// pub mod common;
// pub mod laptop;
// pub mod desktop;
// pub mod server;
// pub mod smartphone;
// pub mod tablet;
pub mod src;
pub mod visualization;


// Trait defining the interface for all computer types
pub trait Computer {
    fn power_on(&mut self);
    fn power_off(&mut self);
    fn is_powered_on(&self) -> bool;
    fn process_input(&mut self, input: &InputEvent) -> Vec<Event>;
    fn update(&mut self) -> Vec<Event>;
}

// Generic input events that can be translated from any UI framework
#[derive(Debug, Clone)]
pub enum InputEvent {
    KeyPressed(Key),
    KeyReleased(Key),
    MousePressed(MouseButton, f32, f32),
    MouseReleased(MouseButton, f32, f32),
    MouseMoved(f32, f32),
}

// Generic keys that can be mapped from any UI framework
#[derive(Debug, Clone, PartialEq)]
pub enum Key {
    Escape,
    Enter,
    Tab,
    Space,
    Up,
    Down,
    Left,
    Right,
    Char(char),
    // Add more keys as needed
}

// Generic mouse buttons
#[derive(Debug, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

// Events that can be emitted by the computer
#[derive(Debug, Clone)]
pub enum Event {
    PowerStateChanged(bool),
    ViewChanged(String),
    AccessRequested,
    AccessGranted,
    AccessDenied,
    // Add more events as needed
}

// Trait for UI rendering abstraction
pub trait ComputerRenderer {
    fn clear(&mut self, color: Color);
    fn draw_text(&mut self, text: &str, x: f32, y: f32, color: Color);
    fn draw_rectangle(&mut self, x: f32, y: f32, width: f32, height: f32, color: Color);
    fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, color: Color);
    fn present(&mut self);
    // Add more drawing methods as needed
}

// Color struct that can be used with any rendering framework
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
    
    pub fn from_rgba(rgba: u32) -> Self {
        let r = ((rgba >> 24) & 0xFF) as u8;
        let g = ((rgba >> 16) & 0xFF) as u8;
        let b = ((rgba >> 8) & 0xFF) as u8;
        let a = (rgba & 0xFF) as u8;
        Color { r, g, b, a }
    }
    
    pub fn to_rgba(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }
} 