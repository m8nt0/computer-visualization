use super::error::{DriverError, DriverResult};
use std::collections::HashMap;

pub struct DriverManager {
    drivers: HashMap<DriverId, Box<dyn Driver>>,
    device_map: HashMap<DeviceId, DriverId>,
    resources: ResourceManager,
    config: DriverConfig,
}

pub trait Driver: Send + Sync {
    fn probe(&self, device: &DeviceInfo) -> bool;
    fn attach(&mut self, device: &DeviceInfo) -> DriverResult<()>;
    fn detach(&mut self) -> DriverResult<()>;
    fn handle_interrupt(&mut self) -> DriverResult<()>;
    fn read(&mut self, offset: u64, buffer: &mut [u8]) -> DriverResult<usize>;
    fn write(&mut self, offset: u64, buffer: &[u8]) -> DriverResult<usize>;
    fn ioctl(&mut self, cmd: u32, arg: usize) -> DriverResult<usize>;
}

struct ResourceManager {
    io_ports: IoPortManager,
    memory_regions: MemoryRegionManager,
    interrupts: InterruptManager,
    dma_channels: DmaManager,
}

impl DriverManager {
    pub fn register_driver(&mut self, driver: Box<dyn Driver>) -> DriverResult<DriverId> {
        let id = self.generate_driver_id();
        self.drivers.insert(id, driver);
        Ok(id)
    }

    pub fn probe_device(&mut self, device: &DeviceInfo) -> DriverResult<Option<DriverId>> {
        for (id, driver) in &self.drivers {
            if driver.probe(device) {
                return Ok(Some(*id));
            }
        }
        Ok(None)
    }

    pub fn attach_device(&mut self, device_id: DeviceId, driver_id: DriverId) -> DriverResult<()> {
        let driver = self.drivers.get_mut(&driver_id)
            .ok_or(DriverError::InvalidDriver)?;

        let device = self.get_device_info(device_id)?;
        driver.attach(&device)?;
        
        self.device_map.insert(device_id, driver_id);
        Ok(())
    }
} 