use super::error::{KernelError, KernelResult};
use std::collections::{HashMap, VecDeque};

pub struct IpcManager {
    message_queues: HashMap<QueueId, MessageQueue>,
    shared_memory: HashMap<ShmId, SharedMemoryRegion>,
    semaphores: HashMap<SemId, Semaphore>,
}

struct MessageQueue {
    id: QueueId,
    messages: VecDeque<Message>,
    senders: Vec<ProcessId>,
    receivers: Vec<ProcessId>,
    max_size: usize,
}

impl IpcManager {
    pub fn send_message(&mut self, queue: QueueId, msg: Message) -> KernelResult<()> {
        let queue = self.message_queues.get_mut(&queue)
            .ok_or(KernelError::InvalidQueue)?;

        if queue.messages.len() >= queue.max_size {
            return Err(KernelError::QueueFull);
        }

        queue.messages.push_back(msg);
        self.wake_receivers(queue);
        Ok(())
    }

    pub fn receive_message(&mut self, queue: QueueId) -> KernelResult<Message> {
        let queue = self.message_queues.get_mut(&queue)
            .ok_or(KernelError::InvalidQueue)?;

        if let Some(msg) = queue.messages.pop_front() {
            self.wake_senders(queue);
            Ok(msg)
        } else {
            Err(KernelError::QueueEmpty)
        }
    }
} 