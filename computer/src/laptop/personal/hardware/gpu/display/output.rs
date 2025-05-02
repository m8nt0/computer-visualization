use super::super::error::{GPUError, GPUResult};
use super::framebuffer::Framebuffer;
use std::time::{Duration, Instant};

pub struct DisplayOutput {
    current_buffer: usize,
    framebuffers: Vec<Framebuffer>,
    vsync_enabled: bool,
    last_present: Instant,
    refresh_interval: Duration,
    stats: OutputStats,
}

struct OutputStats {
    frames_displayed: u64,
    frame_time: Duration,
    vsync_waits: u64,
    tearing_events: u64,
}

impl DisplayOutput {
    pub fn new(width: u32, height: u32, num_buffers: usize) -> Self {
        Self {
            current_buffer: 0,
            framebuffers: Vec::new(), // Initialize in constructor
            vsync_enabled: true,
            last_present: Instant::now(),
            refresh_interval: Duration::from_nanos(16_666_667), // 60Hz
            stats: OutputStats::default(),
        }
    }

    pub fn present(&mut self) -> GPUResult<()> {
        let now = Instant::now();
        self.stats.frame_time = now - self.last_present;

        if self.vsync_enabled {
            let elapsed = now - self.last_present;
            if elapsed < self.refresh_interval {
                self.stats.vsync_waits += 1;
                std::thread::sleep(self.refresh_interval - elapsed);
            }
        } else if now - self.last_present < self.refresh_interval {
            self.stats.tearing_events += 1;
        }

        self.swap_buffers()?;
        self.stats.frames_displayed += 1;
        self.last_present = now;

        Ok(())
    }

    fn swap_buffers(&mut self) -> GPUResult<()> {
        self.current_buffer = (self.current_buffer + 1) % self.framebuffers.len();
        Ok(())
    }

    pub fn get_current_framebuffer(&mut self) -> &mut Framebuffer {
        &mut self.framebuffers[self.current_buffer]
    }

    pub fn set_vsync(&mut self, enabled: bool) {
        self.vsync_enabled = enabled;
    }

    pub fn set_refresh_rate(&mut self, hz: u32) {
        self.refresh_interval = Duration::from_secs(1) / hz;
    }
}
