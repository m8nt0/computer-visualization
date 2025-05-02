use super::super::error::{GPUError, GPUResult};
use super::super::memory::VRAMController;

pub struct Framebuffer {
    width: u32,
    height: u32,
    format: PixelFormat,
    buffer: Vec<u8>,
    vram_address: u64,
    stats: FramebufferStats,
}

#[derive(Clone, Copy)]
pub enum PixelFormat {
    RGBA8,
    BGRA8,
    RGB8,
    BGR8,
    R8G8B8A8,
}

struct FramebufferStats {
    frames_rendered: u64,
    pixels_written: u64,
    bandwidth_usage: f32,
    refresh_rate: f32,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, format: PixelFormat, vram: &mut VRAMController) -> GPUResult<Self> {
        let size = width as usize * height as usize * format.bytes_per_pixel();
        let vram_address = vram.allocate(size)?;

        Ok(Self {
            width,
            height,
            format,
            buffer: vec![0; size],
            vram_address,
            stats: FramebufferStats::default(),
        })
    }

    pub fn clear(&mut self, color: [u8; 4]) {
        let pixel_size = self.format.bytes_per_pixel();
        for pixel in self.buffer.chunks_mut(pixel_size) {
            pixel.copy_from_slice(&color[..pixel_size]);
        }
        self.stats.pixels_written += (self.width * self.height) as u64;
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: [u8; 4]) -> GPUResult<()> {
        if x >= self.width || y >= self.height {
            return Err(GPUError::OutOfBounds);
        }

        let pixel_size = self.format.bytes_per_pixel();
        let offset = (y * self.width + x) as usize * pixel_size;
        self.buffer[offset..offset + pixel_size].copy_from_slice(&color[..pixel_size]);
        self.stats.pixels_written += 1;
        
        Ok(())
    }

    pub fn present(&mut self, vram: &mut VRAMController) -> GPUResult<()> {
        vram.write(self.vram_address, &self.buffer)?;
        self.stats.frames_rendered += 1;
        self.stats.bandwidth_usage += self.buffer.len() as f32;
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32, vram: &mut VRAMController) -> GPUResult<()> {
        let new_size = width as usize * height as usize * self.format.bytes_per_pixel();
        let new_address = vram.allocate(new_size)?;

        self.width = width;
        self.height = height;
        self.buffer.resize(new_size, 0);
        self.vram_address = new_address;

        Ok(())
    }
}

impl PixelFormat {
    fn bytes_per_pixel(&self) -> usize {
        match self {
            PixelFormat::RGBA8 | PixelFormat::BGRA8 | PixelFormat::R8G8B8A8 => 4,
            PixelFormat::RGB8 | PixelFormat::BGR8 => 3,
        }
    }
}
