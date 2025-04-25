// Power delivery system implementation
pub struct PowerDelivery {
    max_current: f32, // Amps
    voltage: f32, // Volts
    power_profiles: Vec<PowerProfile>,
    active_profile: usize,
    is_initialized: bool,
}

#[derive(Clone)]
pub struct PowerProfile {
    name: String,
    voltage: f32, // Volts
    max_current: f32, // Amps
}

impl PowerDelivery {
    pub fn new() -> Self {
        // Default power profiles (USB-PD compatible)
        let profiles = vec![
            PowerProfile { name: "USB 2.0".to_string(), voltage: 5.0, max_current: 0.5 },
            PowerProfile { name: "USB 3.0".to_string(), voltage: 5.0, max_current: 0.9 },
            PowerProfile { name: "USB-C Default".to_string(), voltage: 5.0, max_current: 3.0 },
            PowerProfile { name: "USB-PD 15W".to_string(), voltage: 5.0, max_current: 3.0 },
            PowerProfile { name: "USB-PD 27W".to_string(), voltage: 9.0, max_current: 3.0 },
            PowerProfile { name: "USB-PD 45W".to_string(), voltage: 15.0, max_current: 3.0 },
            PowerProfile { name: "USB-PD 60W".to_string(), voltage: 20.0, max_current: 3.0 },
            PowerProfile { name: "USB-PD 100W".to_string(), voltage: 20.0, max_current: 5.0 },
        ];
        
        Self {
            max_current: 3.0, // 3A default
            voltage: 5.0, // 5V default
            power_profiles: profiles,
            active_profile: 2, // USB-C Default
            is_initialized: false,
        }
    }
    
    pub fn initialize(&mut self) -> bool {
        println!("Power delivery system initialized");
        let profile = &self.power_profiles[self.active_profile];
        println!("Active profile: {} ({}V, {}A, {}W)", 
                profile.name, profile.voltage, profile.max_current,
                profile.voltage * profile.max_current);
        
        self.voltage = profile.voltage;
        self.max_current = profile.max_current;
        self.is_initialized = true;
        true
    }
    
    pub fn shutdown(&mut self) -> bool {
        self.is_initialized = false;
        true
    }
    
    pub fn set_profile(&mut self, profile_index: usize) -> bool {
        if !self.is_initialized || profile_index >= self.power_profiles.len() {
            return false;
        }
        
        self.active_profile = profile_index;
        let profile = &self.power_profiles[profile_index];
        
        self.voltage = profile.voltage;
        self.max_current = profile.max_current;
        
        println!("Switched to power profile: {} ({}V, {}A, {}W)", 
                profile.name, profile.voltage, profile.max_current,
                profile.voltage * profile.max_current);
        
        true
    }
    
    pub fn negotiate_power(&mut self, required_power: f32) -> bool {
        if !self.is_initialized {
            return false;
        }
        
        // Find the most efficient profile that can deliver the required power
        let mut best_profile = 0;
        let mut min_excess_power = f32::MAX;
        
        for (i, profile) in self.power_profiles.iter().enumerate() {
            let max_power = profile.voltage * profile.max_current;
            if max_power >= required_power {
                let excess = max_power - required_power;
                if excess < min_excess_power {
                    min_excess_power = excess;
                    best_profile = i;
                }
            }
        }
        
        self.set_profile(best_profile)
    }
    
    pub fn get_max_power(&self) -> f32 {
        self.voltage * self.max_current
    }
    
    pub fn get_available_profiles(&self) -> &[PowerProfile] {
        &self.power_profiles
    }
    
    pub fn get_active_profile(&self) -> &PowerProfile {
        &self.power_profiles[self.active_profile]
    }
} 