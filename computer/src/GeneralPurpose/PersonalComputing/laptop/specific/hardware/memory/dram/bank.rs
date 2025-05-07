use super::stats::BankStats;

/// Represents a single DRAM bank with rows and columns
pub struct MemoryBank {
    id: usize,
    rows: Vec<Row>,
    active_row: Option<usize>,
    stats: BankStats,
}

struct Row {
    cells: Vec<u8>,
    last_access: u64,
    access_count: u64,
}

impl MemoryBank {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            // Modern DRAM typically has thousands of rows
            rows: (0..8192).map(|_| Row {
                cells: vec![0; 1024], // 1024 columns per row
                last_access: 0,
                access_count: 0,
            }).collect(),
            active_row: None,
            stats: BankStats::new(),
        }
    }

    /// Simulate row activation delay and power usage
    pub fn activate_row(&mut self, row: usize, current_cycle: u64) -> u64 {
        self.stats.row_activations += 1;
        
        // If we need to close another row first (row buffer miss)
        if let Some(active) = self.active_row {
            if active != row {
                self.stats.row_conflicts += 1;
                // Precharge delay + Activation delay
                self.active_row = Some(row);
                current_cycle + 24 // tRP + tRCD in memory cycles
            } else {
                // Row buffer hit
                self.stats.row_hits += 1;
                current_cycle
            }
        } else {
            // No row active, just activation delay
            self.active_row = Some(row);
            current_cycle + 14 // tRCD in memory cycles
        }
    }

    /// Read data from the bank, handling row activation
    pub fn read(&mut self, row: usize, col: usize, current_cycle: u64) -> (u8, u64) {
        let ready_cycle = self.activate_row(row, current_cycle);
        let data = self.rows[row].cells[col];
        self.rows[row].access_count += 1;
        self.rows[row].last_access = current_cycle;
        (data, ready_cycle + 4) // Add CAS latency
    }

    /// Write data to the bank, handling row activation
    pub fn write(&mut self, row: usize, col: usize, data: u8, current_cycle: u64) -> u64 {
        let ready_cycle = self.activate_row(row, current_cycle);
        self.rows[row].cells[col] = data;
        self.rows[row].access_count += 1;
        self.rows[row].last_access = current_cycle;
        ready_cycle + 4 // Add CAS latency
    }

    /// Get visualization data for this bank
    pub fn get_visualization_data(&self) -> BankVisualizationData {
        BankVisualizationData {
            id: self.id,
            active_row: self.active_row,
            row_activity: self.rows.iter()
                .map(|row| (row.last_access, row.access_count))
                .collect(),
            stats: self.stats.clone(),
        }
    }
}

/// Data structure for visualizing bank state
pub struct BankVisualizationData {
    pub id: usize,
    pub active_row: Option<usize>,
    pub row_activity: Vec<(u64, u64)>, // (last_access, access_count)
    pub stats: BankStats,
} 