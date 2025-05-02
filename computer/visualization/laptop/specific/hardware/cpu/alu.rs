use super::common::{Point, Size, Color, Rect};
use crate::hardware::cpu::alu::{ALU, Operation, Flags};

pub struct AluVisualizer {
    position: Point,
    size: Size,
    colors: AluColors,
    animation: AluAnimation,
}

struct AluColors {
    active: Color,
    inactive: Color,
    result: Color,
    error: Color,
}

struct AluAnimation {
    operation_progress: f32,
    input_flows: Vec<DataFlow>,
    result_flow: Option<DataFlow>,
}

impl AluVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        Self {
            position,
            size,
            colors: AluColors::default(),
            animation: AluAnimation::new(),
        }
    }

    pub fn update(&mut self, alu: &ALU) {
        // Update operation animation
        if let Some(op) = alu.current_operation() {
            self.animation.start_operation(op);
        }
        
        // Update data flows
        self.animation.update();
        
        // Update status indicators
        self.update_status_indicators(alu);
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw ALU structure
        self.draw_alu_structure(frame);
        
        // Draw current operation
        if let Some(op) = self.current_operation {
            self.draw_operation(frame, op);
        }
        
        // Draw input/output registers
        self.draw_registers(frame);
        
        // Draw status flags
        self.draw_flags(frame);
        
        // Render animations
        self.animation.render(frame);
    }

    fn draw_alu_structure(&self, frame: &mut Frame) {
        // Draw main ALU block
        let main_rect = Rect::new(self.position, self.size);
        frame.fill_rect(main_rect, self.colors.background);
        
        // Draw input ports
        self.draw_input_ports(frame);
        
        // Draw output port
        self.draw_output_port(frame);
        
        // Draw operation selector
        self.draw_operation_selector(frame);
    }

    fn draw_operation(&self, frame: &mut Frame, op: &Operation) {
        let symbol = match op {
            Operation::Add => "+",
            Operation::Sub => "-",
            Operation::Mul => "×",
            Operation::Div => "÷",
            // ... other operations
        };

        let op_pos = self.get_operation_position();
        frame.draw_text(symbol, op_pos, TextStyle::default());
    }
} 