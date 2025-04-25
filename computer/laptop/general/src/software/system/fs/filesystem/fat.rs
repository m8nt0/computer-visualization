use super::super::error::{StorageError, StorageResult};
use std::collections::HashMap;

pub struct FatFileSystem {
    boot_sector: BootSector,
    fat_table: Vec<FatEntry>,
    root_directory: Directory,
    current_directory: Directory,
    cache: FatCache,
    stats: FatStats,
}

struct BootSector {
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    number_of_fats: u8,
    root_entries: u16,
    total_sectors: u32,
    media_descriptor: u8,
    sectors_per_fat: u16,
}

#[derive(Clone, Copy)]
enum FatEntry {
    Free,
    Reserved,
    Bad,
    EndOfChain,
    Next(u32),
}

struct Directory {
    entries: Vec<DirectoryEntry>,
    first_cluster: u32,
    parent_cluster: Option<u32>,
}

struct DirectoryEntry {
    name: [u8; 8],
    extension: [u8; 3],
    attributes: FileAttributes,
    create_time: u16,
    create_date: u16,
    last_access_date: u16,
    first_cluster_high: u16,
    write_time: u16,
    write_date: u16,
    first_cluster_low: u16,
    file_size: u32,
}

bitflags! {
    struct FileAttributes: u8 {
        const READ_ONLY = 0x01;
        const HIDDEN = 0x02;
        const SYSTEM = 0x04;
        const VOLUME_ID = 0x08;
        const DIRECTORY = 0x10;
        const ARCHIVE = 0x20;
    }
}

impl FatFileSystem {
    pub fn new(device: &mut DiskController) -> StorageResult<Self> {
        // Read boot sector
        let boot_sector = Self::read_boot_sector(device)?;
        
        // Initialize FAT
        let fat_size = boot_sector.sectors_per_fat as usize * boot_sector.bytes_per_sector as usize;
        let mut fat_table = vec![FatEntry::Free; fat_size / 4];
        
        // Read FAT table
        Self::read_fat_table(device, &mut fat_table)?;
        
        // Read root directory
        let root_directory = Self::read_root_directory(device, &boot_sector)?;
        
        Ok(Self {
            boot_sector,
            fat_table,
            root_directory: root_directory.clone(),
            current_directory: root_directory,
            cache: FatCache::new(),
            stats: FatStats::default(),
        })
    }

    pub fn create_file(&mut self, name: &str, attributes: FileAttributes) -> StorageResult<()> {
        // Find free directory entry
        let entry_index = self.find_free_directory_entry()?;
        
        // Allocate first cluster
        let first_cluster = self.allocate_cluster()?;
        
        // Create directory entry
        let entry = DirectoryEntry {
            name: Self::format_filename(name)?,
            extension: Self::format_extension(name)?,
            attributes,
            create_time: self.get_current_time(),
            create_date: self.get_current_date(),
            last_access_date: self.get_current_date(),
            first_cluster_high: (first_cluster >> 16) as u16,
            write_time: self.get_current_time(),
            write_date: self.get_current_date(),
            first_cluster_low: (first_cluster & 0xFFFF) as u16,
            file_size: 0,
        };
        
        // Write directory entry
        self.write_directory_entry(entry_index, entry)?;
        
        Ok(())
    }

    pub fn write(&mut self, path: &str, data: &[u8]) -> StorageResult<usize> {
        // Find file entry
        let mut entry = self.find_file(path)?;
        
        // Allocate clusters if needed
        let clusters_needed = (data.len() + self.cluster_size() - 1) / self.cluster_size();
        let mut current_cluster = self.get_first_cluster(&entry);
        
        for _ in 1..clusters_needed {
            let next_cluster = self.allocate_cluster()?;
            self.set_fat_entry(current_cluster, FatEntry::Next(next_cluster));
            current_cluster = next_cluster;
        }
        self.set_fat_entry(current_cluster, FatEntry::EndOfChain);
        
        // Write data
        self.write_clusters(self.get_first_cluster(&entry), data)?;
        
        // Update directory entry
        entry.file_size = data.len() as u32;
        entry.write_time = self.get_current_time();
        entry.write_date = self.get_current_date();
        self.update_directory_entry(&entry)?;
        
        Ok(data.len())
    }

    // Helper methods...
}
