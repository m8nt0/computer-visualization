pub struct PowerIndicator {
    power_on_color: u32,
    power_off_color: u32,
}

impl PowerIndicator {
    pub fn new() -> Self {
        Self {
            power_on_color: 0x00FF00,
            power_off_color: 0x300000,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize, powered: bool) {
        let color = if powered { self.power_on_color } else { self.power_off_color };
        
        for y in 2*height/3 - 5..2*height/3 + 5 {
            for x in width/6 + 10..width/6 + 20 {
                let index = y * width + x;
                if index < buffer.len() {
                    buffer[index] = color;
                }
            }
        }
    }
}
