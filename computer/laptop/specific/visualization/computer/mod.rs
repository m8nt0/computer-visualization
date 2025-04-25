mod laptop;
mod screen;
mod keyboard;
mod ports;
mod power;

pub use laptop::LaptopRenderer;
pub use screen::ScreenRenderer;
pub use keyboard::KeyboardRenderer;
pub use ports::PortsRenderer;
pub use power::PowerIndicator;

pub struct ComputerView {
    laptop: LaptopRenderer,
    screen: ScreenRenderer,
    keyboard: KeyboardRenderer,
    ports: PortsRenderer,
    power: PowerIndicator,
}

impl ComputerView {
    pub fn new() -> Self {
        Self {
            laptop: LaptopRenderer::new(),
            screen: ScreenRenderer::new(),
            keyboard: KeyboardRenderer::new(),
            ports: PortsRenderer::new(),
            power: PowerIndicator::new(),
        }
    }

    pub fn render(&self, buffer: &mut Vec<u32>, width: usize, height: usize, powered: bool) {
        self.laptop.render(buffer, width, height);
        self.screen.render(buffer, width, height, powered);
        self.keyboard.render(buffer, width, height);
        self.ports.render(buffer, width, height);
        self.power.render(buffer, width, height, powered);
    }
}
