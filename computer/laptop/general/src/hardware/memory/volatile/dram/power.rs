use super::bank::MemoryBank;
use super::rank::Rank;

pub struct PowerController {
    // Power states
    current_state: PowerState,
    power_down_threshold: u32,
    idle_cycles: u32,
    
    // Power consumption tracking
    active_power: f32,      // W
    idle_power: f32,        // W
    power_down_power: f32,  // W
    current_power: f32,     // W
    
    // Temperature impact
    power_to_temp_factor: f32,
    
    // Statistics
    stats: PowerStats,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerState {
    Active,
    Idle,
    PowerDown,
    SelfRefresh,
}

struct PowerStats {
    cycles_active: u64,
    cycles_idle: u64,
    cycles_power_down: u64,
    cycles_self_refresh: u64,
    total_energy: f32,  // Joules
}

impl PowerController {
    pub fn new() -> Self {
        Self {
            current_state: PowerState::Active,
            power_down_threshold: 1000,  // Cycles before power down
            idle_cycles: 0,
            
            // Power values based on DDR4 specifications
            active_power: 1.5,      // 1.5W active
            idle_power: 0.5,        // 0.5W idle
            power_down_power: 0.1,  // 0.1W power down
            current_power: 1.5,     // Start in active state
            
            power_to_temp_factor: 0.1,  // Power to temperature conversion factor
            
            stats: PowerStats::default(),
        }
    }

    pub fn update_state(&mut self, ranks: &[Rank]) {
        // Check activity across all ranks
        let any_active = ranks.iter().any(|rank| rank.is_active());
        
        match self.current_state {
            PowerState::Active => {
                if !any_active {
                    self.idle_cycles += 1;
                    if self.idle_cycles >= self.power_down_threshold {
                        self.transition_to(PowerState::PowerDown);
                    } else if self.idle_cycles >= 100 {
                        self.transition_to(PowerState::Idle);
                    }
                } else {
                    self.idle_cycles = 0;
                }
                self.stats.cycles_active += 1;
            },
            
            PowerState::Idle => {
                if any_active {
                    self.transition_to(PowerState::Active);
                } else if self.idle_cycles >= self.power_down_threshold {
                    self.transition_to(PowerState::PowerDown);
                }
                self.stats.cycles_idle += 1;
            },
            
            PowerState::PowerDown => {
                if any_active {
                    self.transition_to(PowerState::Active);
                }
                self.stats.cycles_power_down += 1;
            },
            
            PowerState::SelfRefresh => {
                self.stats.cycles_self_refresh += 1;
            }
        }
        
        // Update energy consumption
        self.stats.total_energy += self.current_power / 1e9; // Convert to joules (assuming 1ns cycles)
    }

    fn transition_to(&mut self, new_state: PowerState) {
        self.current_state = new_state;
        self.current_power = match new_state {
            PowerState::Active => self.active_power,
            PowerState::Idle => self.idle_power,
            PowerState::PowerDown => self.power_down_power,
            PowerState::SelfRefresh => self.power_down_power * 0.8,
        };
        self.idle_cycles = 0;
    }

    pub fn get_power_state(&self) -> PowerState {
        self.current_state
    }

    pub fn get_current_power(&self) -> f32 {
        self.current_power
    }

    pub fn get_temperature_impact(&self) -> f32 {
        self.current_power * self.power_to_temp_factor
    }
} 