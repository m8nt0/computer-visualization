use super::error::{KernelError, KernelResult};

pub struct SyscallHandler {
    process_manager: ProcessManager,
    memory_manager: MemoryManager,
    file_system: FileSystem,
    network_stack: NetworkStack,
}

impl SyscallHandler {
    pub fn handle_syscall(&mut self, syscall: Syscall) -> KernelResult<SyscallResult> {
        match syscall {
            Syscall::Fork => self.handle_fork(),
            Syscall::Exec { path, args } => self.handle_exec(path, args),
            Syscall::Exit { code } => self.handle_exit(code),
            Syscall::Read { fd, buf } => self.handle_read(fd, buf),
            Syscall::Write { fd, buf } => self.handle_write(fd, buf),
            Syscall::Open { path, flags } => self.handle_open(path, flags),
            Syscall::Close { fd } => self.handle_close(fd),
            Syscall::Socket { domain, type_, protocol } => self.handle_socket(domain, type_, protocol),
            Syscall::Connect { socket, addr } => self.handle_connect(socket, addr),
            _ => Err(KernelError::InvalidSyscall),
        }
    }

    fn handle_fork(&mut self) -> KernelResult<SyscallResult> {
        let child_pid = self.process_manager.fork_current_process()?;
        Ok(SyscallResult::Pid(child_pid))
    }

    fn handle_exec(&mut self, path: &str, args: &[&str]) -> KernelResult<SyscallResult> {
        self.process_manager.exec_process(path, args)?;
        Ok(SyscallResult::Success)
    }
} 