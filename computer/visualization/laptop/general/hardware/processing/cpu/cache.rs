use super::super::super::super::src::hardware::visualization::{HardwareVisualizer, HardwareComponent, ComponentState};
use super::common::{Point, Size, Color, Rect, Meter, Animation};

pub struct CacheVisualizer {
    position: Point,
    size: Size,
    colors: CacheColors,
    animation: CacheAnimation,
    current_state: Option<ComponentState>,
}

struct CacheColors {
    background: Color,
    hit: Color,
    miss: Color,
    active: Color,
    inactive: Color,
}

struct CacheAnimation {
    access_animation: Animation,
    hit_miss_animation: Animation,
    last_hit: bool,
}

impl CacheVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        let colors = CacheColors {
            background: Color::new(0.1, 0.1, 0.2, 1.0),
            hit: Color::new(0.0, 1.0, 0.0, 1.0),
            miss: Color::new(1.0, 0.0, 0.0, 1.0),
            active: Color::new(0.0, 0.7, 1.0, 1.0),
            inactive: Color::new(0.3, 0.3, 0.3, 1.0),
        };

        Self {
            position,
            size,
            colors,
            animation: CacheAnimation {
                access_animation: Animation::new(0.2),
                hit_miss_animation: Animation::new(0.5),
                last_hit: false,
            },
            current_state: None,
        }
    }

    fn draw_cache_structure(&self) {
        // Draw main cache block
        let main_rect = Rect::new(self.position, self.size);
        main_rect.fill(self.colors.background);

        // Draw cache lines
        self.draw_cache_lines();

        // Draw hit/miss indicator
        self.draw_hit_miss_indicator();
    }

    fn draw_cache_lines(&self) {
        //fixed cache lines
        let line_count = 8; // Number of cache lines to display
        let line_height = self.size.height / line_count as f32;
        
        for i in 0..line_count {
            let line_rect = Rect::new(
                Point::new(self.position.x, self.position.y + i as f32 * line_height),
                Size::new(self.size.width, line_height)
            );
            
            // Draw line with animation if it's being accessed
            if self.animation.access_animation.is_playing() {
                let color = if self.animation.last_hit {
                    self.colors.hit
                } else {
                    self.colors.miss
                };
                line_rect.fill(color);
            } else {
                line_rect.fill(self.colors.inactive);
            }
        }
    }

    fn draw_hit_miss_indicator(&self) {
        if self.animation.hit_miss_animation.is_playing() {
            let indicator_size = Size::new(20.0, 20.0);
            let indicator_pos = Point::new(
                self.position.x + self.size.width - indicator_size.width,
                self.position.y + self.size.height - indicator_size.height
            );
            
            let color = if self.animation.last_hit {
                self.colors.hit
            } else {
                self.colors.miss
            };
            
            let indicator_rect = Rect::new(indicator_pos, indicator_size);
            indicator_rect.fill(color);
        }
    }

    fn draw_metrics(&self) {
        if let Some(state) = &self.current_state {
            // Draw hit rate meter
            let hit_rate_meter = Meter::new(
                Point::new(self.position.x + 10.0, self.position.y + self.size.height + 10.0),
                Size::new(100.0, 10.0),
                "Hit Rate".to_string(),
                state.utilization,
                0.0..1.0,
                self.colors.hit
            );
            hit_rate_meter.draw();
            
            // Draw power consumption meter
            let power_meter = Meter::new(
                Point::new(self.position.x + 10.0, self.position.y + self.size.height + 30.0),
                Size::new(100.0, 10.0),
                "Power".to_string(),
                state.power_consumption,
                0.0..5.0,
                self.colors.active
            );
            power_meter.draw();
        }
    }
}

impl HardwareVisualizer for CacheVisualizer {
    fn update(&mut self, component: &dyn HardwareComponent) {
        self.current_state = Some(component.get_state());
        
        // Update animations based on cache activity
        if let Some(state) = &self.current_state {
            if state.is_active {
                self.animation.access_animation.start();
                self.animation.hit_miss_animation.start();
                // In a real implementation, we would get hit/miss information from the cache
                self.animation.last_hit = rand::random();
            }
        }
    }

    fn render(&self) {
        // Draw cache structure
        self.draw_cache_structure();
        
        // Draw metrics
        self.draw_metrics();
    }
} 