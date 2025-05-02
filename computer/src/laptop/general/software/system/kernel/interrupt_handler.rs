use super::error::{KernelError, KernelResult};
use std::collections::HashMap;

pub struct InterruptController {
    handlers: HashMap<InterruptVector, InterruptHandler>,
    enabled_interrupts: u64,
    current_interrupt: Option<InterruptContext>,
    nesting_level: u32,
}

struct InterruptContext {
    vector: InterruptVector,
    saved_state: ProcessorState,
    error_code: Option<u32>,
}

type InterruptHandler = Box<dyn Fn(&InterruptContext) -> KernelResult<()>>;

impl InterruptController {
    pub fn register_handler(&mut self, vector: InterruptVector, handler: InterruptHandler) -> KernelResult<()> {
        if self.handlers.contains_key(&vector) {
            return Err(KernelError::HandlerExists);
        }
        self.handlers.insert(vector, handler);
        Ok(())
    }

    pub fn handle_interrupt(&mut self, vector: InterruptVector, error_code: Option<u32>) -> KernelResult<()> {
        let saved_state = self.save_processor_state();
        
        let context = InterruptContext {
            vector,
            saved_state,
            error_code,
        };

        self.nesting_level += 1;
        self.current_interrupt = Some(context);

        let result = if let Some(handler) = self.handlers.get(&vector) {
            handler(&context)
        } else {
            Err(KernelError::NoHandler)
        };

        self.nesting_level -= 1;
        self.current_interrupt = None;
        self.restore_processor_state(saved_state);

        result
    }
} 