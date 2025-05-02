pub struct LaptopRenderer {
    body_color: u32,
    highlight_color: u32,
}

impl LaptopRenderer {
    pub fn new() -> Self {
        Self {
            body_color: 0xD0D0D0,
            highlight_color: 0xE0E0E0,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        self.draw_body(buffer, width, height);
        self.draw_highlights(buffer, width, height);
    }

    fn draw_body(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Main laptop body
        for y in height/3..7*height/8 {
            for x in width/6..5*width/6 {
                let index = y * width + x;
                if index < buffer.len() {
                    buffer[index] = self.body_color;
                }
            }
        }
    }

    fn draw_highlights(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Add metallic highlights
        for y in height/3..7*height/8 {
            let highlight = ((y - height/3) as f32 / (height/2) as f32 * 32.0) as u32;
            for x in width/6..width/6 + 20 {
                let index = y * width + x;
                if index < buffer.len() {
                    buffer[index] = self.highlight_color.saturating_add(highlight);
                }
            }
        }
    }
}
