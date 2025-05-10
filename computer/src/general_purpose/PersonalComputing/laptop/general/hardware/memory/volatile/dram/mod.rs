// Export all modules in dram
pub mod bank;
pub mod ecc;
pub mod rank;
pub mod refresh;
pub mod temperature;
pub mod timing;
pub mod voltage;
pub mod power;
pub mod controller;


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

struct VoltageController {
    vDD: f32,    // Voltage (V)
    vDDQ: f32,   // Voltage (V)
    vPP: f32,    // Voltage (V)
    vPPQ: f32,   // Voltage (V)
}

struct ECC {
    enabled: bool,
    error_correction_type: ErrorCorrectionType,
}

struct SPDInfo {
    manufacturer: String,
    part_number: String,
    serial_number: String,
    voltage_range: String,
    speed_grade: String,
    module_type: String,
}

struct MemoryChip {
    id: u32,
    status: ChipStatus,
    voltage: f32,
}

enum ChipStatus {
    Ok,
    Failed,
    Unknown,
}
