use super::super::error::{StorageError, StorageResult};

#[derive(Clone)]
pub struct NVMeCommand {
    opcode: CommandOpcode,
    namespace_id: u32,
    command_id: u16,
    flags: CommandFlags,
    metadata_ptr: u64,
    prp1: u64,
    prp2: u64,
    command_specific: [u32; 6],
}

#[derive(Clone)]
pub struct NVMeCompletion {
    command_id: u16,
    status: NVMeStatus,
    phase_bit: bool,
    sq_head_ptr: u16,
    sq_id: u16,
    command_specific: u32,
}

#[derive(Clone, Copy)]
pub enum CommandOpcode {
    // Admin Commands
    DeleteIOSQ = 0x00,
    CreateIOSQ = 0x01,
    DeleteIOCQ = 0x04,
    CreateIOCQ = 0x05,
    Identify = 0x06,
    GetFeatures = 0x0A,
    SetFeatures = 0x09,
    
    // IO Commands
    Flush = 0x00,
    Write = 0x01,
    Read = 0x02,
    WriteUncorrectable = 0x04,
    Compare = 0x05,
    WriteZeroes = 0x08,
    DatasetManagement = 0x09,
}

bitflags! {
    pub struct CommandFlags: u16 {
        const NONE = 0x0;
        const PRP_EXTENDED = 0x1;
        const SGL_EXTENDED = 0x2;
        const FUSED_FIRST = 0x4;
        const FUSED_SECOND = 0x8;
    }
}

#[derive(Clone, Copy)]
pub struct NVMeStatus {
    phase_tag: bool,
    status_code: u8,
    status_code_type: u8,
    more: bool,
    do_not_retry: bool,
}

impl NVMeCommand {
    pub fn new(opcode: CommandOpcode) -> Self {
        Self {
            opcode,
            namespace_id: 0,
            command_id: 0,
            flags: CommandFlags::NONE,
            metadata_ptr: 0,
            prp1: 0,
            prp2: 0,
            command_specific: [0; 6],
        }
    }

    pub fn new_identify() -> Self {
        let mut cmd = Self::new(CommandOpcode::Identify);
        cmd.command_specific[0] = 1; // CNS = 1 for controller identify
        cmd
    }

    pub fn new_read(lba: u64, num_blocks: u16) -> Self {
        let mut cmd = Self::new(CommandOpcode::Read);
        cmd.command_specific[0] = (lba & 0xFFFFFFFF) as u32;
        cmd.command_specific[1] = (lba >> 32) as u32;
        cmd.command_specific[2] = num_blocks as u32;
        cmd
    }

    pub fn new_write(lba: u64, num_blocks: u16) -> Self {
        let mut cmd = Self::new(CommandOpcode::Write);
        cmd.command_specific[0] = (lba & 0xFFFFFFFF) as u32;
        cmd.command_specific[1] = (lba >> 32) as u32;
        cmd.command_specific[2] = num_blocks as u32;
        cmd
    }

    pub fn set_prp_entries(&mut self, prp1: u64, prp2: u64) {
        self.prp1 = prp1;
        self.prp2 = prp2;
    }

    pub fn get_queue_id(&self) -> u16 {
        (self.command_specific[0] >> 16) as u16
    }
}
