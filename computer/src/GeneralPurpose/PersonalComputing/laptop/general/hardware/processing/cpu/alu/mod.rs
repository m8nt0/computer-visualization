use super::super::super::types::PhysicalAddress;
use super::super::super::visualization::{HardwareComponent, ComponentState};

pub struct ALU {
    address: PhysicalAddress,
    name: String,
    is_active: bool,
    current_operation: Option<Operation>,
    input_a: u32,
    input_b: u32,
    result: u32,
    flags: Flags,
    power_consumption: f32,
    temperature: f32,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Xor,
    Not,
    ShiftLeft,
    ShiftRight,
}

#[derive(Debug, Clone)]
pub struct Flags {
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl ALU {
    pub fn new(address: PhysicalAddress) -> Self {
        Self {
            address,
            name: "Arithmetic Logic Unit".to_string(),
            is_active: false,
            current_operation: None,
            input_a: 0,
            input_b: 0,
            result: 0,
            flags: Flags {
                zero: false,
                carry: false,
                overflow: false,
                negative: false,
            },
            power_consumption: 0.0,
            temperature: 25.0,
        }
    }

    pub fn execute_operation(&mut self, op: Operation, a: u32, b: u32) -> u32 {
        self.is_active = true;
        self.current_operation = Some(op.clone());
        self.input_a = a;
        self.input_b = b;
        
        // Simulate power consumption and temperature increase
        self.power_consumption = 5.0;
        self.temperature += 0.1;
        
        match op {
            Operation::Add => {
                let (result, overflow) = a.overflowing_add(b);
                self.result = result;
                self.flags.overflow = overflow;
                self.flags.zero = result == 0;
                self.flags.negative = (result as i32) < 0;
                result
            }
            // ... other operations ...
            _ => 0
        }
    }

    pub fn get_operation_data(&self) -> Vec<u8> {
        let mut data = Vec::new();
        if let Some(op) = &self.current_operation {
            data.push(op.clone() as u8);
            data.extend_from_slice(&self.input_a.to_le_bytes());
            data.extend_from_slice(&self.input_b.to_le_bytes());
            data.extend_from_slice(&self.result.to_le_bytes());
        }
        data
    }
}

impl HardwareComponent for ALU {
    fn get_state(&self) -> ComponentState {
        ComponentState {
            is_active: self.is_active,
            power_consumption: self.power_consumption,
            temperature: self.temperature,
            utilization: if self.is_active { 1.0 } else { 0.0 },
            error_state: None,
            custom_data: self.get_operation_data(),
        }
    }

    fn get_address(&self) -> PhysicalAddress {
        self.address
    }

    fn get_name(&self) -> &str {
        &self.name
    }
} 