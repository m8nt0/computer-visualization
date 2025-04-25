use super::VisualizationSystem;
use crate::hardware::gpu::GPU;

impl VisualizationSystem {
    pub fn render_screen_view(&self, buffer: &mut Vec<u32>, width: usize, height: usize, gpu: &GPU) {
        if self.computer_powered {
            // Get the GPU's display buffer and render it
            let gpu_buffer = gpu.get_display_buffer();
            let (gpu_width, gpu_height) = gpu.get_dimensions();
            
            // Calculate scaling factors
            let scale_x = width as f32 / gpu_width as f32;
            let scale_y = height as f32 / gpu_height as f32;
            
            // Draw scaled GPU buffer
            for y in 0..height {
                for x in 0..width {
                    let gpu_x = (x as f32 / scale_x) as usize;
                    let gpu_y = (y as f32 / scale_y) as usize;
                    
                    if gpu_x < gpu_width as usize && gpu_y < gpu_height as usize {
                        let gpu_index = gpu_y * gpu_width as usize + gpu_x;
                        let color = gpu_buffer[gpu_index];
                        buffer[y * width + x] = color as u32;
                    }
                }
            }
        } else {
            // Draw "powered off" screen
            for pixel in buffer.iter_mut() {
                *pixel = 0x000000;
            }
        }
    }
} 