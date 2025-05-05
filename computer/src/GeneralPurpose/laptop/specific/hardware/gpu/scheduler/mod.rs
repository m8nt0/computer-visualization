// Export all modules in scheduler
pub mod dispatcher;
pub mod workload;

pub struct Scheduler {
    dispatcher: dispatcher::Dispatcher,
    workload: workload::Workload,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            dispatcher: dispatcher::Dispatcher::new(),
            workload: workload::Workload::new(),
        }
    }
}   