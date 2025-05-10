//implement mouse input

pub struct Mouse {
    pub mouse_moved: bool,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

impl Mouse {
    pub fn new() -> Self {
        Self { mouse_moved: false, mouse_x: 0.0, mouse_y: 0.0 }
    }

    pub fn mouse_moved(&mut self, x: f32, y: f32) {
        //detect mouse movement, and then what happens when the mouse moves
        self.mouse_moved = true;
        self.mouse_x = x;
        self.mouse_y = y;
        println!("Mouse moved to ({}, {})", x, y);
    }
    
    
}