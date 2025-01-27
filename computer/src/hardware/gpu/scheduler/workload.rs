use super::super::error::GPUResult;

#[derive(Clone)]
pub struct Workload {
    pub id: u64,
    pub workload_type: WorkloadType,
    pub priority: Priority,
    pub submit_time: u64,
    pub deadline: Option<u64>,
    pub dependencies: Vec<u64>,
    pub state: WorkloadState,
    pub stats: WorkloadStats,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WorkloadType {
    Shader,
    RayTracing,
    Tensor,
}

#[derive(Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
pub enum Priority {
    Low,
    Normal,
    High,
    RealTime,
}

#[derive(Clone, Copy, PartialEq)]
pub enum WorkloadState {
    Pending,
    Running,
    Preempted,
    Completed,
    Failed,
}

#[derive(Clone)]
pub struct WorkloadStats {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub preemptions: u32,
    pub core_cycles: u64,
    pub memory_bytes: u64,
}

impl Workload {
    pub fn new(id: u64, workload_type: WorkloadType, priority: Priority) -> Self {
        Self {
            id,
            workload_type,
            priority,
            submit_time: 0,
            deadline: None,
            dependencies: Vec::new(),
            state: WorkloadState::Pending,
            stats: WorkloadStats::default(),
        }
    }

    pub fn set_deadline(&mut self, deadline: u64) {
        self.deadline = Some(deadline);
    }

    pub fn add_dependency(&mut self, dependency_id: u64) {
        self.dependencies.push(dependency_id);
    }

    pub fn is_ready(&self, completed_workloads: &[u64]) -> bool {
        self.dependencies.iter().all(|dep| completed_workloads.contains(dep))
    }

    pub fn start(&mut self, current_time: u64) {
        self.state = WorkloadState::Running;
        self.stats.start_time = Some(current_time);
    }

    pub fn complete(&mut self, current_time: u64) {
        self.state = WorkloadState::Completed;
        self.stats.end_time = Some(current_time);
    }

    pub fn preempt(&mut self) {
        self.state = WorkloadState::Preempted;
        self.stats.preemptions += 1;
    }
}
