// Export all modules in display
pub mod framebuffer;
pub mod output;
pub mod rasterizer;

pub use self::framebuffer::Framebuffer;
pub use self::output::DisplayOutput;
pub use self::rasterizer::Rasterizer;

pub struct Display {
    output: DisplayOutput,
    rasterizer: Rasterizer,
}

impl Display {
    pub fn new() -> Self {
        Self {
            output: DisplayOutput::new(),
            rasterizer: Rasterizer::new(),
        }
    }
}