//implement touchpad input

pub struct Touchpad {
    pub touchpad_moved: bool,
    pub touchpad_x: f32,
    pub touchpad_y: f32,
    pub touchpad_pressed: bool,
    pub touchpad_released: bool,
}

impl Touchpad {
    pub fn new() -> Self {
        Self { touchpad_moved: false, touchpad_x: 0.0, touchpad_y: 0.0, touchpad_pressed: false, touchpad_released: false }
    }

    pub fn touchpad_moved(&self) -> bool {
        self.touchpad_moved
    }

    pub fn touchpad_x(&self) -> f32 {
        self.touchpad_x
    }

    pub fn touchpad_y(&self) -> f32 {
        self.touchpad_y
    }

    pub fn touchpad_moved(&mut self, x: f32, y: f32) {
        self.touchpad_moved = true;
        self.touchpad_x = x;
        self.touchpad_y = y;
        println!("Touchpad moved to ({}, {})", x, y);
    }

    pub fn touchpad_pressed(&mut self, x: f32, y: f32) {
        self.touchpad_pressed = true;
        self.touchpad_x = x;
        self.touchpad_y = y;
        println!("Touchpad pressed at ({}, {})", x, y);
    }

    pub fn touchpad_released(&mut self, x: f32, y: f32) {
        self.touchpad_released = true;
        self.touchpad_x = x;
        self.touchpad_y = y;
        println!("Touchpad released at ({}, {})", x, y);
    }
    
}       
