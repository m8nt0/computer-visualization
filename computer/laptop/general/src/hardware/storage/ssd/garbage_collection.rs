use super::nand::{NANDFlash, Block, BlockStatus, PageState};
use super::super::error::{StorageError, StorageResult};
use std::collections::{BinaryHeap, HashMap};

pub struct GarbageCollector {
    block_info: HashMap<usize, BlockInfo>,
    candidates: BinaryHeap<GCCandidate>,
    config: GCConfig,
    stats: GCStats,
}

struct BlockInfo {
    valid_pages: u32,
    invalid_pages: u32,
    free_pages: u32,
    last_collection: u64,
}

#[derive(Clone)]
struct GCCandidate {
    block_id: usize,
    score: f32,
    valid_ratio: f32,
}

struct GCConfig {
    threshold_ratio: f32,      // When to start GC
    target_free_blocks: u32,   // How many blocks to maintain free
    max_valid_ratio: f32,      // Max valid pages ratio for collection
    collection_batch: usize,   // How many blocks to collect at once
}

struct GCStats {
    collections: u64,
    pages_moved: u64,
    blocks_reclaimed: u64,
    time_spent: u64,
}

impl GarbageCollector {
    pub fn new(config: GCConfig) -> Self {
        Self {
            block_info: HashMap::new(),
            candidates: BinaryHeap::new(),
            config,
            stats: GCStats::default(),
        }
    }

    pub fn check_space(&mut self, nand: &NANDFlash) -> StorageResult<()> {
        // Update block information
        self.update_block_info(nand);

        // Check if GC is needed
        if self.needs_collection() {
            self.perform_collection(nand)?;
        }

        Ok(())
    }

    fn update_block_info(&mut self, nand: &NANDFlash) {
        self.block_info.clear();
        self.candidates.clear();

        for (block_id, status) in nand.get_block_statuses().iter().enumerate() {
            let info = BlockInfo {
                valid_pages: status.valid_pages,
                invalid_pages: status.invalid_pages,
                free_pages: status.free_pages,
                last_collection: 0,
            };

            // Calculate collection score
            let valid_ratio = info.valid_pages as f32 / 
                (info.valid_pages + info.invalid_pages) as f32;

            if valid_ratio < self.config.max_valid_ratio {
                let score = self.calculate_gc_score(&info);
                self.candidates.push(GCCandidate {
                    block_id,
                    score,
                    valid_ratio,
                });
            }

            self.block_info.insert(block_id, info);
        }
    }

    fn needs_collection(&self) -> bool {
        let free_ratio = self.get_free_space_ratio();
        free_ratio < self.config.threshold_ratio
    }

    fn perform_collection(&mut self, nand: &mut NANDFlash) -> StorageResult<()> {
        let start_time = self.get_current_time();
        let mut pages_moved = 0;
        let mut blocks_reclaimed = 0;

        // Process most promising candidates
        for _ in 0..self.config.collection_batch {
            if let Some(candidate) = self.candidates.pop() {
                if self.collect_block(nand, candidate.block_id)? {
                    blocks_reclaimed += 1;
                    pages_moved += candidate.valid_ratio * nand.get_pages_per_block() as f32;
                }
            } else {
                break;
            }
        }

        // Update statistics
        self.stats.collections += 1;
        self.stats.pages_moved += pages_moved as u64;
        self.stats.blocks_reclaimed += blocks_reclaimed;
        self.stats.time_spent += self.get_current_time() - start_time;

        Ok(())
    }

    fn collect_block(&mut self, nand: &mut NANDFlash, block_id: usize) -> StorageResult<bool> {
        // Read valid pages
        let valid_data = self.read_valid_pages(nand, block_id)?;

        // Find target block for valid data
        let target_block = self.find_free_block(nand)?;

        // Write valid data to new location
        for (page_offset, data) in valid_data {
            nand.write_page(target_block, page_offset, &data)?;
        }

        // Erase the collected block
        nand.erase_block(block_id)?;

        Ok(true)
    }

    fn calculate_gc_score(&self, info: &BlockInfo) -> f32 {
        let valid_ratio = info.valid_pages as f32 / 
            (info.valid_pages + info.invalid_pages) as f32;
        
        // Score based on invalid pages and age
        let age_factor = 1.0 - (self.get_current_time() - info.last_collection) as f32 / 1000.0;
        
        (1.0 - valid_ratio) * age_factor
    }

    fn get_free_space_ratio(&self) -> f32 {
        let total_free_pages: u32 = self.block_info.values()
            .map(|info| info.free_pages)
            .sum();

        let total_pages: u32 = self.block_info.len() as u32 * 
            self.config.target_free_blocks;

        total_free_pages as f32 / total_pages as f32
    }

    fn read_valid_pages(&self, nand: &NANDFlash, block_id: usize) 
        -> StorageResult<Vec<(usize, Vec<u8>)>> 
    {
        let mut valid_data = Vec::new();
        let pages_per_block = nand.get_pages_per_block();

        for page_id in 0..pages_per_block {
            if nand.is_page_valid(block_id, page_id)? {
                let data = nand.read_page(block_id, page_id)?;
                valid_data.push((page_id, data));
            }
        }

        Ok(valid_data)
    }

    fn find_free_block(&self, nand: &NANDFlash) -> StorageResult<usize> {
        // Find a completely free block
        for (block_id, info) in &self.block_info {
            if info.free_pages as usize == nand.get_pages_per_block() {
                return Ok(*block_id);
            }
        }
        Err(StorageError::NoFreeBlocks)
    }

    fn get_current_time(&self) -> u64 {
        // Implementation would use actual system time
        0
    }
}

impl PartialOrd for GCCandidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl Ord for GCCandidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialEq for GCCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for GCCandidate {}
