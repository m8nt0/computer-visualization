#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

pub struct Process {
    pub id: u32,
    pub state: ProcessState,
    pub priority: u8,
    pub memory_start: u16,
    pub memory_size: u16,
    pub program_counter: u16,
}

impl Process {
    pub fn new(id: u32, memory_start: u16, memory_size: u16) -> Self {
        Self {
            id,
            state: ProcessState::Ready,
            priority: 1,
            memory_start,
            memory_size,
            program_counter: memory_start,
        }
    }
}
