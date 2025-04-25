pub struct RegisterFile {
    // General purpose registers
    gprs: [u32; 32],
    
    // Special registers
    pc: u32,    // Program Counter
    sp: u32,    // Stack Pointer
    lr: u32,    // Link Register
    sr: u32,    // Status Register
    
    // Register activity tracking for visualization
    last_written: Option<usize>,
    last_read: Option<usize>,
}

impl RegisterFile {
    pub fn new() -> Self {
        Self {
            gprs: [0; 32],
            pc: 0,
            sp: 0xFFFF_FFF0, // Initialize stack pointer to near top of memory
            lr: 0,
            sr: 0,
            last_written: None,
            last_read: None,
        }
    }

    pub fn read_gpr(&mut self, index: usize) -> u32 {
        self.last_read = Some(index);
        self.gprs[index]
    }

    pub fn write_gpr(&mut self, index: usize, value: u32) {
        self.last_written = Some(index);
        self.gprs[index] = value;
    }

    pub fn get_pc(&self) -> u32 { self.pc }
    pub fn set_pc(&mut self, value: u32) { self.pc = value; }
    
    pub fn get_sp(&self) -> u32 { self.sp }
    pub fn set_sp(&mut self, value: u32) { self.sp = value; }
    
    pub fn get_lr(&self) -> u32 { self.lr }
    pub fn set_lr(&mut self, value: u32) { self.lr = value; }
    
    pub fn get_sr(&self) -> u32 { self.sr }
    pub fn set_sr(&mut self, value: u32) { self.sr = value; }

    // Methods for visualization system
    pub fn get_register_values(&self) -> &[u32] {
        &self.gprs
    }

    pub fn get_last_accessed(&self) -> (Option<usize>, Option<usize>) {
        (self.last_read, self.last_written)
    }

    pub fn clear_access_tracking(&mut self) {
        self.last_read = None;
        self.last_written = None;
    }
}
