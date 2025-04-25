use super::{alu::ALU, registers::RegisterFile};

pub struct ExecutionUnit {
    alu: ALU,
    registers: *mut RegisterFile,
    current_operation: Option<Operation>,
    busy: bool,
    cycles_remaining: u32,
}

#[derive(Clone, Debug)]
pub struct Operation {
    pub opcode: u8,
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub immediate: u32,
    pub result: Option<u32>,
}

impl ExecutionUnit {
    pub fn new(registers: *mut RegisterFile) -> Self {
        Self {
            alu: ALU::new(),
            registers,
            current_operation: None,
            busy: false,
            cycles_remaining: 0,
        }
    }

    pub fn execute(&mut self, operation: Operation) -> bool {
        if self.busy {
            return false;
        }

        self.busy = true;
        self.current_operation = Some(operation.clone());
        
        // Set cycles needed based on operation type
        self.cycles_remaining = match operation.opcode {
            0x00..=0x0F => 1, // Basic ALU ops
            0x10..=0x1F => 2, // Multiply
            0x20..=0x2F => 3, // Divide
            _ => 1,
        };

        // Get operands from registers
        let rs1_val = unsafe { (*self.registers).read_gpr(operation.rs1) };
        let rs2_val = unsafe { (*self.registers).read_gpr(operation.rs2) };

        // Execute operation
        let result = self.alu.execute(operation.opcode, rs1_val, rs2_val);

        // Store result
        if let Some(mut op) = self.current_operation {
            op.result = Some(result);
        }

        true
    }

    pub fn tick(&mut self) {
        if !self.busy {
            return;
        }

        if self.cycles_remaining > 0 {
            self.cycles_remaining -= 1;
        }

        if self.cycles_remaining == 0 {
            // Operation complete, write back result
            if let Some(op) = &self.current_operation {
                if let Some(result) = op.result {
                    unsafe {
                        (*self.registers).write_gpr(op.rd, result);
                    }
                }
            }
            self.busy = false;
            self.current_operation = None;
        }
    }

    // Methods for visualization system
    pub fn is_busy(&self) -> bool {
        self.busy
    }

    pub fn get_current_operation(&self) -> Option<&Operation> {
        self.current_operation.as_ref()
    }

    pub fn get_cycles_remaining(&self) -> u32 {
        self.cycles_remaining
    }

    pub fn get_alu_state(&self) -> &ALU {
        &self.alu
    }
}
