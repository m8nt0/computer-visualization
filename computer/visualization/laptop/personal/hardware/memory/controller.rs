use super::common::{Point, Size, Color, Rect};
use crate::hardware::memory::controller::{
    MemoryController,
    CommandQueue,
    RequestQueue,
    Scheduler,
    PowerState,
    Timing
};

pub struct MemoryControllerVisualizer {
    position: Point,
    size: Size,
    command_queue: CommandQueueView,
    request_queue: RequestQueueView,
    scheduler: SchedulerView,
    timing: TimingVisualizer,
    power: PowerVisualizer,
}

impl MemoryControllerVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        let layout = ControllerLayout::new(position, size);
        
        Self {
            position,
            size,
            command_queue: CommandQueueView::new(layout.command_queue_region),
            request_queue: RequestQueueView::new(layout.request_queue_region),
            scheduler: SchedulerView::new(layout.scheduler_region),
            timing: TimingVisualizer::new(layout.timing_region),
            power: PowerVisualizer::new(layout.power_region),
        }
    }

    pub fn update(&mut self, controller: &MemoryController) {
        self.command_queue.update(&controller.command_queue);
        self.request_queue.update(&controller.request_queue);
        self.scheduler.update(&controller.scheduler);
        self.timing.update(&controller.timing);
        self.power.update(&controller.power_state);
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw controller structure
        self.draw_controller_structure(frame);
        
        // Render queues
        self.command_queue.render(frame);
        self.request_queue.render(frame);
        
        // Render scheduler
        self.scheduler.render(frame);
        
        // Render timing diagram
        self.timing.render(frame);
        
        // Render power state
        self.power.render(frame);
        
        // Draw performance metrics
        self.draw_controller_metrics(frame);
    }
} 