use super::super::error::{GPUError, GPUResult};
use super::cache::GPUCache;
use super::vram::VRAMController;
use std::collections::VecDeque;

pub struct MemoryController {
    cache: GPUCache,
    vram: VRAMController,
    request_queue: RequestQueue,
    scheduler: MemoryScheduler,
    stats: ControllerStats,
}

struct RequestQueue {
    read_queue: VecDeque<MemoryRequest>,
    write_queue: VecDeque<MemoryRequest>,
    max_queue_size: usize,
}

struct MemoryRequest {
    address: u64,
    size: usize,
    request_type: RequestType,
    priority: RequestPriority,
    timestamp: u64,
}

#[derive(Clone, Copy)]
enum RequestType {
    Read,
    Write,
    Prefetch,
    Writeback,
}

#[derive(Clone, Copy, PartialEq, Ord, PartialOrd, Eq)]
enum RequestPriority {
    Critical,
    High,
    Normal,
    Low,
}

struct MemoryScheduler {
    scheduling_policy: SchedulingPolicy,
    bank_states: Vec<BankState>,
    current_cycle: u64,
}

#[derive(Clone, Copy)]
enum SchedulingPolicy {
    FirstComeFirstServed,
    FrFcfs, // First-Ready First-Come-First-Served
    BankRoundRobin,
    PowerAware,
}

struct BankState {
    active_row: Option<u64>,
    last_access: u64,
    access_count: u64,
    power_state: PowerState,
}

#[derive(Clone, Copy)]
enum PowerState {
    Active,
    Precharge,
    PowerDown,
    SelfRefresh,
}

struct ControllerStats {
    total_requests: u64,
    read_requests: u64,
    write_requests: u64,
    queue_full_events: u64,
    bank_conflicts: u64,
    average_latency: f32,
    bandwidth_usage: f32,
}

impl MemoryController {
    pub fn new(cache_configs: Vec<CacheConfig>) -> Self {
        Self {
            cache: GPUCache::new(cache_configs),
            vram: VRAMController::new(),
            request_queue: RequestQueue::new(),
            scheduler: MemoryScheduler::new(),
            stats: ControllerStats::default(),
        }
    }

    pub fn read(&mut self, address: u64, size: usize) -> GPUResult<Vec<u8>> {
        self.stats.total_requests += 1;
        self.stats.read_requests += 1;

        // Try cache first
        if let Ok(data) = self.cache.read(address) {
            return Ok(data);
        }

        // Queue memory request
        let request = MemoryRequest {
            address,
            size,
            request_type: RequestType::Read,
            priority: RequestPriority::Normal,
            timestamp: self.scheduler.current_cycle,
        };

        self.request_queue.push_read(request)?;
        self.process_requests()
    }

    pub fn write(&mut self, address: u64, data: &[u8]) -> GPUResult<()> {
        self.stats.total_requests += 1;
        self.stats.write_requests += 1;

        // Write through cache
        self.cache.write(address, data)?;

        // Queue memory request
        let request = MemoryRequest {
            address,
            size: data.len(),
            request_type: RequestType::Write,
            priority: RequestPriority::Normal,
            timestamp: self.scheduler.current_cycle,
        };

        self.request_queue.push_write(request)?;
        self.process_requests()
    }

    pub fn tick(&mut self) {
        self.scheduler.current_cycle += 1;
        self.process_requests().ok();
        self.update_stats();
    }

    fn process_requests(&mut self) -> GPUResult<()> {
        while let Some(request) = self.scheduler.schedule_next(&self.request_queue) {
            match request.request_type {
                RequestType::Read => {
                    let data = self.vram.read(request.address, request.size)?;
                    self.cache.update(request.address, &data)?;
                }
                RequestType::Write => {
                    // Data already in cache, just write to VRAM
                    self.vram.write(request.address, &[0; 64])?; // Simplified
                }
                RequestType::Prefetch => {
                    // Handle prefetch
                }
                RequestType::Writeback => {
                    // Handle writeback
                }
            }
        }
        Ok(())
    }

    fn update_stats(&mut self) {
        // Update performance metrics
        self.stats.average_latency = self.calculate_average_latency();
        self.stats.bandwidth_usage = self.calculate_bandwidth_usage();
    }
}

impl RequestQueue {
    fn new() -> Self {
        Self {
            read_queue: VecDeque::new(),
            write_queue: VecDeque::new(),
            max_queue_size: 64,
        }
    }

    fn push_read(&mut self, request: MemoryRequest) -> GPUResult<()> {
        if self.read_queue.len() >= self.max_queue_size {
            return Err(GPUError::QueueFull);
        }
        self.read_queue.push_back(request);
        Ok(())
    }

    fn push_write(&mut self, request: MemoryRequest) -> GPUResult<()> {
        if self.write_queue.len() >= self.max_queue_size {
            return Err(GPUError::QueueFull);
        }
        self.write_queue.push_back(request);
        Ok(())
    }
}

impl MemoryScheduler {
    fn new() -> Self {
        Self {
            scheduling_policy: SchedulingPolicy::FrFcfs,
            bank_states: vec![BankState::new(); 16], // 16 banks
            current_cycle: 0,
        }
    }

    fn schedule_next(&mut self, queue: &RequestQueue) -> Option<MemoryRequest> {
        match self.scheduling_policy {
            SchedulingPolicy::FirstComeFirstServed => self.schedule_fcfs(queue),
            SchedulingPolicy::FrFcfs => self.schedule_frfcfs(queue),
            SchedulingPolicy::BankRoundRobin => self.schedule_round_robin(queue),
            SchedulingPolicy::PowerAware => self.schedule_power_aware(queue),
        }
    }

    fn schedule_fcfs(&mut self, queue: &RequestQueue) -> Option<MemoryRequest> {
        // First come first served - just take the oldest request
        if !queue.read_queue.is_empty() {
            return Some(queue.read_queue[0].clone());
        }
        if !queue.write_queue.is_empty() {
            return Some(queue.write_queue[0].clone());
        }
        None
    }

    fn schedule_frfcfs(&mut self, queue: &RequestQueue) -> Option<MemoryRequest> {
        // First ready, first come first served
        // Prioritize row buffer hits, then oldest request
        let ready_read = self.find_row_hit(&queue.read_queue);
        let ready_write = self.find_row_hit(&queue.write_queue);

        match (ready_read, ready_write) {
            (Some(read), Some(write)) => {
                // Choose the older request if both are row hits
                if read.timestamp < write.timestamp {
                    Some(read)
                } else {
                    Some(write)
                }
            }
            (Some(read), None) => Some(read),
            (None, Some(write)) => Some(write),
            (None, None) => self.schedule_fcfs(queue),
        }
    }

    fn schedule_round_robin(&mut self, queue: &RequestQueue) -> Option<MemoryRequest> {
        // Round robin between banks
        let mut selected_bank = None;
        
        // Find next bank with pending requests
        for bank_id in 0..self.bank_states.len() {
            if self.has_pending_requests(bank_id, queue) {
                selected_bank = Some(bank_id);
                break;
            }
        }

        if let Some(bank_id) = selected_bank {
            self.find_bank_request(bank_id, queue)
        } else {
            None
        }
    }

    fn schedule_power_aware(&mut self, queue: &RequestQueue) -> Option<MemoryRequest> {
        // Power-aware scheduling - try to minimize bank/rank power state transitions
        let mut best_request = None;
        let mut min_power_cost = f32::MAX;

        // Check all pending requests
        for request in queue.read_queue.iter().chain(queue.write_queue.iter()) {
            let bank_id = self.get_bank_id(request.address);
            let power_cost = self.calculate_power_cost(bank_id, request);

            if power_cost < min_power_cost {
                min_power_cost = power_cost;
                best_request = Some(request.clone());
            }
        }

        best_request
    }

    // Helper methods
    fn find_row_hit(&self, queue: &VecDeque<MemoryRequest>) -> Option<MemoryRequest> {
        queue.iter().find(|req| {
            let bank_id = self.get_bank_id(req.address);
            let row = self.get_row(req.address);
            let bank = &self.bank_states[bank_id];
            bank.active_row == Some(row)
        }).cloned()
    }

    fn has_pending_requests(&self, bank_id: usize, queue: &RequestQueue) -> bool {
        queue.read_queue.iter().chain(queue.write_queue.iter())
            .any(|req| self.get_bank_id(req.address) == bank_id)
    }

    fn find_bank_request(&self, bank_id: usize, queue: &RequestQueue) -> Option<MemoryRequest> {
        queue.read_queue.iter().chain(queue.write_queue.iter())
            .find(|req| self.get_bank_id(req.address) == bank_id)
            .cloned()
    }

    fn calculate_power_cost(&self, bank_id: usize, request: &MemoryRequest) -> f32 {
        let bank = &self.bank_states[bank_id];
        let mut cost = 0.0;

        // Add power state transition cost
        cost += match bank.power_state {
            PowerState::PowerDown | PowerState::SelfRefresh => 10.0, // High cost to wake up
            PowerState::Precharge => 5.0,  // Medium cost to activate
            PowerState::Active => 1.0,     // Low cost if already active
        };

        // Add row activation cost if needed
        let row = self.get_row(request.address);
        if bank.active_row != Some(row) {
            cost += 5.0; // Row activation cost
        }

        cost
    }

    fn get_bank_id(&self, address: u64) -> usize {
        ((address >> 13) & 0xF) as usize // Extract bank bits
    }

    fn get_row(&self, address: u64) -> u64 {
        (address >> 17) & 0xFFFF // Extract row bits
    }
}

impl BankState {
    fn new() -> Self {
        Self {
            active_row: None,
            last_access: 0,
            access_count: 0,
            power_state: PowerState::Precharge,
        }
    }
}

impl MemoryController {
    // Add missing helper methods
    fn calculate_average_latency(&self) -> f32 {
        // Calculate average request latency
        if self.stats.total_requests == 0 {
            0.0
        } else {
            // Simplified latency calculation
            let total_cycles = self.scheduler.current_cycle;
            total_cycles as f32 / self.stats.total_requests as f32
        }
    }

    fn calculate_bandwidth_usage(&self) -> f32 {
        // Calculate current bandwidth usage
        let window_size = 1000; // Look at last 1000 cycles
        let recent_bytes = (self.stats.read_requests + self.stats.write_requests) * 64;
        recent_bytes as f32 / window_size as f32
    }
} 