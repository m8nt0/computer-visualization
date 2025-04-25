use super::common::{Point, Size, Color, Rect};
use crate::hardware::io::{
    controllers::{NetworkController, SataController, UsbController},
    devices::{DisplayDevice, InputDevice, StorageDevice}
};

pub mod controllers;
pub mod devices;

pub struct IoVisualizer {
    position: Point,
    size: Size,
    
    // Controllers
    network: controllers::NetworkVisualizer,
    sata: controllers::SataVisualizer,
    usb: controllers::UsbVisualizer,
    
    // Devices
    display: devices::DisplayVisualizer,
    input: devices::InputVisualizer,
    storage: devices::StorageVisualizer,
    
    // Data flows
    io_transfers: Vec<IoTransferAnimation>,
}

impl IoVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        let layout = IoLayout::new(position, size);
        
        Self {
            position,
            size,
            network: controllers::NetworkVisualizer::new(layout.network_region),
            sata: controllers::SataVisualizer::new(layout.sata_region),
            usb: controllers::UsbVisualizer::new(layout.usb_region),
            display: devices::DisplayVisualizer::new(layout.display_region),
            input: devices::InputVisualizer::new(layout.input_region),
            storage: devices::StorageVisualizer::new(layout.storage_region),
            io_transfers: Vec::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw I/O subsystem layout
        self.draw_io_layout(frame);
        
        // Render controllers
        self.network.render(frame);
        self.sata.render(frame);
        self.usb.render(frame);
        
        // Render devices
        self.display.render(frame);
        self.input.render(frame);
        self.storage.render(frame);
        
        // Render data transfers
        for transfer in &self.io_transfers {
            transfer.render(frame);
        }
        
        // Draw performance metrics
        self.draw_io_metrics(frame);
    }
} 