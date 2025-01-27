// src/hardware/memory/dram.rs
pub struct DRAM {
    ranks: Vec<Rank>,
    timing_controller: TimingController,
    temperature_sensor: TempSensor,
    voltage_controller: VoltageController,
    error_correction: ECC,
    spd: SPDInfo,  // Serial Presence Detect
}

struct Rank {
    chips: Vec<MemoryChip>,
    bank_groups: Vec<BankGroup>,
    status: RankStatus,
}

struct BankGroup {
    banks: Vec<Bank>,
    internal_bus: InternalBus,
}

struct TimingController {
    tRCD: u8,    // RAS to CAS Delay
    tRP: u8,     // Row Precharge Time
    tCAS: u8,    // Column Access Strobe Latency
    tRAS: u8,    // Row Access Strobe
    tRC: u8,     // Row Cycle Time
    tWR: u8,     // Write Recovery Time
    refresh_interval: u32,
}