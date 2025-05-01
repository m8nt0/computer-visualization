use crate::src::hardware::visualization::{HardwareVisualizer, HardwareComponent, ComponentState};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub position: Point,
    pub size: Size,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Size {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl Rect {
    pub fn new(position: Point, size: Size) -> Self {
        Self { position, size }
    }
    
    pub fn fill(&self, color: Color) {
        // This would be implemented by the actual rendering backend
        // For now, it's a placeholder
    }
}

// Common visualization components
pub struct Meter {
    position: Point,
    size: Size,
    label: String,
    value: f32,
    range: std::ops::Range<f32>,
    color: Color,
}

impl Meter {
    pub fn new(
        position: Point,
        size: Size,
        label: String,
        value: f32,
        range: std::ops::Range<f32>,
        color: Color,
    ) -> Self {
        Self {
            position,
            size,
            label,
            value,
            range,
            color,
        }
    }
    
    pub fn draw(&self) {
        // Draw meter background
        let bg_rect = Rect::new(self.position, self.size);
        bg_rect.fill(Color::new(0.1, 0.1, 0.1, 1.0));
        
        // Calculate fill percentage
        let percentage = (self.value - self.range.start) / (self.range.end - self.range.start);
        let fill_width = self.size.width * percentage;
        
        // Draw fill
        let fill_rect = Rect::new(
            self.position,
            Size::new(fill_width, self.size.height)
        );
        fill_rect.fill(self.color);
    }
}

// Common animation utilities
pub struct Animation {
    progress: f32,
    duration: f32,
    is_playing: bool,
}

impl Animation {
    pub fn new(duration: f32) -> Self {
        Self {
            progress: 0.0,
            duration,
            is_playing: false,
        }
    }
    
    pub fn start(&mut self) {
        self.is_playing = true;
        self.progress = 0.0;
    }
    
    pub fn update(&mut self, delta_time: f32) {
        if self.is_playing {
            self.progress = (self.progress + delta_time / self.duration).min(1.0);
            if self.progress >= 1.0 {
                self.is_playing = false;
            }
        }
    }
    
    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }
    
    pub fn get_progress(&self) -> f32 {
        self.progress
    }
}

struct transistor {
    n_dopant: u32,
    p_dopant: u32,
    gate_oxide: u32,
    gate_length: u32,
    gate_width: u32,
    gate_thickness: u32,
}

struct transistor_colors {
    n_dopant: Color,
    p_dopant: Color,
    gate_oxide: Color,
    gate_length: Color,
    gate_width: Color,
    gate_thickness: Color,
}