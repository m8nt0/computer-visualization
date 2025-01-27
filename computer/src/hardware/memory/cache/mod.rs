pub mod l1_cache;
pub mod l2_cache;
pub mod l3_cache;
pub mod coherency;
pub mod replacement;
pub mod prefetch;
pub mod cache;
pub mod stats;

pub use l1_cache::{L1ICache, L1DCache};
pub use l2_cache::L2Cache;
pub use l3_cache::L3Cache;

pub struct CacheHierarchy {
    l1i: L1ICache,
    l1d: L1DCache,
    l2: L2Cache,
    l3: L3Cache,
    stats: CacheStats,
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    hits: u64,
    misses: u64,
    evictions: u64,
    prefetches: u64,
    write_backs: u64,
} 