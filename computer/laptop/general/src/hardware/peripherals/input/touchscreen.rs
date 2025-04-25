//implement touchscreen input

pub struct Touchscreen {
    pub touch_detected: bool,
    pub touch_x: f32,
    pub touch_y: f32,
}

impl Touchscreen {
    pub fn new() -> Self {
        Self { touch_detected: false, touch_x: 0.0, touch_y: 0.0 }
    }

    pub fn touch_detected(&self) -> bool {
        self.touch_detected
    }

    pub fn touch_x(&self) -> f32 {
        self.touch_x
    }

    pub fn touch_y(&self) -> f32 {
        self.touch_y
    }
    
}
