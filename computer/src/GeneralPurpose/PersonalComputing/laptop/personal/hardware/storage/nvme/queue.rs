use super::protocol::{NVMeCommand, NVMeCompletion};
use super::super::error::{StorageError, StorageResult};
use std::collections::VecDeque;

pub struct SubmissionQueue {
    id: u16,
    entries: VecDeque<NVMeCommand>,
    head: u16,
    tail: u16,
    size: u16,
    phase: bool,
    stats: QueueStats,
}

pub struct CompletionQueue {
    id: u16,
    entries: VecDeque<NVMeCompletion>,
    head: u16,
    tail: u16,
    size: u16,
    phase: bool,
    stats: QueueStats,
}

struct QueueStats {
    entries_submitted: u64,
    entries_completed: u64,
    overflows: u64,
    underflows: u64,
}

impl SubmissionQueue {
    pub fn new(id: u16, size: u16) -> Self {
        Self {
            id,
            entries: VecDeque::with_capacity(size as usize),
            head: 0,
            tail: 0,
            size,
            phase: true,
            stats: QueueStats::default(),
        }
    }

    pub fn submit(&mut self, command: NVMeCommand) -> StorageResult<()> {
        if self.is_full() {
            self.stats.overflows += 1;
            return Err(StorageError::QueueFull);
        }

        self.entries.push_back(command);
        self.tail = (self.tail + 1) % self.size;
        self.stats.entries_submitted += 1;

        Ok(())
    }

    pub fn update_head(&mut self, new_head: u16) {
        self.head = new_head;
        // Remove completed entries
        while self.head != self.tail {
            self.entries.pop_front();
            self.head = (self.head + 1) % self.size;
        }
    }

    fn is_full(&self) -> bool {
        ((self.tail + 1) % self.size) == self.head
    }
}

impl CompletionQueue {
    pub fn new(id: u16, size: u16) -> Self {
        Self {
            id,
            entries: VecDeque::with_capacity(size as usize),
            head: 0,
            tail: 0,
            size,
            phase: true,
            stats: QueueStats::default(),
        }
    }

    pub fn poll(&mut self) -> StorageResult<Option<NVMeCompletion>> {
        if self.is_empty() {
            return Ok(None);
        }

        let completion = self.entries.pop_front()
            .ok_or(StorageError::QueueEmpty)?;

        self.head = (self.head + 1) % self.size;
        if self.head == 0 {
            self.phase = !self.phase;
        }

        self.stats.entries_completed += 1;
        Ok(Some(completion))
    }

    pub fn push_completion(&mut self, completion: NVMeCompletion) -> StorageResult<()> {
        if self.is_full() {
            self.stats.overflows += 1;
            return Err(StorageError::QueueFull);
        }

        self.entries.push_back(completion);
        self.tail = (self.tail + 1) % self.size;
        
        Ok(())
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn is_full(&self) -> bool {
        ((self.tail + 1) % self.size) == self.head
    }
}
