//implement keyboard input


pub struct Keyboard {
    pub key_pressed: bool,
}

impl Keyboard {
    pub fn new() -> Self {
        Self { key_pressed: false }
    }

    pub fn key_pressed(&self) -> bool {
        self.key_pressed
    }

    pub fn key_released(&self) -> bool {
        !self.key_pressed
    }
}
