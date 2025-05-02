pub mod cooling;
pub mod memory;
pub mod peripherals;
pub mod power;
pub mod processing;
pub mod storage;
pub mod firmware;
pub mod visualization;

// Public exports for convenient usage
pub use self::processing::cpu::CPU;
pub use self::processing::gpu::GPU;
pub use self::memory::ram::RAM;
pub use self::memory::cache::Cache;
pub use self::storage::ssd::SSD;
pub use self::peripherals::input::keyboard::Keyboard;
pub use self::peripherals::input::touchpad::Touchpad;
pub use self::peripherals::output::display::Display;
pub use self::cooling::CoolingSystem;
pub use self::power::PowerManagement;

// Default error types
pub mod error {
    pub type HardwareResult<T> = Result<T, HardwareError>;
    
    #[derive(Debug)]
    pub enum HardwareError {
        NotInitialized,
        MemoryError(MemoryError),
        StorageError(StorageError),
        PowerError(PowerError),
        IOError(IOError),
        ProcessingError(ProcessingError),
    }
    
    #[derive(Debug)]
    pub enum MemoryError {
        OutOfMemory,
        PermissionDenied,
        InvalidAddress,
        CacheMiss,
    }
    
    #[derive(Debug)]
    pub enum StorageError {
        DiskFull,
        FileNotFound,
        PermissionDenied,
        ReadError,
        WriteError,
    }
    
    #[derive(Debug)]
    pub enum PowerError {
        Overheat,
        LowBattery,
        PowerSurge,
        NoPower,
    }
    
    #[derive(Debug)]
    pub enum IOError {
        DeviceNotFound,
        Timeout,
        TransferError,
        UnsupportedOperation,
        UnsupportedMode,
        InvalidBufferSize,
    }
    
    #[derive(Debug)]
    pub enum ProcessingError {
        InvalidInstruction,
        IllegalOperation,
        DivideByZero,
        Overflow,
        Underflow,
    }
}

// Common hardware types
pub mod types {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct PhysicalAddress(pub u64);
    
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct VirtualAddress(pub u64);
    
    #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
    pub struct ProcessId(pub u32);
}

// Implement the bus system that connects components
pub mod bus {
    use super::types::PhysicalAddress;
    
    pub struct Bus {
        // Bus connections and routing
        address_lines: u64,
        data_lines: u64,
        control_lines: u32,
        transactions: Vec<BusTransaction>,
    }
    
    struct BusTransaction {
        source: BusDevice,
        destination: BusDevice,
        address: PhysicalAddress,
        data: Option<Vec<u8>>,
        transaction_type: TransactionType,
        completed: bool,
    }
    
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum BusDevice {
        CPU,
        Memory,
        Storage,
        GPU,
        IO,
        DMA,
    }
    
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    enum TransactionType {
        Read,
        Write,
        IO,
        Interrupt,
    }
    
    impl Bus {
        pub fn new() -> Self {
            Self {
                address_lines: 0,
                data_lines: 0,
                control_lines: 0,
                transactions: Vec::new(),
            }
        }
        
        pub fn read(&mut self, device: BusDevice, address: PhysicalAddress) -> Vec<u8> {
            // Simulate bus read operation
            self.transactions.push(BusTransaction {
                source: device,
                destination: self.map_address_to_device(&address),
                address,
                data: None,
                transaction_type: TransactionType::Read,
                completed: false,
            });
            
            // For simulation, just return empty data
            vec![0; 8]
        }
        
        pub fn write(&mut self, device: BusDevice, address: PhysicalAddress, data: &[u8]) {
            // Simulate bus write operation
            self.transactions.push(BusTransaction {
                source: device,
                destination: self.map_address_to_device(&address),
                address,
                data: Some(data.to_vec()),
                transaction_type: TransactionType::Write,
                completed: false,
            });
        }
        
        fn map_address_to_device(&self, address: &PhysicalAddress) -> BusDevice {
            // Map physical address to corresponding device
            // This is a simplified version
            let addr = address.0;
            
            match addr >> 60 {
                0 => BusDevice::Memory,
                1 => BusDevice::Storage,
                2 => BusDevice::GPU,
                3 => BusDevice::IO,
                _ => BusDevice::Memory,
            }
        }
    }
}

pub struct Hardware {
    bus: Bus,
    cpu: CPU,
    memory: Memory,
    gpu: GPU,
    storage: Storage,
    io: IOSystem,
}

impl Hardware {
    pub fn new() -> Self {
        let mut bus = Bus::new();

        // Initialize components and register them with the bus
        let cpu = CPU::new(&mut bus);
        let memory = Memory::new(&mut bus);
        let gpu = GPU::new(&mut bus);
        let storage = Storage::new(&mut bus);
        let io = IOSystem::new(&mut bus);

        Self {
            bus,
            cpu,
            memory,
            gpu,
            storage,
            io,
        }
    }

    pub fn tick(&mut self) {
        self.cpu.tick();
        self.memory.tick();
        self.gpu.tick();
        self.storage.tick();
        self.io.tick();
    }

    pub fn get_stats(&self) -> HardwareStats {
        HardwareStats {
            cpu_utilization: self.cpu.get_utilization(),
            memory_usage: self.memory.get_usage(),
            gpu_utilization: self.gpu.get_utilization(),
            storage_activity: self.storage.get_activity(),
            io_throughput: self.io.get_throughput(),
        }
    }
}

pub struct HardwareStats {
    cpu_utilization: f32,
    memory_usage: f32,
    gpu_utilization: f32,
    storage_activity: f32,
    io_throughput: f32,
}