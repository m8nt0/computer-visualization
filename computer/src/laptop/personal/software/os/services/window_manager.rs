use super::error::{WindowError, WindowResult};
use std::collections::HashMap;

pub struct WindowManager {
    windows: HashMap<WindowId, Window>,
    layers: Vec<Layer>,
    focus: Option<WindowId>,
    compositor: Compositor,
    input_handler: InputHandler,
}

struct Window {
    id: WindowId,
    title: String,
    bounds: Rectangle,
    state: WindowState,
    flags: WindowFlags,
    buffer: FrameBuffer,
    event_queue: VecDeque<WindowEvent>,
}

struct Layer {
    id: LayerId,
    windows: Vec<WindowId>,
    opacity: f32,
    visible: bool,
    composite_mode: CompositeMode,
}

struct Compositor {
    output: DisplayOutput,
    damage_tracker: DamageTracker,
    render_queue: Vec<RenderCommand>,
    effects: Vec<CompositeEffect>,
}

impl WindowManager {
    pub fn new(display: DisplayOutput) -> Self {
        Self {
            windows: HashMap::new(),
            layers: vec![Layer::default()],
            focus: None,
            compositor: Compositor::new(display),
            input_handler: InputHandler::new(),
        }
    }

    pub fn create_window(&mut self, config: WindowConfig) -> WindowResult<WindowId> {
        let id = self.generate_window_id();
        
        let window = Window {
            id,
            title: config.title,
            bounds: config.bounds,
            state: WindowState::Normal,
            flags: config.flags,
            buffer: FrameBuffer::new(config.bounds.size),
            event_queue: VecDeque::new(),
        };
        
        self.windows.insert(id, window);
        self.layers[0].windows.push(id);
        
        Ok(id)
    }

    pub fn handle_input(&mut self, event: InputEvent) -> WindowResult<()> {
        // Update input state
        self.input_handler.process_event(event)?;
        
        // Find target window
        if let Some(window_id) = self.find_window_at(self.input_handler.cursor_position()) {
            // Update window state
            let window = self.windows.get_mut(&window_id)
                .ok_or(WindowError::InvalidWindow)?;
                
            window.event_queue.push_back(WindowEvent::from(event));
            
            // Update focus if needed
            if event.is_mouse_click() {
                self.set_focus(Some(window_id));
            }
        }
        
        Ok(())
    }

    pub fn composite(&mut self) -> WindowResult<()> {
        // Clear damage tracking
        self.compositor.damage_tracker.clear();
        
        // Update window buffers
        for window in self.windows.values_mut() {
            if window.needs_redraw() {
                window.render()?;
                self.compositor.damage_tracker.add_damage(window.bounds);
            }
        }
        
        // Composite layers
        for layer in &self.layers {
            if layer.visible {
                self.compositor.composite_layer(layer)?;
            }
        }
        
        // Present final output
        self.compositor.present()?;
        
        Ok(())
    }
} 