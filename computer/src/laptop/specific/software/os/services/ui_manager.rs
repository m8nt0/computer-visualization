use super::error::{ServiceError, ServiceResult};
use std::collections::HashMap;

pub struct UiManager {
    windows: HashMap<WindowId, Window>,
    compositor: Compositor,
    input_manager: InputManager,
    theme_manager: ThemeManager,
    config: UiConfig,
}

struct Window {
    id: WindowId,
    bounds: Rectangle,
    buffer: FrameBuffer,
    state: WindowState,
    flags: WindowFlags,
    event_queue: VecDeque<WindowEvent>,
}

impl UiManager {
    pub fn create_window(&mut self, config: WindowConfig) -> ServiceResult<WindowId> {
        let id = self.generate_window_id();
        
        let window = Window {
            id,
            bounds: config.bounds,
            buffer: FrameBuffer::new(config.bounds.size),
            state: WindowState::Normal,
            flags: config.flags,
            event_queue: VecDeque::new(),
        };
        
        self.windows.insert(id, window);
        self.compositor.add_window(id, config.bounds)?;
        
        Ok(id)
    }

    pub fn handle_input(&mut self, event: InputEvent) -> ServiceResult<()> {
        // Update input state
        self.input_manager.process_event(event)?;
        
        // Find target window
        if let Some(window_id) = self.find_window_at(self.input_manager.cursor_position()) {
            let window = self.windows.get_mut(&window_id)
                .ok_or(ServiceError::InvalidWindow)?;
                
            window.event_queue.push_back(WindowEvent::from(event));
            
            if event.is_mouse_click() {
                self.focus_window(window_id)?;
            }
        }
        
        Ok(())
    }
} 