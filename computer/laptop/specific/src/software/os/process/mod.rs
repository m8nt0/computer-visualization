use super::error::{ProcessError, ProcessResult};
use std::collections::HashMap;

pub struct ProcessManager {
    processes: HashMap<Pid, Process>,
    threads: HashMap<Tid, Thread>,
    scheduler: Scheduler,
    memory_manager: MemoryManager,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pid(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tid(u32);

pub struct Process {
    pid: Pid,
    parent: Option<Pid>,
    state: ProcessState,
    threads: Vec<Tid>,
    memory_map: MemoryMap,
    file_handles: Vec<FileHandle>,
    exit_code: Option<i32>,
}

pub struct Thread {
    tid: Tid,
    pid: Pid,
    state: ThreadState,
    context: ThreadContext,
    stack: Vec<u8>,
    priority: u8,
}

struct ThreadContext {
    registers: [u64; 16],
    program_counter: u64,
    stack_pointer: u64,
    cpu_flags: u64,
}

impl ProcessManager {
    pub fn new(scheduler: Scheduler, memory_manager: MemoryManager) -> Self {
        Self {
            processes: HashMap::new(),
            threads: HashMap::new(),
            scheduler,
            memory_manager,
        }
    }

    pub fn create_process(&mut self, executable: &[u8], args: &[String]) -> ProcessResult<Pid> {
        // Allocate memory for the process
        let memory_map = self.memory_manager.create_memory_map(executable)?;
        
        // Create process structure
        let pid = self.generate_pid();
        let process = Process::new(pid, memory_map);
        
        // Create main thread
        let main_thread = self.create_thread(pid, process.memory_map.entry_point)?;
        process.threads.push(main_thread);
        
        // Register process
        self.processes.insert(pid, process);
        
        // Schedule main thread
        self.scheduler.add_thread(main_thread);
        
        Ok(pid)
    }

    pub fn create_thread(&mut self, pid: Pid, entry_point: u64) -> ProcessResult<Tid> {
        let process = self.processes.get_mut(&pid)
            .ok_or(ProcessError::ProcessNotFound)?;
            
        let tid = self.generate_tid();
        let stack = self.memory_manager.allocate_stack()?;
        
        let thread = Thread::new(tid, pid, entry_point, stack);
        self.threads.insert(tid, thread);
        
        Ok(tid)
    }

    pub fn terminate_process(&mut self, pid: Pid, exit_code: i32) -> ProcessResult<()> {
        let process = self.processes.remove(&pid)
            .ok_or(ProcessError::ProcessNotFound)?;
            
        // Clean up threads
        for tid in process.threads {
            self.terminate_thread(tid)?;
        }
        
        // Free resources
        self.memory_manager.free_memory_map(process.memory_map);
        for handle in process.file_handles {
            self.close_file_handle(handle);
        }
        
        Ok(())
    }

    fn generate_pid(&self) -> Pid {
        Pid(self.processes.len() as u32)
    }

    fn generate_tid(&self) -> Tid {
        Tid(self.threads.len() as u32)
    }
} 