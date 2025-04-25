#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    // Basic stats
    pub total_accesses: u64,
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub write_backs: u64,

    // Prefetch stats
    pub prefetches: u64,
    pub prefetch_hits: u64,
    pub prefetch_misses: u64,

    // Coherency stats
    pub invalidations: u64,
    pub interventions: u64,
    pub coherency_misses: u64,

    // Timing stats
    pub total_latency: u64,
    pub stall_cycles: u64,

    // Power stats
    pub active_cycles: u64,
    pub idle_cycles: u64,
}

impl CacheStats {
    pub fn hit_rate(&self) -> f32 {
        if self.total_accesses == 0 {
            return 0.0;
        }
        self.hits as f32 / self.total_accesses as f32
    }

    pub fn miss_rate(&self) -> f32 {
        if self.total_accesses == 0 {
            return 0.0;
        }
        self.misses as f32 / self.total_accesses as f32
    }

    pub fn prefetch_accuracy(&self) -> f32 {
        if self.prefetches == 0 {
            return 0.0;
        }
        self.prefetch_hits as f32 / self.prefetches as f32
    }

    pub fn average_latency(&self) -> f32 {
        if self.total_accesses == 0 {
            return 0.0;
        }
        self.total_latency as f32 / self.total_accesses as f32
    }

    pub fn power_efficiency(&self) -> f32 {
        let total_cycles = self.active_cycles + self.idle_cycles;
        if total_cycles == 0 {
            return 0.0;
        }
        self.idle_cycles as f32 / total_cycles as f32
    }

    pub fn merge(&mut self, other: &CacheStats) {
        self.total_accesses += other.total_accesses;
        self.hits += other.hits;
        self.misses += other.misses;
        self.evictions += other.evictions;
        self.write_backs += other.write_backs;
        self.prefetches += other.prefetches;
        self.prefetch_hits += other.prefetch_hits;
        self.prefetch_misses += other.prefetch_misses;
        self.invalidations += other.invalidations;
        self.interventions += other.interventions;
        self.coherency_misses += other.coherency_misses;
        self.total_latency += other.total_latency;
        self.stall_cycles += other.stall_cycles;
        self.active_cycles += other.active_cycles;
        self.idle_cycles += other.idle_cycles;
    }
}
