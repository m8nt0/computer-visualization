use super::common::{Point, Size, Color, Rect};
use crate::src::hardware::processing::cpu::alu::{ALU, Operation, Flags};
use crate::src::hardware::visualization::{HardwareVisualizer, HardwareComponent, ComponentState};

pub struct AluVisualizer {
    position: Point,
    size: Size,
    colors: AluColors,
    animation: AluAnimation,
    current_state: Option<ComponentState>,
}

struct AluColors {
    background: Color,
    active: Color,
    inactive: Color,
    input: Color,
    output: Color,
    error: Color,
}

struct AluAnimation {
    operation_progress: f32,
    input_flow: bool,
    output_flow: bool,
}

impl AluVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        Self {
            position,
            size,
            colors: AluColors {
                background: Color::new(0.2, 0.2, 0.2, 1.0),
                active: Color::new(0.0, 1.0, 0.0, 1.0),
                inactive: Color::new(0.5, 0.5, 0.5, 1.0),
                input: Color::new(0.0, 0.0, 1.0, 1.0),
                output: Color::new(1.0, 0.0, 0.0, 1.0),
                error: Color::new(1.0, 0.0, 0.0, 1.0),
            },
            animation: AluAnimation {
                operation_progress: 0.0,
                input_flow: false,
                output_flow: false,
            },
            current_state: None,
        }
    }

    fn draw_alu_structure(&self) {
        // Draw main ALU block
        let main_rect = Rect::new(self.position, self.size);
        let color = if self.current_state.as_ref().map_or(false, |s| s.is_active) {
            self.colors.active
        } else {
            self.colors.inactive
        };
        main_rect.fill(color);
        
        // Draw input ports
        self.draw_input_ports();
        
        // Draw output port
        self.draw_output_port();
        
        // Draw operation selector
        self.draw_operation_selector();
    }

    fn draw_operation(&self, state: &ComponentState) {
        if state.custom_data.len() >= 13 { // 1 byte for operation + 4 bytes each for inputs and result
            let op = state.custom_data[0];
            let input_a = u32::from_le_bytes(state.custom_data[1..5].try_into().unwrap());
            let input_b = u32::from_le_bytes(state.custom_data[5..9].try_into().unwrap());
            let result = u32::from_le_bytes(state.custom_data[9..13].try_into().unwrap());
            
            // Draw operation symbol
            let symbol = match op {
                0 => "+",
                1 => "-",
                2 => "×",
                3 => "÷",
                _ => "?",
            };
            
            // Draw operation with animation
            self.draw_animated_operation(symbol, input_a, input_b, result);
        }
    }

    fn draw_status_indicators(&self) {
        if let Some(state) = &self.current_state {
            // Draw power consumption meter
            self.draw_meter(
                "Power",
                state.power_consumption,
                0.0..10.0,
                self.colors.active
            );
            
            // Draw temperature gauge
            self.draw_meter(
                "Temp",
                state.temperature,
                20.0..100.0,
                self.colors.error
            );
            
            // Draw utilization bar
            self.draw_meter(
                "Util",
                state.utilization,
                0.0..1.0,
                self.colors.active
            );
        }
    }
}

impl HardwareVisualizer for AluVisualizer {
    fn update(&mut self, component: &dyn HardwareComponent) {
        self.current_state = Some(component.get_state());
        
        // Update animation based on component state
        if let Some(state) = &self.current_state {
            if state.is_active {
                self.animation.operation_progress = (self.animation.operation_progress + 0.1).min(1.0);
                self.animation.input_flow = true;
                self.animation.output_flow = true;
            } else {
                self.animation.operation_progress = 0.0;
                self.animation.input_flow = false;
                self.animation.output_flow = false;
            }
        }
    }
    
    fn render(&self) {
        // Draw ALU structure
        self.draw_alu_structure();
        
        // Draw current operation if available
        if let Some(state) = &self.current_state {
            self.draw_operation(state);
        }
        
        // Draw status indicators
        self.draw_status_indicators();
    }
} 