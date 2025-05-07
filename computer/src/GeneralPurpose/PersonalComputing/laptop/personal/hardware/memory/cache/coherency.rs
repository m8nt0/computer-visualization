#[derive(Clone, Copy, PartialEq)]
pub enum CoherencyState {
    Modified,  // Cache line is modified and exclusive
    Exclusive, // Cache line is unmodified and exclusive
    Shared,    // Cache line is shared with other caches
    Invalid,   // Cache line is invalid
}

pub struct CoherencyController {
    bus_snooping: bool,
    directory: CoherencyDirectory,
    pending_requests: Vec<CoherencyRequest>,
    stats: CoherencyStats,
}

struct CoherencyDirectory {
    entries: Vec<DirectoryEntry>,
}

struct DirectoryEntry {
    address: u64,
    sharers: Vec<CacheId>,
    owner: Option<CacheId>,
    state: CoherencyState,
}

#[derive(Clone, Copy, PartialEq)]
pub struct CacheId {
    level: CacheLevel,
    core_id: u8,
}

#[derive(Clone, Copy, PartialEq)]
enum CacheLevel {
    L1I,
    L1D,
    L2,
    L3,
}

#[derive(Clone)]
struct CoherencyRequest {
    address: u64,
    operation: CoherencyOp,
    requester: CacheId,
    state: RequestState,
}

enum CoherencyOp {
    GetShared,      // Read request
    GetModified,    // Write request
    Upgrade,        // Shared to Modified
    Writeback,      // Eviction of Modified line
    Invalidate,     // Force invalidation
}

enum RequestState {
    Pending,
    WaitingForAcks,
    Complete,
}

impl CoherencyController {
    pub fn new(bus_snooping: bool) -> Self {
        Self {
            bus_snooping,
            directory: CoherencyDirectory::new(),
            pending_requests: Vec::new(),
            stats: CoherencyStats::default(),
        }
    }

    pub fn handle_request(&mut self, address: u64, operation: CoherencyOp, 
                         requester: CacheId) -> CoherencyResult {
        self.stats.total_requests += 1;

        // Check directory state
        let entry = self.directory.lookup(address);
        
        match (operation, entry.state) {
            (CoherencyOp::GetShared, CoherencyState::Modified) => {
                // Need to get data from current owner
                self.request_writeback(entry.owner.unwrap(), address);
                self.add_sharer(address, requester);
                CoherencyResult::WaitForData
            },
            
            (CoherencyOp::GetModified, _) => {
                // Invalidate all other copies
                self.invalidate_sharers(address);
                self.set_owner(address, requester);
                CoherencyResult::Proceed
            },

            (CoherencyOp::Upgrade, CoherencyState::Shared) => {
                // Invalidate other sharers
                self.invalidate_other_sharers(address, requester);
                self.set_owner(address, requester);
                CoherencyResult::Proceed
            },

            _ => CoherencyResult::Proceed
        }
    }

    fn invalidate_sharers(&mut self, address: u64) {
        let entry = self.directory.lookup_mut(address);
        for sharer in &entry.sharers {
            self.send_invalidate(*sharer, address);
        }
        entry.sharers.clear();
        entry.state = CoherencyState::Invalid;
    }

    fn add_sharer(&mut self, address: u64, cache_id: CacheId) {
        let entry = self.directory.lookup_mut(address);
        entry.sharers.push(cache_id);
        entry.state = CoherencyState::Shared;
    }

    fn set_owner(&mut self, address: u64, cache_id: CacheId) {
        let entry = self.directory.lookup_mut(address);
        entry.owner = Some(cache_id);
        entry.state = CoherencyState::Modified;
    }

    // Methods for bus snooping
    fn handle_snoop(&mut self, address: u64, operation: CoherencyOp) {
        if self.bus_snooping {
            match operation {
                CoherencyOp::GetShared => self.handle_read_snoop(address),
                CoherencyOp::GetModified => self.handle_write_snoop(address),
                _ => {}
            }
        }
    }
}

pub enum CoherencyResult {
    Proceed,
    WaitForData,
    Retry,
}

#[derive(Default)]
struct CoherencyStats {
    total_requests: u64,
    invalidations: u64,
    writebacks: u64,
    upgrades: u64,
}
