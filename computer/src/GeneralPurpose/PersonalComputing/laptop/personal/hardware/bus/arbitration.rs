use std::collections::VecDeque;

pub struct BusArbiter {
    // Queue of pending bus requests
    request_queue: VecDeque<BusRequest>,
    
    // Current bus owner
    current_owner: Option<DeviceId>,
    
    // Priority levels for different devices
    device_priorities: Vec<DevicePriority>,
    
    // Statistics for visualization
    total_requests: u64,
    total_wait_cycles: u64,
    conflicts: u64,
}

#[derive(Clone, Copy, PartialEq)]
pub struct DeviceId {
    device_type: DeviceType,
    id: u8,
}

#[derive(Clone, Copy)]
struct DevicePriority {
    device: DeviceId,
    base_priority: u8,
    current_priority: u8,  // Increases with wait time
}

#[derive(Clone, Copy)]
pub struct BusRequest {
    device: DeviceId,
    address: u32,
    request_type: RequestType,
    cycles_needed: u8,
    wait_cycles: u32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DeviceType {
    CPU,
    GPU,
    DMA,
    Peripheral,
}

#[derive(Clone, Copy)]
pub enum RequestType {
    Read,
    Write,
    DMA,
    IO,
}

impl BusArbiter {
    pub fn new() -> Self {
        let mut arbiter = Self {
            request_queue: VecDeque::new(),
            current_owner: None,
            device_priorities: Vec::new(),
            total_requests: 0,
            total_wait_cycles: 0,
            conflicts: 0,
        };

        // Set up default device priorities
        arbiter.device_priorities = vec![
            DevicePriority {
                device: DeviceId { device_type: DeviceType::CPU, id: 0 },
                base_priority: 3,
                current_priority: 3,
            },
            DevicePriority {
                device: DeviceId { device_type: DeviceType::GPU, id: 0 },
                base_priority: 2,
                current_priority: 2,
            },
            DevicePriority {
                device: DeviceId { device_type: DeviceType::DMA, id: 0 },
                base_priority: 1,
                current_priority: 1,
            },
        ];

        arbiter
    }

    pub fn request_bus(&mut self, device: DeviceId, address: u32, 
                      request_type: RequestType, cycles: u8) -> bool {
        self.total_requests += 1;

        // Check if bus is available
        if let Some(owner) = self.current_owner {
            if owner == device {
                return true; // Device already owns the bus
            }
            
            // Bus is busy, queue request
            self.conflicts += 1;
            self.request_queue.push_back(BusRequest {
                device,
                address,
                request_type,
                cycles_needed: cycles,
                wait_cycles: 0,
            });
            false
        } else {
            // Bus is free, grant immediately
            self.current_owner = Some(device);
            true
        }
    }

    pub fn tick(&mut self) {
        // Update wait times and priorities
        for request in &mut self.request_queue {
            request.wait_cycles += 1;
            self.total_wait_cycles += 1;
            
            // Increase priority of waiting devices
            if let Some(priority) = self.device_priorities.iter_mut()
                .find(|p| p.device == request.device) {
                priority.current_priority = priority.current_priority.saturating_add(1);
            }
        }

        // If bus is free, select next device
        if self.current_owner.is_none() && !self.request_queue.is_empty() {
            self.select_next_device();
        }
    }

    fn select_next_device(&mut self) {
        // Find highest priority request
        let mut highest_priority = 0;
        let mut selected_index = 0;

        for (i, request) in self.request_queue.iter().enumerate() {
            if let Some(priority) = self.device_priorities.iter()
                .find(|p| p.device == request.device) {
                let effective_priority = priority.current_priority + 
                    (request.wait_cycles / 10) as u8; // Age factor
                if effective_priority > highest_priority {
                    highest_priority = effective_priority;
                    selected_index = i;
                }
            }
        }

        // Grant bus to selected device
        if let Some(request) = self.request_queue.remove(selected_index) {
            self.current_owner = Some(request.device);
            // Reset priority for granted device
            if let Some(priority) = self.device_priorities.iter_mut()
                .find(|p| p.device == request.device) {
                priority.current_priority = priority.base_priority;
            }
        }
    }

    // Methods for visualization system
    pub fn get_current_owner(&self) -> Option<DeviceId> {
        self.current_owner
    }

    pub fn get_queue_length(&self) -> usize {
        self.request_queue.len()
    }

    pub fn get_average_wait_time(&self) -> f32 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.total_wait_cycles as f32 / self.total_requests as f32
    }

    pub fn get_conflict_rate(&self) -> f32 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.conflicts as f32 / self.total_requests as f32
    }

    pub fn is_bus_available(&self) -> bool {
        // Bus is available if:
        // 1. No current owner OR
        // 2. Queue is not full
        self.current_owner.is_none() || 
        self.request_queue.len() < 32  // Maximum queue depth
    }
}
