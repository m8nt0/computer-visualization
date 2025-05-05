use std::collections::VecDeque;
use super::super::error::{MemoryError, MemoryResult};
use super::super::types::PhysicalAddress;

pub struct CommandQueue {
    commands: VecDeque<Command>,
    max_size: usize,
    stats: QueueStats,
}

#[derive(Clone)]
pub struct Command {
    pub cmd_type: CommandType,
    pub address: PhysicalAddress,
    pub data: Option<u32>,
    pub priority: Priority,
    pub timestamp: u64,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CommandType {
    Read,
    Write,
    Activate,
    Precharge,
    Refresh,
    PowerDown,
    PowerUp,
}

#[derive(Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub enum Priority {
    Critical,
    High,
    Normal,
    Low,
}

struct QueueStats {
    total_commands: u64,
    queue_full_events: u64,
    average_latency: f32,
    max_queue_depth: usize,
}

impl CommandQueue {
    pub fn new() -> Self {
        Self {
            commands: VecDeque::with_capacity(32),
            max_size: 32,
            stats: QueueStats::default(),
        }
    }

    pub fn push(&mut self, cmd: Command) -> MemoryResult<()> {
        if self.commands.len() >= self.max_size {
            self.stats.queue_full_events += 1;
            return Err(MemoryError::CommandQueueFull);
        }

        self.stats.total_commands += 1;
        self.stats.max_queue_depth = self.stats.max_queue_depth.max(self.commands.len() + 1);
        
        self.commands.push_back(cmd);
        Ok(())
    }

    pub fn peek(&self) -> Option<&Command> {
        self.commands.front()
    }

    pub fn pop(&mut self) -> Option<Command> {
        self.commands.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.commands.len() >= self.max_size
    }

    pub fn reorder(&mut self) {
        // Sort by priority and age
        let mut vec: Vec<_> = self.commands.drain(..).collect();
        vec.sort_by_key(|cmd| (std::cmp::Reverse(cmd.priority), cmd.timestamp));
        self.commands.extend(vec);
    }
}
