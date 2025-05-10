// Power saving modes
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum PowerState {
    Active,
    Idle,
    Sleep,
    Hibernate,
    Off,
}

#[derive(Debug, Clone)]
pub struct PowerManager {
    current_state: PowerState,
    last_activity: Instant,
    idle_timeout: Duration,
    sleep_timeout: Duration,
}

impl PowerManager {
    pub fn new(idle_timeout_secs: u64, sleep_timeout_secs: u64) -> Self {
        Self {
            current_state: PowerState::Active,
            last_activity: Instant::now(),
            idle_timeout: Duration::from_secs(idle_timeout_secs),
            sleep_timeout: Duration::from_secs(sleep_timeout_secs),
        }
    }
    
    pub fn current_state(&self) -> &PowerState {
        &self.current_state
    }
    
    pub fn record_activity(&mut self) {
        self.last_activity = Instant::now();
        
        // If we were in a power saving state, return to active
        if self.current_state != PowerState::Active {
            self.current_state = PowerState::Active;
        }
    }
    
    pub fn check_state(&mut self) {
        let elapsed = self.last_activity.elapsed();
        
        match self.current_state {
            PowerState::Active if elapsed >= self.idle_timeout => {
                self.current_state = PowerState::Idle;
            },
            PowerState::Idle if elapsed >= self.sleep_timeout => {
                self.current_state = PowerState::Sleep;
            },
            _ => {} // No change for other states
        }
    }
    
    pub fn sleep(&mut self) -> Result<(), &'static str> {
        if self.current_state == PowerState::Off {
            return Err("Cannot sleep from Off state");
        }
        
        self.current_state = PowerState::Sleep;
        Ok(())
    }
    
    pub fn hibernate(&mut self) -> Result<(), &'static str> {
        if self.current_state == PowerState::Off {
            return Err("Cannot hibernate from Off state");
        }
        
        self.current_state = PowerState::Hibernate;
        Ok(())
    }
    
    pub fn wake(&mut self) -> Result<(), &'static str> {
        match self.current_state {
            PowerState::Sleep | PowerState::Hibernate | PowerState::Idle => {
                self.current_state = PowerState::Active;
                self.record_activity();
                Ok(())
            },
            PowerState::Off => Err("Cannot wake from Off state"),
            PowerState::Active => Ok(()) // Already active
        }
    }
    
    pub fn power_off(&mut self) {
        self.current_state = PowerState::Off;
    }
    
    pub fn power_on(&mut self) -> Result<(), &'static str> {
        if self.current_state != PowerState::Off {
            return Err("Device is already powered on");
        }
        
        self.current_state = PowerState::Active;
        self.record_activity();
        Ok(())
    }
}

// Function to calculate power consumption based on power state
pub fn calculate_power_consumption(state: &PowerState, base_consumption: f32) -> f32 {
    match state {
        PowerState::Active => base_consumption,
        PowerState::Idle => base_consumption * 0.6,
        PowerState::Sleep => base_consumption * 0.1,
        PowerState::Hibernate => base_consumption * 0.02,
        PowerState::Off => 0.0,
    }
} 