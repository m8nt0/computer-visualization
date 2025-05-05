use std::collections::HashMap;

pub struct BranchPredictor {
    // Branch Target Buffer (BTB)
    btb: HashMap<u32, BtbEntry>,
    
    // Pattern History Table (PHT)
    pht: Vec<u8>,
    
    // Global History Register
    ghr: u32,
    
    // Statistics for visualization
    predictions: u64,
    correct_predictions: u64,
}

struct BtbEntry {
    target_address: u32,
    prediction: u8,    // 2-bit saturating counter
    last_taken: bool,
}

impl BranchPredictor {
    pub fn new(pht_size: usize) -> Self {
        Self {
            btb: HashMap::new(),
            pht: vec![2; pht_size], // Initialize with weakly taken
            ghr: 0,
            predictions: 0,
            correct_predictions: 0,
        }
    }

    pub fn predict(&mut self, pc: u32) -> Option<u32> {
        self.predictions += 1;
        
        if let Some(entry) = self.btb.get(&pc) {
            let pht_index = self.get_pht_index(pc);
            let prediction = self.pht[pht_index] >= 2;
            
            if prediction {
                Some(entry.target_address)
            } else {
                Some(pc + 4)
            }
        } else {
            None
        }
    }

    pub fn update(&mut self, pc: u32, target: u32, taken: bool) {
        let pht_index = self.get_pht_index(pc);
        
        // Update BTB
        let entry = self.btb.entry(pc).or_insert(BtbEntry {
            target_address: target,
            prediction: 2,
            last_taken: false,
        });

        // Update prediction counter
        if taken {
            if entry.prediction < 3 {
                entry.prediction += 1;
            }
            if self.pht[pht_index] < 3 {
                self.pht[pht_index] += 1;
            }
        } else {
            if entry.prediction > 0 {
                entry.prediction -= 1;
            }
            if self.pht[pht_index] > 0 {
                self.pht[pht_index] -= 1;
            }
        }

        // Update global history register
        self.ghr = ((self.ghr << 1) | (taken as u32)) & 0xFFFF;
        
        // Update statistics
        if (taken && entry.prediction >= 2) || (!taken && entry.prediction < 2) {
            self.correct_predictions += 1;
        }
        
        entry.last_taken = taken;
        entry.target_address = target;
    }

    fn get_pht_index(&self, pc: u32) -> usize {
        // XOR PC with global history for better prediction
        ((pc ^ self.ghr) as usize) % self.pht.len()
    }

    // Methods for visualization system
    pub fn get_accuracy(&self) -> f32 {
        if self.predictions == 0 {
            return 0.0;
        }
        self.correct_predictions as f32 / self.predictions as f32
    }

    pub fn get_btb_entry(&self, pc: u32) -> Option<(u32, bool)> {
        self.btb.get(&pc).map(|entry| (
            entry.target_address,
            entry.prediction >= 2
        ))
    }

    pub fn get_ghr(&self) -> u32 {
        self.ghr
    }

    pub fn get_pht_state(&self, index: usize) -> u8 {
        self.pht[index]
    }
}
