pub struct PortsRenderer {
    port_color: u32,
    cable_color: u32,
}

impl PortsRenderer {
    pub fn new() -> Self {
        Self {
            port_color: 0x202020,
            cable_color: 0x404040,
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        self.draw_usb_ports(buffer, width, height);
    }

    fn draw_usb_ports(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        // Draw two USB ports on the side
        for port in 0..2 {
            let port_y = height/2 + port * 30;
            for y in port_y..port_y + 15 {
                for x in width/6 - 5..width/6 + 5 {
                    let index = y * width + x;
                    if index < buffer.len() {
                        buffer[index] = self.port_color;
                    }
                }
            }
        }
    }

    pub fn draw_charging_cable(&self, buffer: &mut Vec<u32>, width: usize, height: usize) {
        let port_y = height/2;
        
        // Cable extending left
        for x in width/12..width/6 {
            let y = port_y + 7;
            let index = y * width + x;
            if index < buffer.len() {
                buffer[index] = self.cable_color;
            }
        }

        // Cable connector
        for y in port_y..port_y + 15 {
            for x in width/6 - 15..width/6 - 5 {
                let index = y * width + x;
                if index < buffer.len() {
                    buffer[index] = self.cable_color;
                }
            }
        }
    }
}
