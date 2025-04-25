use super::queue::{Command, CommandType, Priority};
use super::super::types::PhysicalAddress;
use super::super::error::MemoryResult;

pub struct CommandScheduler {
    current_cycle: u64,
    bank_states: Vec<BankState>,
    scheduling_policy: SchedulingPolicy,
    stats: SchedulerStats,
}

struct BankState {
    active: bool,
    active_row: Option<u32>,
    last_activate: u64,
    last_precharge: u64,
}

#[derive(Clone, Copy)]
enum SchedulingPolicy {
    FirstComeFirstServed,
    BankRoundRobin,
    RowBuffer,
    PowerAware,
}

struct SchedulerStats {
    row_hits: u64,
    row_misses: u64,
    bank_conflicts: u64,
    total_commands: u64,
}

impl CommandScheduler {
    pub fn new() -> Self {
        Self {
            current_cycle: 0,
            bank_states: vec![BankState::new(); 8], // 8 banks
            scheduling_policy: SchedulingPolicy::RowBuffer,
            stats: SchedulerStats::default(),
        }
    }

    pub fn tick(&mut self) {
        self.current_cycle += 1;
    }

    pub fn create_read_command(&self, address: PhysicalAddress) -> Command {
        Command {
            cmd_type: CommandType::Read,
            address,
            data: None,
            priority: Priority::Normal,
            timestamp: self.current_cycle,
        }
    }

    pub fn create_write_command(&self, address: PhysicalAddress, data: u32) -> Command {
        Command {
            cmd_type: CommandType::Write,
            address,
            data: Some(data),
            priority: Priority::Normal,
            timestamp: self.current_cycle,
        }
    }

    pub fn schedule_command(&mut self, cmd: &Command) -> MemoryResult<()> {
        self.stats.total_commands += 1;
        
        let bank_id = self.get_bank_id(cmd.address);
        let bank = &mut self.bank_states[bank_id];

        match cmd.cmd_type {
            CommandType::Read | CommandType::Write => {
                let row = self.get_row(cmd.address);
                
                if !bank.active {
                    // Need to activate first
                    self.stats.row_misses += 1;
                    bank.active = true;
                    bank.active_row = Some(row);
                    bank.last_activate = self.current_cycle;
                } else if bank.active_row != Some(row) {
                    // Row conflict
                    self.stats.bank_conflicts += 1;
                    bank.active_row = Some(row);
                } else {
                    // Row buffer hit
                    self.stats.row_hits += 1;
                }
            }
            CommandType::Precharge => {
                bank.active = false;
                bank.active_row = None;
                bank.last_precharge = self.current_cycle;
            }
            _ => {}
        }

        Ok(())
    }

    fn get_bank_id(&self, address: PhysicalAddress) -> usize {
        // Extract bank ID from address
        ((address.0 >> 13) & 0x7) as usize
    }

    fn get_row(&self, address: PhysicalAddress) -> u32 {
        // Extract row ID from address
        ((address.0 >> 16) & 0xFFFF) as u32
    }
}

impl BankState {
    fn new() -> Self {
        Self {
            active: false,
            active_row: None,
            last_activate: 0,
            last_precharge: 0,
        }
    }
}
