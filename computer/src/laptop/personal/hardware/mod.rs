pub mod bus;
pub mod cpu;
pub mod memory;
pub mod gpu;
pub mod storage;
pub mod io;
pub mod error;

use self::bus::Bus;
use self::cpu::CPU;
use self::memory::Memory;
use self::gpu::GPU;
use self::storage::Storage;
use self::io::IOSystem;

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