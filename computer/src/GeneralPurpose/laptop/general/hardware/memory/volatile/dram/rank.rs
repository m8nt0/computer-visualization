use super::bank::MemoryBank;
use super::temperature::TempSensor;
use super::voltage::VoltageController;
use super::power::PowerState;

pub struct Rank {
    id: usize,
    banks: Vec<MemoryBank>,
    temperature: TempSensor,
    voltage: VoltageController,
    power_state: PowerState,
    
    // Rank state
    active_banks: u32,
    last_activate: u64,
    last_precharge: u64,
    
    // Statistics
    stats: RankStats,
}

struct RankStats {
    total_activates: u64,
    total_precharges: u64,
    bank_conflicts: u64,
    power_state_transitions: u64,
    cycles_active: u64,
    cycles_idle: u64,
    cycles_power_down: u64,
}

impl Rank {
    pub fn new(id: usize, num_banks: usize) -> Self {
        Self {
            id,
            banks: (0..num_banks).map(|id| MemoryBank::new(id)).collect(),
            temperature: TempSensor::new(),
            voltage: VoltageController::new(),
            power_state: PowerState::Active,
            active_banks: 0,
            last_activate: 0,
            last_precharge: 0,
            stats: RankStats::default(),
        }
    }

    pub fn activate_bank(&mut self, bank_id: usize, current_cycle: u64) -> Result<(), RankError> {
        if bank_id >= self.banks.len() {
            return Err(RankError::InvalidBank);
        }

        // Check timing constraints
        if current_cycle - self.last_activate < 4 { // tRRD (Rank to Rank Delay)
            return Err(RankError::TimingViolation);
        }

        // Check power constraints
        if self.active_banks >= 4 { // Maximum active banks per rank
            self.stats.bank_conflicts += 1;
            return Err(RankError::TooManyActiveBanks);
        }

        // Activate the bank
        self.banks[bank_id].activate(current_cycle)?;
        self.active_banks += 1;
        self.last_activate = current_cycle;
        self.stats.total_activates += 1;

        Ok(())
    }

    pub fn precharge_bank(&mut self, bank_id: usize, current_cycle: u64) -> Result<(), RankError> {
        if bank_id >= self.banks.len() {
            return Err(RankError::InvalidBank);
        }

        // Precharge the bank
        self.banks[bank_id].precharge(current_cycle)?;
        self.active_banks -= 1;
        self.last_precharge = current_cycle;
        self.stats.total_precharges += 1;

        Ok(())
    }

    pub fn update(&mut self, current_cycle: u64) {
        // Update all banks
        for bank in &mut self.banks {
            bank.update(current_cycle);
        }

        // Update temperature and voltage
        self.temperature.update();
        self.voltage.update();

        // Update power state
        self.update_power_state();

        // Update statistics
        match self.power_state {
            PowerState::Active => self.stats.cycles_active += 1,
            PowerState::Idle => self.stats.cycles_idle += 1,
            PowerState::PowerDown => self.stats.cycles_power_down += 1,
            _ => {}
        }
    }

    fn update_power_state(&mut self) {
        let new_state = match (self.active_banks, self.power_state) {
            (0, PowerState::Active) => PowerState::Idle,
            (0, PowerState::Idle) if self.idle_cycles() > 1000 => PowerState::PowerDown,
            (n, _) if n > 0 => PowerState::Active,
            _ => self.power_state,
        };

        if new_state != self.power_state {
            self.stats.power_state_transitions += 1;
            self.power_state = new_state;
        }
    }

    pub fn is_active(&self) -> bool {
        self.active_banks > 0
    }

    fn idle_cycles(&self) -> u64 {
        self.stats.cycles_idle
    }

    // Methods for visualization/monitoring
    pub fn get_active_banks(&self) -> u32 {
        self.active_banks
    }

    pub fn get_temperature(&self) -> f32 {
        self.temperature.get_temperature()
    }

    pub fn get_voltage(&self) -> f32 {
        self.voltage.get_current_voltage()
    }

    pub fn get_power_state(&self) -> PowerState {
        self.power_state
    }

    pub fn get_stats(&self) -> &RankStats {
        &self.stats
    }

    pub fn refresh_bank(&mut self, bank_id: usize, row: u32) -> Result<(), RankError> {
        if bank_id >= self.banks.len() {
            return Err(RankError::InvalidBank);
        }

        // Check if bank is in proper state for refresh
        if self.banks[bank_id].is_active() {
            // Need to precharge first
            self.precharge_bank(bank_id, self.last_precharge)?;
        }

        // Perform refresh operation
        self.banks[bank_id].refresh_row(row)?;
        
        Ok(())
    }

    pub fn refresh_all(&mut self, row: u32) -> Result<(), RankError> {
        // Precharge all banks if needed
        for (bank_id, bank) in self.banks.iter().enumerate() {
            if bank.is_active() {
                self.precharge_bank(bank_id, self.last_precharge)?;
            }
        }

        // Refresh all banks simultaneously
        for bank in &mut self.banks {
            bank.refresh_row(row)?;
        }

        Ok(())
    }

    pub fn handle_temperature_event(&mut self, current_temp: f32) -> Result<(), RankError> {
        // Adjust refresh rate or trigger emergency refresh based on temperature
        if current_temp > self.temperature.get_critical_threshold() {
            // Emergency measures
            self.power_state = PowerState::PowerDown;
            return Err(RankError::TemperatureError);
        }

        // Adjust timing parameters based on temperature
        self.adjust_timing_parameters(current_temp);
        Ok(())
    }

    fn adjust_timing_parameters(&mut self, temperature: f32) {
        // Adjust timing parameters based on temperature
        // Higher temperatures require more frequent refreshes
        let temp_factor = (temperature - 85.0).max(0.0) / 10.0; // Adjust every 10°C above 85°C
        
        for bank in &mut self.banks {
            bank.adjust_refresh_rate(temp_factor);
        }
    }

    pub fn enter_power_down(&mut self) -> Result<(), RankError> {
        // Check if we can enter power down
        if self.active_banks > 0 {
            return Err(RankError::PowerStateError);
        }

        self.power_state = PowerState::PowerDown;
        self.stats.power_state_transitions += 1;
        Ok(())
    }

    pub fn exit_power_down(&mut self) {
        self.power_state = PowerState::Idle;
        self.stats.power_state_transitions += 1;
    }

    pub fn enter_self_refresh(&mut self) -> Result<(), RankError> {
        // Can only enter self-refresh from idle or power-down
        match self.power_state {
            PowerState::Idle | PowerState::PowerDown => {
                self.power_state = PowerState::SelfRefresh;
                self.stats.power_state_transitions += 1;
                Ok(())
            },
            _ => Err(RankError::PowerStateError),
        }
    }

    pub fn exit_self_refresh(&mut self) {
        if self.power_state == PowerState::SelfRefresh {
            self.power_state = PowerState::Idle;
            self.stats.power_state_transitions += 1;
        }
    }

    // Add methods for bank management
    pub fn get_bank(&self, bank_id: usize) -> Option<&MemoryBank> {
        self.banks.get(bank_id)
    }

    pub fn get_bank_mut(&mut self, bank_id: usize) -> Option<&mut MemoryBank> {
        self.banks.get_mut(bank_id)
    }

    // Add methods for power management
    pub fn get_power_consumption(&self) -> f32 {
        let base_power = match self.power_state {
            PowerState::Active => 1.0,
            PowerState::Idle => 0.3,
            PowerState::PowerDown => 0.1,
            PowerState::SelfRefresh => 0.05,
        };

        // Add power from active banks
        base_power + (self.active_banks as f32 * 0.2)
    }

    // Add methods for thermal management
    pub fn get_thermal_status(&self) -> ThermalStatus {
        let temp = self.temperature.get_temperature();
        if temp > 95.0 {
            ThermalStatus::Critical
        } else if temp > 85.0 {
            ThermalStatus::Warning
        } else {
            ThermalStatus::Normal
        }
    }
}

#[derive(Debug)]
pub enum RankError {
    InvalidBank,
    TimingViolation,
    TooManyActiveBanks,
    PowerStateError,
    TemperatureError,
    VoltageError,
}

#[derive(Debug, PartialEq)]
pub enum ThermalStatus {
    Normal,
    Warning,
    Critical,
}
