use super::memory::MainMemory;
use super::cpu::CPU;
use super::gpu::GPU;
use super::power::Power;

pub mod arbitration;
pub mod memory_bus;
pub mod pci_bus;
pub mod system_bus;

pub struct Bus {
    memory_bus: memory_bus::MemoryBus,
    pci_bus: pci_bus::PciBus,
    system_bus: system_bus::SystemBus,
    arbitration: arbitration::BusArbiter,
    
    // Bus statistics for visualization
    total_transfers: u64,
    current_utilization: f32,
    bandwidth_usage: Vec<f32>,
}

#[derive(Debug)]
pub enum BusError {
    AddressOutOfRange,
    DeviceNotFound,
    BusUnavailable,
    TransactionTimeout,
    ArbitrationFailed,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory_bus: memory_bus::MemoryBus::new(),
            pci_bus: pci_bus::PciBus::new(),
            system_bus: system_bus::SystemBus::new(),
            arbitration: arbitration::BusArbiter::new(),
            total_transfers: 0,
            current_utilization: 0.0,
            bandwidth_usage: Vec::new(),
        }
    }

    pub fn read(&mut self, address: u32) -> Result<Option<u32>, BusError> {
        self.total_transfers += 1;
        
        // Check if bus is available through arbiter
        if !self.arbitration.is_bus_available() {
            return Err(BusError::BusUnavailable);
        }
        
        // Determine which bus to use based on address range
        match self.get_bus_for_address(address) {
            BusType::Memory => Ok(self.memory_bus.read(address)),
            BusType::PCI => Ok(self.pci_bus.read(address)),
            BusType::System => Ok(self.system_bus.read(address)),
        }
    }

    pub fn write(&mut self, address: u32, data: u32) -> Result<(), BusError> {
        self.total_transfers += 1;
        
        if !self.arbitration.is_bus_available() {
            return Err(BusError::BusUnavailable);
        }

        match self.get_bus_for_address(address) {
            BusType::Memory => {
                self.memory_bus.write(address, data);
                Ok(())
            },
            BusType::PCI => {
                self.pci_bus.write(address, data);
                Ok(())
            },
            BusType::System => {
                self.system_bus.write(address, data);
                Ok(())
            },
        }
    }

    fn get_bus_for_address(&self, address: u32) -> BusType {
        match address {
            0x0000_0000..=0x7FFF_FFFF => BusType::Memory, // First 2GB for main memory
            0x8000_0000..=0xBFFF_FFFF => BusType::PCI,    // Next 1GB for PCI devices
            _ => BusType::System,                          // Rest for system devices
        }
    }

    // Methods for visualization system
    pub fn get_utilization(&self) -> f32 {
        self.current_utilization
    }

    pub fn get_bandwidth_history(&self) -> &[f32] {
        &self.bandwidth_usage
    }

    pub fn get_total_transfers(&self) -> u64 {
        self.total_transfers
    }

    pub fn tick(&mut self) {
        // Update all bus components
        self.memory_bus.tick();
        self.pci_bus.tick();
        self.system_bus.tick();
        self.arbitration.tick();
        
        // Update statistics
        self.update_utilization();
        self.update_bandwidth_history();
    }

    fn update_utilization(&mut self) {
        // Calculate current bus utilization
        let total_pending = 
            self.memory_bus.pending_requests.len() +
            self.pci_bus.pending_transactions.len() +
            self.system_bus.pending_operations.len();
        
        self.current_utilization = total_pending as f32 / 100.0;
    }

    fn update_bandwidth_history(&mut self) {
        // Track bandwidth usage over time
        self.bandwidth_usage.push(self.current_utilization);
        if self.bandwidth_usage.len() > 100 {
            self.bandwidth_usage.remove(0);
        }
    }
}

enum BusType {
    Memory,
    PCI,
    System,
}


