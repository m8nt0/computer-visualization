pub struct ScreenRenderer {
    bezel_color: u32,
    screen_on_color: u32,
    screen_off_color: u32,
}

impl ScreenRenderer {
    pub fn new() -> Self {
        Self {
            bezel_color: 0x202020,
            screen_on_color: 0x000080,  // Dark blue when on
            screen_off_color: 0x000000,  // Black when off
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize, powered: bool) {
        self.draw_bezel(buffer, width, height);
        self.draw_screen(buffer, width, height, powered);
    }

    fn draw_bezel(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        for y in height/3..2*height/3 {
            for x in width/4..3*width/4 {
                let index = y * width + x;
                if index < buffer.len() {
                    buffer[index] = self.bezel_color;
                }
            }
        }
    }

    fn draw_screen(&self, buffer: &mut Vec<u32>, width: usize, height: usize, powered: bool) {
        let screen_color = if powered { self.screen_on_color } else { self.screen_off_color };
        
        for y in height/3 + 10..2*height/3 - 10 {
            for x in width/4 + 10..3*width/4 - 10 {
                let index = y * width + x;
                if index < buffer.len() {
                    if powered {
                        // Create a gradient effect when powered on
                        let progress = (x - (width/4 + 10)) as f32 / ((width/2 - 20) as f32);
                        let blue = (0xFF as f32 * (1.0 - progress)) as u32;
                        let purple = (0xFF as f32 * progress) as u32;
                        buffer[index] = (blue << 16) | purple;
                    } else {
                        buffer[index] = screen_color;
                    }
                }
            }
        }
    }
}
