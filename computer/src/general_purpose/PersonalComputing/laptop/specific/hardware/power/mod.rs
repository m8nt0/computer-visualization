use std::collections::HashMap;

pub struct PowerManager {
    domains: HashMap<PowerDomain, PowerState>,
    config: PowerConfig,
    stats: PowerStats,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
enum PowerDomain {
    CPU,
    GPU,
    Memory,
    IO,
    Storage,
}

#[derive(Clone, Copy)]
enum PowerState {
    Full,
    Low,
    Sleep,
    Off,
}

struct PowerConfig {
    voltage_levels: HashMap<PowerDomain, f32>,
    frequency_limits: HashMap<PowerDomain, u32>,
    thermal_limits: HashMap<PowerDomain, f32>,
}

impl PowerManager {
    pub fn new(config: PowerConfig) -> Self {
        let mut domains = HashMap::new();
        for domain in [PowerDomain::CPU, PowerDomain::GPU, PowerDomain::Memory, 
                      PowerDomain::IO, PowerDomain::Storage].iter() {
            domains.insert(*domain, PowerState::Full);
        }

        Self {
            domains,
            config,
            stats: PowerStats::default(),
        }
    }

    pub fn set_power_state(&mut self, domain: PowerDomain, state: PowerState) {
        self.domains.insert(domain, state);
        self.adjust_voltage(domain);
        self.adjust_frequency(domain);
    }

    pub fn get_power_consumption(&self) -> f32 {
        self.domains.iter()
            .map(|(domain, state)| self.calculate_domain_power(*domain, *state))
            .sum()
    }
} 