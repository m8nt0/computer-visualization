pub struct InstructionDecoder {
    current_instruction: Option<DecodedInstruction>,
}

#[derive(Clone)]
pub struct DecodedInstruction {
    pub opcode: u8,
    pub rd: usize,
    pub rs1: usize,
    pub rs2: usize,
    pub immediate: u32,
    pub function: u8,
}

impl InstructionDecoder {
    pub fn new() -> Self {
        Self {
            current_instruction: None,
        }
    }

    pub fn decode(&mut self, instruction: u32) -> DecodedInstruction {
        // Extract instruction fields
        let opcode = (instruction & 0x7F) as u8;
        let rd = ((instruction >> 7) & 0x1F) as usize;
        let rs1 = ((instruction >> 15) & 0x1F) as usize;
        let rs2 = ((instruction >> 20) & 0x1F) as usize;
        let function = ((instruction >> 12) & 0x7) as u8;
        let immediate = self.extract_immediate(instruction, opcode);

        let decoded = DecodedInstruction {
            opcode,
            rd,
            rs1,
            rs2,
            immediate,
            function,
        };

        self.current_instruction = Some(decoded.clone());
        decoded
    }

    fn extract_immediate(&self, instruction: u32, opcode: u8) -> u32 {
        match opcode & 0x7F {
            0x13 => (instruction >> 20) & 0xFFF, // I-type
            0x23 => {                            // S-type
                ((instruction >> 25) << 5) |
                ((instruction >> 7) & 0x1F)
            },
            0x63 => {                            // B-type
                ((instruction >> 31) << 12) |
                ((instruction >> 7) << 11) |
                ((instruction >> 25) << 5) |
                ((instruction >> 8) & 0xF) << 1
            },
            _ => 0
        }
    }

    // Methods for visualization
    pub fn get_current_instruction(&self) -> Option<&DecodedInstruction> {
        self.current_instruction.as_ref()
    }

    pub fn get_instruction_type(&self, opcode: u8) -> &'static str {
        match opcode & 0x7F {
            0x13 => "I-type",
            0x23 => "S-type",
            0x63 => "B-type",
            0x33 => "R-type",
            0x6F => "J-type",
            _ => "Unknown"
        }
    }
}
