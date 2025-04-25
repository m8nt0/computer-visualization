use super::rank::Rank;
use std::collections::VecDeque;

pub struct RefreshController {
    // Basic refresh parameters
    refresh_interval: u32,    // Cycles between refreshes
    rows_per_refresh: u32,    // Number of rows to refresh at once
    current_row: u32,         // Next row to refresh
    cycles_since_refresh: u32, // Cycles since last refresh
    refresh_in_progress: bool,
    
    // Advanced refresh features
    temperature_compensated: bool,
    distributed_refresh: bool,
    per_bank_refresh: bool,
    
    // Refresh queue
    pending_refreshes: VecDeque<RefreshCommand>,
    
    // Statistics
    stats: RefreshStats,
}

struct RefreshCommand {
    row: u32,
    bank: u32,
    rank: u32,
    deadline: u64,
    temperature_triggered: bool,
}

#[derive(Default)]
struct RefreshStats {
    total_refreshes: u64,
    refresh_cycles: u64,
    delayed_refreshes: u64,
    temperature_triggered_refreshes: u64,
    refresh_energy: f32,
}

impl RefreshController {
    pub fn new(ranks: usize, banks_per_rank: usize) -> Self {
        Self {
            refresh_interval: 7800,     // 64ms/8192 rows ≈ 7.8μs at 1ns cycle time
            rows_per_refresh: 1,        // Refresh one row at a time
            current_row: 0,
            cycles_since_refresh: 0,
            refresh_in_progress: false,
            
            temperature_compensated: true,
            distributed_refresh: true,
            per_bank_refresh: true,
            
            pending_refreshes: VecDeque::new(),
            stats: RefreshStats::default(),
        }
    }

    pub fn tick(&mut self, ranks: &mut [Rank], current_cycle: u64) {
        self.cycles_since_refresh += 1;
        
        // Check if we need to issue new refreshes
        if self.cycles_since_refresh >= self.get_adjusted_interval() {
            self.schedule_refreshes(current_cycle);
        }
        
        // Process pending refreshes
        self.process_refreshes(ranks, current_cycle);
        
        // Update statistics
        if self.refresh_in_progress {
            self.stats.refresh_cycles += 1;
        }
    }

    fn get_adjusted_interval(&self) -> u32 {
        if !self.temperature_compensated {
            return self.refresh_interval;
        }
        
        // Adjust refresh interval based on temperature
        // Higher temperature = more frequent refreshes
        self.refresh_interval
    }

    fn schedule_refreshes(&mut self, current_cycle: u64) {
        let deadline = current_cycle + self.refresh_interval as u64;
        
        if self.distributed_refresh {
            // Schedule multiple smaller refreshes
            for i in 0..8 {
                self.pending_refreshes.push_back(RefreshCommand {
                    row: self.current_row + i,
                    bank: (self.current_row % 8) as u32,
                    rank: 0,
                    deadline,
                    temperature_triggered: false,
                });
            }
        } else {
            // Schedule one large refresh
            self.pending_refreshes.push_back(RefreshCommand {
                row: self.current_row,
                bank: 0,
                rank: 0,
                deadline,
                temperature_triggered: false,
            });
        }
        
        self.cycles_since_refresh = 0;
    }

    fn process_refreshes(&mut self, ranks: &mut [Rank], current_cycle: u64) {
        while let Some(cmd) = self.pending_refreshes.front() {
            if current_cycle > cmd.deadline {
                self.stats.delayed_refreshes += 1;
            }
            
            // Try to issue refresh
            if let Some(rank) = ranks.get_mut(cmd.rank as usize) {
                if self.per_bank_refresh {
                    // Refresh specific bank
                    if let Ok(_) = rank.refresh_bank(cmd.bank as usize, cmd.row) {
                        self.pending_refreshes.pop_front();
                        self.stats.total_refreshes += 1;
                    }
                } else {
                    // Refresh entire rank
                    if let Ok(_) = rank.refresh_all(cmd.row) {
                        self.pending_refreshes.pop_front();
                        self.stats.total_refreshes += 1;
                    }
                }
            }
        }
    }

    pub fn handle_temperature_alert(&mut self, rank: usize, bank: usize, current_cycle: u64) {
        // Schedule emergency refresh due to high temperature
        self.pending_refreshes.push_front(RefreshCommand {
            row: self.current_row,
            bank: bank as u32,
            rank: rank as u32,
            deadline: current_cycle,
            temperature_triggered: true,
        });
        self.stats.temperature_triggered_refreshes += 1;
    }
}
