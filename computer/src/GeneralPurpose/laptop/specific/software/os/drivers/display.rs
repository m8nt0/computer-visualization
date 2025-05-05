use super::error::{DriverError, DriverResult};
use crate::hardware::gpu::{GPU, DisplayBuffer, RenderCommand};

pub struct DisplayDriver {
    gpu: GPU,
    framebuffer: DisplayBuffer,
    command_queue: Vec<RenderCommand>,
    mode: DisplayMode,
    stats: DisplayStats,
}

struct DisplayMode {
    width: u32,
    height: u32,
    refresh_rate: u32,
    bits_per_pixel: u32,
}

impl DisplayDriver {
    pub fn new(gpu: GPU) -> Self {
        let mode = gpu.get_default_mode();
        let framebuffer = gpu.allocate_buffer(mode.width, mode.height)?;

        Self {
            gpu,
            framebuffer,
            command_queue: Vec::new(),
            mode,
            stats: DisplayStats::default(),
        }
    }

    pub fn set_mode(&mut self, mode: DisplayMode) -> DriverResult<()> {
        if !self.gpu.supports_mode(&mode) {
            return Err(DriverError::UnsupportedMode);
        }

        // Allocate new framebuffer
        let new_buffer = self.gpu.allocate_buffer(mode.width, mode.height)?;
        
        // Switch to new mode
        self.gpu.set_display_mode(&mode)?;
        self.framebuffer = new_buffer;
        self.mode = mode;

        Ok(())
    }

    pub fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: Color) -> DriverResult<()> {
        let cmd = RenderCommand::DrawRect {
            x, y, width, height, color
        };
        self.command_queue.push(cmd);
        Ok(())
    }

    pub fn present(&mut self) -> DriverResult<()> {
        // Process all pending render commands
        for cmd in self.command_queue.drain(..) {
            self.gpu.execute_command(cmd)?;
        }

        // Swap buffers
        self.gpu.present_buffer(&self.framebuffer)?;
        self.stats.frames_presented += 1;

        Ok(())
    }
} 