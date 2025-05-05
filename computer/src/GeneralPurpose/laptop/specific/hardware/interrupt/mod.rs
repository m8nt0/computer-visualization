use super::error::{InterruptError, InterruptResult};
use std::collections::VecDeque;

pub struct InterruptController {
    pending: VecDeque<Interrupt>,
    handlers: Vec<InterruptHandler>,
    mask: InterruptMask,
    config: InterruptConfig,
    stats: InterruptStats,
}

struct Interrupt {
    id: InterruptID,
    priority: u8,
    data: Vec<u8>,
    timestamp: u64,
}

type InterruptHandler = Box<dyn Fn(&Interrupt) -> InterruptResult<()>>;

impl InterruptController {
    pub fn new(config: InterruptConfig) -> Self {
        Self {
            pending: VecDeque::new(),
            handlers: Vec::new(),
            mask: InterruptMask::default(),
            config,
            stats: InterruptStats::default(),
        }
    }

    pub fn raise_interrupt(&mut self, interrupt: Interrupt) -> InterruptResult<()> {
        if !self.mask.is_enabled(interrupt.id) {
            return Err(InterruptError::Masked);
        }

        self.pending.push_back(interrupt);
        self.stats.interrupts_raised += 1;
        Ok(())
    }

    pub fn handle_pending(&mut self) {
        while let Some(interrupt) = self.pending.pop_front() {
            if let Some(handler) = self.handlers.get(interrupt.id.0 as usize) {
                if let Ok(()) = handler(&interrupt) {
                    self.stats.interrupts_handled += 1;
                } else {
                    self.stats.interrupts_failed += 1;
                }
            }
        }
    }
} 