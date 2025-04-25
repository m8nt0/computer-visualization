pub mod process;
pub mod memory;
pub mod scheduler;

pub struct Kernel {
    scheduler: scheduler::Scheduler,
    memory_manager: memory::MemoryManager,
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            scheduler: scheduler::Scheduler::new(),
            memory_manager: memory::MemoryManager::new(),
        }
    }

    pub fn init(&mut self) {
        // Initialize kernel subsystems
    }

    pub fn create_process(&mut self, memory_start: u16, memory_size: u16) -> u32 {
        // Allocate memory for the process
        if let Some(_allocated_addr) = self.memory_manager.allocate(memory_size) {
            // Create and schedule the process
            self.scheduler.create_process(memory_start, memory_size)
        } else {
            panic!("Failed to allocate memory for process")
        }
    }
} 