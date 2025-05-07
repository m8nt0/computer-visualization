use super::cache::CacheStats;
use super::dram::DRAMStats;
use super::mmu::MMUStats;
use super::error::ErrorStats;

pub struct MemoryMetrics {
    pub cache_metrics: CacheMetrics,
    pub dram_metrics: DRAMMetrics,
    pub mmu_metrics: MMUMetrics,
    pub error_metrics: ErrorMetrics,
    pub power_metrics: PowerMetrics,
}

#[derive(Clone, Debug)]
pub struct CacheMetrics {
    pub hit_rates: [f32; 4],
    pub miss_rates: [f32; 4],
    pub bandwidth: f32,
    pub latency: f32,
    pub power_consumption: f32,
    pub temperature: f32,
}

#[derive(Clone, Debug)]
pub struct DRAMMetrics {
    pub bandwidth_usage: f32,
    pub row_buffer_hits: f32,
    pub refresh_overhead: f32,
    pub power_consumption: f32,
    pub temperature: f32,
    pub active_banks: u32,
}

#[derive(Clone, Debug)]
pub struct MMUMetrics {
    pub tlb_hit_rate: f32,
    pub page_fault_rate: f32,
    pub translation_latency: f32,
    pub memory_usage: f32,
}

#[derive(Clone, Debug)]
pub struct PowerMetrics {
    pub total_power: f32,
    pub dynamic_power: f32,
    pub static_power: f32,
    pub power_states: Vec<PowerStateMetric>,
}

#[derive(Clone, Debug)]
pub struct PowerStateMetric {
    pub state: PowerState,
    pub time_in_state: u64,
    pub transitions: u64,
}

impl MemoryMetrics {
    pub fn new() -> Self {
        Self {
            cache_metrics: CacheMetrics::default(),
            dram_metrics: DRAMMetrics::default(),
            mmu_metrics: MMUMetrics::default(),
            error_metrics: ErrorMetrics::default(),
            power_metrics: PowerMetrics::default(),
        }
    }

    pub fn update(&mut self, memory: &Memory) {
        self.cache_metrics.update(&memory.cache);
        self.dram_metrics.update(&memory.dram);
        self.mmu_metrics.update(&memory.mmu);
        self.error_metrics.update(&memory.error_handler);
        self.power_metrics.update(memory);
    }
} 