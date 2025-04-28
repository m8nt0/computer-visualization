use super::common::{Point, Size, Color, Rect};
use crate::hardware::cpu::cache_controller::{CacheController, CacheLine, AccessType};

pub struct CacheVisualizer {
    position: Point,
    size: Size,
    cache_lines: Vec<CacheLineView>,
    access_animations: Vec<AccessAnimation>,
}

struct CacheLineView {
    region: Rect,
    line: CacheLine,
    state: CacheLineState,
    access_count: usize,
}

impl CacheVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        Self {
            position,
            size,
            cache_lines: Vec::new(),
            access_animations: Vec::new(),
        }
    }

    pub fn update(&mut self, cache: &CacheController) {
        // Update cache line states
        for (i, line) in cache.lines().iter().enumerate() {
            if i >= self.cache_lines.len() {
                self.cache_lines.push(CacheLineView::new(line.clone()));
            } else {
                self.cache_lines[i].update(line);
            }
        }

        // Update access animations
        self.update_animations(cache);
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw cache structure
        self.draw_cache_structure(frame);
        
        // Draw cache lines
        for line in &self.cache_lines {
            line.render(frame);
        }
        
        // Draw access animations
        for animation in &self.access_animations {
            animation.render(frame);
        }
        
        // Draw statistics
        self.draw_cache_stats(frame);
    }

    fn draw_cache_structure(&self, frame: &mut Frame) {
        // Draw cache outline
        frame.draw_rect_outline(
            Rect::new(self.position, self.size),
            Color::WHITE,
            2.0
        );
        
        // Draw set boundaries
        let sets = self.cache_lines.chunks(4); // 4-way set associative
        for (i, _) in sets.enumerate() {
            let y = self.position.y + (i as f32 * self.size.height / 8.0);
            frame.draw_line(
                Point::new(self.position.x, y),
                Point::new(self.position.x + self.size.width, y),
                Color::GRAY
            );
        }
    }

    fn draw_cache_stats(&self, frame: &mut Frame) {
        let hits = self.cache_lines.iter()
            .filter(|line| line.state == CacheLineState::Hit)
            .count();
        let misses = self.cache_lines.iter()
            .filter(|line| line.state == CacheLineState::Miss)
            .count();
        
        let hit_rate = hits as f32 / (hits + misses) as f32;
        
        frame.draw_text(
            &format!("Hit Rate: {:.1}%", hit_rate * 100.0),
            self.position + Point::new(10.0, self.size.height - 20.0),
            TextStyle::default()
        );
    }
} 