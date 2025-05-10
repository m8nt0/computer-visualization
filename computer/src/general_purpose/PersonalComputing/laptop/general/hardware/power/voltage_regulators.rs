// Voltage regulator implementation
use std::collections::HashMap;

pub struct VoltageRegulatorSystem {
    regulators: HashMap<String, VoltageRegulator>,
    is_initialized: bool,
}

pub struct VoltageRegulator {
    name: String,
    input_voltage: f32, // Volts
    output_voltage: f32, // Volts
    max_current: f32, // Amps
    efficiency: f32, // 0-1.0
    temperature: f32, // Celsius
    power_state: PowerState,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerState {
    On,
    LowPower,
    Off,
}

impl VoltageRegulatorSystem {
    pub fn new() -> Self {
        // Create default voltage regulators
        let mut regulators = HashMap::new();
        
        // CPU voltage regulator
        regulators.insert("CPU_VR".to_string(), VoltageRegulator {
            name: "CPU VR".to_string(),
            input_voltage: 12.0,
            output_voltage: 1.2,
            max_current: 15.0,
            efficiency: 0.92,
            temperature: 35.0,
            power_state: PowerState::Off,
        });
        
        // Memory voltage regulator
        regulators.insert("MEM_VR".to_string(), VoltageRegulator {
            name: "Memory VR".to_string(),
            input_voltage: 12.0,
            output_voltage: 1.35,
            max_current: 10.0,
            efficiency: 0.90,
            temperature: 33.0,
            power_state: PowerState::Off,
        });
        
        // GPU voltage regulator
        regulators.insert("GPU_VR".to_string(), VoltageRegulator {
            name: "GPU VR".to_string(),
            input_voltage: 12.0,
            output_voltage: 1.1,
            max_current: 20.0,
            efficiency: 0.91,
            temperature: 36.0,
            power_state: PowerState::Off,
        });
        
        Self {
            regulators,
            is_initialized: false,
        }
    }
    
    pub fn initialize(&mut self) -> bool {
        println!("Voltage regulator system initialized");
        
        // Power on all regulators
        for (_, regulator) in self.regulators.iter_mut() {
            regulator.power_state = PowerState::On;
        }
        
        self.is_initialized = true;
        true
    }
    
    pub fn shutdown(&mut self) -> bool {
        println!("Voltage regulator system shutting down");
        
        // Power off all regulators
        for (_, regulator) in self.regulators.iter_mut() {
            regulator.power_state = PowerState::Off;
        }
        
        self.is_initialized = false;
        true
    }
    
    pub fn set_voltage(&mut self, regulator_name: &str, voltage: f32) -> bool {
        if !self.is_initialized {
            return false;
        }
        
        if let Some(regulator) = self.regulators.get_mut(regulator_name) {
            // Check if voltage is within safe limits
            // Most voltage regulators have a specific range they can operate in
            let min_voltage = regulator.output_voltage * 0.8;
            let max_voltage = regulator.output_voltage * 1.2;
            
            if voltage >= min_voltage && voltage <= max_voltage {
                regulator.output_voltage = voltage;
                println!("{} voltage set to {:.2}V", regulator.name, voltage);
                return true;
            } else {
                println!("Warning: Attempted to set unsafe voltage for {}", regulator.name);
                return false;
            }
        }
        
        false
    }
    
    pub fn set_power_state(&mut self, regulator_name: &str, state: PowerState) -> bool {
        if !self.is_initialized {
            return false;
        }
        
        if let Some(regulator) = self.regulators.get_mut(regulator_name) {
            regulator.power_state = state;
            
            match state {
                PowerState::On => {
                    println!("{} powered on", regulator.name);
                }
                PowerState::LowPower => {
                    println!("{} set to low power mode", regulator.name);
                }
                PowerState::Off => {
                    println!("{} powered off", regulator.name);
                }
            }
            
            return true;
        }
        
        false
    }
    
    pub fn update_load(&mut self, regulator_name: &str, current: f32) {
        if !self.is_initialized {
            return;
        }
        
        if let Some(regulator) = self.regulators.get_mut(regulator_name) {
            if regulator.power_state == PowerState::Off {
                return;
            }
            
            // Clamp current to max current
            let actual_current = current.min(regulator.max_current);
            
            // Calculate power dissipation and update temperature
            let input_power = regulator.input_voltage * actual_current;
            let output_power = regulator.output_voltage * actual_current;
            let power_loss = input_power - (output_power / regulator.efficiency);
            
            // Temperature increases with power loss
            regulator.temperature = 30.0 + power_loss * 0.1;
        }
    }
    
    pub fn get_regulator(&self, name: &str) -> Option<&VoltageRegulator> {
        self.regulators.get(name)
    }
    
    pub fn add_regulator(&mut self, name: &str, input_v: f32, output_v: f32, max_a: f32) {
        if !self.is_initialized {
            return;
        }
        
        self.regulators.insert(name.to_string(), VoltageRegulator {
            name: name.to_string(),
            input_voltage: input_v,
            output_voltage: output_v,
            max_current: max_a,
            efficiency: 0.90, // Default efficiency
            temperature: 30.0,
            power_state: PowerState::Off,
        });
    }
} 