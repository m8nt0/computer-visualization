use super::common::{Point, Size, Color, Rect};
use crate::hardware::storage::{
    disk::{DiskController, Platter, Head},
    ssd::{SsdController, NandFlash, WearLeveling},
    nvme::{NvmeController, Protocol, Queue},
    filesystem::{Ext4, Fat, Ntfs}
};

pub mod disk;
pub mod ssd;
pub mod nvme;
pub mod filesystem;

pub struct StorageVisualizer {
    position: Point,
    size: Size,
    
    // Storage devices
    disk: disk::DiskVisualizer,
    ssd: ssd::SsdVisualizer,
    nvme: nvme::NvmeVisualizer,
    
    // Filesystem
    filesystem: filesystem::FilesystemVisualizer,
    
    // Data flows
    data_transfers: Vec<StorageTransferAnimation>,
}

impl StorageVisualizer {
    pub fn new(position: Point, size: Size) -> Self {
        let layout = StorageLayout::new(position, size);
        
        Self {
            position,
            size,
            disk: disk::DiskVisualizer::new(layout.disk_region),
            ssd: ssd::SsdVisualizer::new(layout.ssd_region),
            nvme: nvme::NvmeVisualizer::new(layout.nvme_region),
            filesystem: filesystem::FilesystemVisualizer::new(layout.fs_region),
            data_transfers: Vec::new(),
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        // Draw storage subsystem layout
        self.draw_storage_layout(frame);
        
        // Render storage devices
        self.disk.render(frame);
        self.ssd.render(frame);
        self.nvme.render(frame);
        
        // Render filesystem
        self.filesystem.render(frame);
        
        // Render data transfers
        for transfer in &self.data_transfers {
            transfer.render(frame);
        }
        
        // Draw performance metrics
        self.draw_storage_metrics(frame);
    }
} 