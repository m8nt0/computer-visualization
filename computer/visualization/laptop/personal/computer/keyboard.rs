pub struct KeyboardRenderer {
    keyboard_color: u32,
    key_color: u32,
    touchpad_color: u32,
}

impl KeyboardRenderer {
    pub fn new() -> Self {
        Self {
            keyboard_color: 0xC0C0C0,
            key_color: 0xB0B0B0,
            touchpad_color: 0xB0B0B0,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        self.draw_keyboard_area(buffer, width, height);
        self.draw_keys(buffer, width, height);
        self.draw_touchpad(buffer, width, height);
    }

    fn draw_keyboard_area(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        for y in 2*height/3..7*height/8 {
            for x in width/5..4*width/5 {
                let index = y * width + x;
                if index < buffer.len() {
                    buffer[index] = self.keyboard_color;
                }
            }
        }
    }

    fn draw_keys(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        let key_spacing = 20;
        let key_size = 16;
        for row in 0..6 {
            for col in 0..15 {
                let key_x = width/5 + 30 + col * key_spacing;
                let key_y = 2*height/3 + 20 + row * key_spacing;
                self.draw_single_key(buffer, key_x, key_y, key_size, width);
            }
        }
    }

    fn draw_single_key(&self, buffer: &mut Vec<u32>, x: usize, y: usize, size: usize, width: usize) {
        for dy in 0..size {
            for dx in 0..size {
                let pos = (y + dy) * width + (x + dx);
                if pos < buffer.len() {
                    buffer[pos] = self.key_color;
                }
            }
        }
    }

    fn draw_touchpad(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        for y in 5*height/6 - 30..5*height/6 + 30 {
            for x in width/2 - 50..width/2 + 50 {
                let index = y * width + x;
                if index < buffer.len() {
                    buffer[index] = self.touchpad_color;
                }
            }
        }
    }
}
