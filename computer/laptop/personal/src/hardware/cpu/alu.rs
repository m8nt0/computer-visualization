pub struct ALU {
    accumulator: u32,
    flags: Flags,
    busy: bool,
}

#[derive(Default)]
struct Flags {
    zero: bool,      // Result is zero
    carry: bool,     // Operation produced a carry
    overflow: bool,  // Operation produced overflow
    negative: bool,  // Result is negative
}

impl ALU {
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            flags: Flags::default(),
            busy: false,
        }
    }

    pub fn execute(&mut self, opcode: u8, operand1: u32, operand2: u32) -> u32 {
        self.busy = true;
        let result = match opcode {
            0x00 => self.add(operand1, operand2),
            0x01 => self.sub(operand1, operand2),
            0x02 => self.mul(operand1, operand2),
            0x03 => self.div(operand1, operand2),
            0x04 => self.and(operand1, operand2),
            0x05 => self.or(operand1, operand2),
            0x06 => self.xor(operand1, operand2),
            0x07 => self.shift_left(operand1, operand2),
            0x08 => self.shift_right(operand1, operand2),
            _ => operand1,
        };
        
        self.update_flags(result);
        self.accumulator = result;
        self.busy = false;
        result
    }

    fn add(&mut self, a: u32, b: u32) -> u32 {
        let (result, carry) = a.overflowing_add(b);
        self.flags.carry = carry;
        result
    }

    fn sub(&mut self, a: u32, b: u32) -> u32 {
        let (result, carry) = a.overflowing_sub(b);
        self.flags.carry = carry;
        result
    }

    fn mul(&mut self, a: u32, b: u32) -> u32 {
        let (result, overflow) = a.overflowing_mul(b);
        self.flags.overflow = overflow;
        result
    }

    fn div(&mut self, a: u32, b: u32) -> u32 {
        if b == 0 {
            self.flags.overflow = true;
            return 0;
        }
        a.wrapping_div(b)
    }

    fn and(&self, a: u32, b: u32) -> u32 { a & b }
    fn or(&self, a: u32, b: u32) -> u32 { a | b }
    fn xor(&self, a: u32, b: u32) -> u32 { a ^ b }
    
    fn shift_left(&self, value: u32, shift: u32) -> u32 {
        value.wrapping_shl(shift)
    }
    
    fn shift_right(&self, value: u32, shift: u32) -> u32 {
        value.wrapping_shr(shift)
    }

    fn update_flags(&mut self, result: u32) {
        self.flags.zero = result == 0;
        self.flags.negative = (result as i32) < 0;
    }

    // Methods for visualization system to query state
    pub fn is_busy(&self) -> bool {
        self.busy
    }

    pub fn get_accumulator(&self) -> u32 {
        self.accumulator
    }

    pub fn get_flags(&self) -> (bool, bool, bool, bool) {
        (self.flags.zero, self.flags.carry, 
         self.flags.overflow, self.flags.negative)
    }
}
