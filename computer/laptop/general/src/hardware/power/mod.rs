pub mod battery;
pub mod power_delivery;
pub mod power_supply_unit;
pub mod thermal_management;
pub mod voltage_regulators;

use std::collections::HashMap;
use self::battery::Battery;
use self::power_supply_unit::PowerSupplyUnit;

// Main power management system
pub struct PowerManagement {
    battery: Battery,
    psu: PowerSupplyUnit,
    domains: HashMap<PowerDomain, PowerState>,
    current_source: PowerSource,
    is_initialized: bool,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
pub enum PowerDomain {
    CPU,
    GPU,
    Memory,
    Storage,
    Display,
    Network,
    Peripherals,
    System,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerState {
    Full,
    Balanced,
    PowerSave,
    Sleep,
    Hibernate,
    Off,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerSource {
    Battery,
    AC,
    External,
}

impl PowerManagement {
    pub fn new() -> Self {
        let mut domains = HashMap::new();
        for domain in [
            PowerDomain::CPU, 
            PowerDomain::GPU, 
            PowerDomain::Memory,
            PowerDomain::Storage, 
            PowerDomain::Display, 
            PowerDomain::Network,
            PowerDomain::Peripherals,
            PowerDomain::System
        ].iter() {
            domains.insert(*domain, PowerState::Off);
        }

        Self {
            battery: Battery::new(),
            psu: PowerSupplyUnit::new(),
            domains,
            current_source: PowerSource::Battery,
            is_initialized: false,
        }
    }
    
    pub fn initialize(&mut self) -> bool {
        println!("Initializing power management system");
        self.battery.initialize();
        self.psu.initialize();
        
        // Set all domains to balanced mode initially
        for domain in self.domains.values_mut() {
            *domain = PowerState::Balanced;
        }
        
        // Detect power source
        if self.psu.is_connected() {
            self.current_source = PowerSource::AC;
        } else {
            self.current_source = PowerSource::Battery;
        }
        
        self.is_initialized = true;
        true
    }
    
    pub fn shutdown(&mut self) -> bool {
        println!("Shutting down power management system");
        
        // Set all domains to off
        for domain in self.domains.values_mut() {
            *domain = PowerState::Off;
        }
        
        self.is_initialized = false;
        true
    }
    
    pub fn power_on(&mut self) -> bool {
        if !self.is_initialized {
            return self.initialize();
        }
        
        true
    }
    
    pub fn power_off(&mut self) -> bool {
        self.shutdown()
    }
    
    pub fn emergency_shutdown(&mut self) {
        println!("EMERGENCY SHUTDOWN INITIATED");
        
        // Immediately cut power to all domains
        for domain in self.domains.values_mut() {
            *domain = PowerState::Off;
        }
        
        self.is_initialized = false;
    }
    
    pub fn set_power_state(&mut self, domain: PowerDomain, state: PowerState) {
        if !self.is_initialized {
            return;
        }
        
        self.domains.insert(domain, state);
        
        // Apply power policy based on current source
        self.apply_power_policy(domain);
    }
    
    pub fn get_power_state(&self, domain: PowerDomain) -> PowerState {
        *self.domains.get(&domain).unwrap_or(&PowerState::Off)
    }
    
    fn apply_power_policy(&mut self, domain: PowerDomain) {
        // If on battery, limit high-power components
        if self.current_source == PowerSource::Battery {
            if domain == PowerDomain::GPU || domain == PowerDomain::CPU {
                let current = self.get_power_state(domain);
                
                // Don't allow full power on battery for power-hungry components
                if current == PowerState::Full {
                    self.domains.insert(domain, PowerState::Balanced);
                }
            }
        }
    }
    
    pub fn get_battery_percentage(&self) -> u8 {
        self.battery.get_percentage()
    }
    
    pub fn is_on_battery(&self) -> bool {
        self.current_source == PowerSource::Battery
    }
    
    pub fn get_power_source(&self) -> PowerSource {
        self.current_source
    }
    
    pub fn set_power_source(&mut self, source: PowerSource) {
        if self.current_source != source {
            self.current_source = source;
            
            // Apply power policies for all domains
            let domains_to_update: Vec<PowerDomain> = self.domains.keys().cloned().collect();
            for domain in domains_to_update {
                self.apply_power_policy(domain);
            }
        }
    }
} 