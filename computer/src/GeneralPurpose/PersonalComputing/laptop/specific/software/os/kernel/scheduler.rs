use super::process::{Process, ProcessState};
use super::error::{KernelError, KernelResult};
use std::collections::{BinaryHeap, HashMap};
use std::time::Duration;

pub struct Scheduler {
    ready_queue: BinaryHeap<Process>,
    waiting_queues: HashMap<WaitReason, Vec<Process>>,
    current_process: Option<Process>,
    time_slice: Duration,
    policy: SchedulingPolicy,
}

enum SchedulingPolicy {
    RoundRobin { quantum: Duration },
    Priority { levels: u8 },
    MultiLevel { queues: Vec<ProcessQueue> },
}

impl Scheduler {
    pub fn schedule(&mut self) -> KernelResult<Option<Process>> {
        // Handle current process
        if let Some(mut current) = self.current_process.take() {
            if current.state == ProcessState::Running {
                current.state = ProcessState::Ready;
                self.ready_queue.push(current);
            }
        }

        // Select next process
        if let Some(mut next) = self.ready_queue.pop() {
            next.state = ProcessState::Running;
            self.current_process = Some(next.clone());
            Ok(Some(next))
        } else {
            Ok(None)
        }
    }
}
