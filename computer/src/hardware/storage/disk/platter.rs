use super::super::error::{StorageError, StorageResult};
use std::collections::HashMap;

pub struct Platter {
    surfaces: Vec<Surface>,
    config: PlatterConfig,
    stats: PlatterStats,
}

struct Surface {
    tracks: Vec<Track>,
    defect_map: HashMap<(u32, u32), bool>, // (track, sector) -> is_defective
}

struct Track {
    sectors: Vec<Sector>,
    track_id: u32,
}

struct Sector {
    data: Vec<u8>,
    sector_id: u32,
    error_correction: ECC,
    flags: SectorFlags,
}

struct ECC {
    code: Vec<u8>,
    syndrome: Option<u32>,
}

bitflags! {
    struct SectorFlags: u8 {
        const VALID = 0x01;
        const BAD = 0x02;
        const REMAPPED = 0x04;
        const PENDING_REMAP = 0x08;
    }
}

struct PlatterConfig {
    surfaces: u32,
    tracks_per_surface: u32,
    sectors_per_track: u32,
    bytes_per_sector: u32,
}

struct PlatterStats {
    read_errors: u64,
    write_errors: u64,
    corrected_errors: u64,
    remapped_sectors: u64,
}

impl Platter {
    pub fn new(config: PlatterConfig) -> Self {
        let surfaces = (0..config.surfaces)
            .map(|_| Surface::new(config.tracks_per_surface, config.sectors_per_track, config.bytes_per_sector))
            .collect();

        Self {
            surfaces,
            config,
            stats: PlatterStats::default(),
        }
    }

    pub fn read_sector(&mut self, surface: u32, track: u32, sector: u32) -> StorageResult<Vec<u8>> {
        let surface = self.get_surface(surface)?;
        let track = surface.get_track(track)?;
        let sector = track.get_sector(sector)?;

        if sector.flags.contains(SectorFlags::BAD) {
            self.stats.read_errors += 1;
            return Err(StorageError::BadSector);
        }

        // Check ECC
        if let Some(error) = sector.check_ecc() {
            self.stats.read_errors += 1;
            if sector.error_correction.correct_error(error) {
                self.stats.corrected_errors += 1;
            } else {
                return Err(StorageError::UncorrectableError);
            }
        }

        Ok(sector.data.clone())
    }

    pub fn write_sector(&mut self, surface: u32, track: u32, sector: u32, data: &[u8]) -> StorageResult<()> {
        let surface = self.get_surface_mut(surface)?;
        let track = surface.get_track_mut(track)?;
        let sector = track.get_sector_mut(sector)?;

        if sector.flags.contains(SectorFlags::BAD) {
            self.stats.write_errors += 1;
            return Err(StorageError::BadSector);
        }

        sector.data.copy_from_slice(data);
        sector.error_correction.update(&sector.data);
        Ok(())
    }

    pub fn mark_bad_sector(&mut self, surface: u32, track: u32, sector: u32) -> StorageResult<()> {
        let surface = self.get_surface_mut(surface)?;
        let track = surface.get_track_mut(track)?;
        let sector = track.get_sector_mut(sector)?;

        sector.flags.insert(SectorFlags::BAD);
        self.stats.remapped_sectors += 1;
        Ok(())
    }

    fn get_surface(&self, index: u32) -> StorageResult<&Surface> {
        self.surfaces.get(index as usize)
            .ok_or(StorageError::InvalidSurface)
    }

    fn get_surface_mut(&mut self, index: u32) -> StorageResult<&mut Surface> {
        self.surfaces.get_mut(index as usize)
            .ok_or(StorageError::InvalidSurface)
    }
}
