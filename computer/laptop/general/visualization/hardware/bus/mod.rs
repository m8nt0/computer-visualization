use super::common::{Point, Size, Color, Rect};
use crate::hardware::bus::{
    SystemBus,
    MemoryBus,
    PciBus,
    arbitration::BusArbiter
};

pub struct BusVisualizer {
    // Bus components
    system_bus: SystemBusView,
    memory_bus: MemoryBusView,
    pci_bus: PciBusView,
    
    // Layout
    position: Point,
    size: Size,
    
    // Animation state
    data_flows: Vec<DataFlow>,
    active_transfers: Vec<TransferAnimation>,
}

impl BusVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        Self {
            system_bus: SystemBusView::new(),
            memory_bus: MemoryBusView::new(),
            pci_bus: PciBusView::new(),
            position,
            size,
            data_flows: Vec::new(),
            active_transfers: Vec::new(),
        }
    }

    pub fn update(&mut self, bus: &SystemBus) {
        // Update bus states
        self.system_bus.update(&bus);
        self.memory_bus.update(&bus.memory_bus);
        self.pci_bus.update(&bus.pci_bus);

        // Update data flows
        self.update_data_flows(bus);
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw bus interconnects
        self.draw_bus_topology(frame);
        
        // Render individual buses
        self.system_bus.render(frame);
        self.memory_bus.render(frame);
        self.pci_bus.render(frame);
        
        // Render active transfers
        for transfer in &self.active_transfers {
            transfer.render(frame);
        }
        
        // Draw metrics
        self.draw_bus_metrics(frame);
    }

    fn update_data_flows(&mut self, bus: &SystemBus) {
        // Clear completed transfers
        self.active_transfers.retain(|t| !t.is_complete());
        
        // Add new transfers
        for transfer in bus.active_transfers() {
            let animation = TransferAnimation::new(
                transfer.source,
                transfer.destination,
                transfer.size,
            );
            self.active_transfers.push(animation);
        }
    }
} 