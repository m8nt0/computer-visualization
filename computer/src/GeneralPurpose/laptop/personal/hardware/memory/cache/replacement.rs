use std::collections::HashMap;

pub enum ReplacementPolicy {
    LRU(LRUPolicy),
    PLRU(PLRUPolicy),
    RRIP(RRIPPolicy),
    Random,
}

impl ReplacementPolicy {
    pub fn new() -> Self {
        ReplacementPolicy::LRU(LRUPolicy::new())
    }

    pub fn update_access(&mut self, way: usize) {
        match self {
            ReplacementPolicy::LRU(policy) => policy.update_access(way),
            ReplacementPolicy::PLRU(policy) => policy.update_access(way),
            ReplacementPolicy::RRIP(policy) => policy.update_access(way),
            ReplacementPolicy::Random => {}
        }
    }

    pub fn get_victim(&self, valid_ways: &[bool]) -> usize {
        match self {
            ReplacementPolicy::LRU(policy) => policy.get_victim(valid_ways),
            ReplacementPolicy::PLRU(policy) => policy.get_victim(valid_ways),
            ReplacementPolicy::RRIP(policy) => policy.get_victim(valid_ways),
            ReplacementPolicy::Random => {
                // Simple random selection among valid ways
                let valid_indices: Vec<usize> = valid_ways.iter()
                    .enumerate()
                    .filter(|(_, &valid)| valid)
                    .map(|(i, _)| i)
                    .collect();
                
                valid_indices[fastrand::usize(..valid_indices.len())]
            }
        }
    }
}

// Least Recently Used
struct LRUPolicy {
    access_order: Vec<usize>,
}

impl LRUPolicy {
    fn new() -> Self {
        Self {
            access_order: Vec::new(),
        }
    }

    fn update_access(&mut self, way: usize) {
        self.access_order.retain(|&w| w != way);
        self.access_order.push(way);
    }

    fn get_victim(&self, valid_ways: &[bool]) -> usize {
        // Return least recently used valid way
        self.access_order.iter()
            .rev()
            .find(|&&way| valid_ways[way])
            .copied()
            .unwrap_or(0)
    }
}

// Pseudo-LRU using a binary tree
struct PLRUPolicy {
    tree_bits: Vec<bool>, // Binary tree for tracking access pattern
}

// Re-Reference Interval Prediction
struct RRIPPolicy {
    prediction_values: Vec<u8>,
    max_value: u8,
}

impl RRIPPolicy {
    fn new(ways: usize) -> Self {
        Self {
            prediction_values: vec![2; ways], // Initialize with distant re-reference
            max_value: 3,
        }
    }

    fn update_access(&mut self, way: usize) {
        self.prediction_values[way] = 0; // Set to immediate re-reference
    }

    fn get_victim(&self, valid_ways: &[bool]) -> usize {
        // Find valid way with highest prediction value (longest re-reference)
        valid_ways.iter()
            .enumerate()
            .filter(|(_, &valid)| valid)
            .max_by_key(|(i, _)| self.prediction_values[*i])
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}
