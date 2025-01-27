pub struct VoltageController {
    current_voltage: f32,      // Current voltage in volts
    target_voltage: f32,       // Target voltage
    voltage_step: f32,         // Voltage change per step
    min_voltage: f32,          // Minimum allowed voltage
    max_voltage: f32,          // Maximum allowed voltage
    voltage_margin: f32,       // Safety margin
    stats: VoltageStats,
}

struct VoltageStats {
    voltage_transitions: u64,
    time_at_min_voltage: u64,
    time_at_max_voltage: u64,
    voltage_violations: u64,
}

impl VoltageController {
    pub fn new() -> Self {
        Self {
            current_voltage: 1.2,    // DDR4 nominal voltage
            target_voltage: 1.2,
            voltage_step: 0.01,      // 10mV steps
            min_voltage: 1.14,       // -5% tolerance
            max_voltage: 1.26,       // +5% tolerance
            voltage_margin: 0.02,    // 20mV margin
            stats: VoltageStats::default(),
        }
    }

    pub fn update(&mut self) {
        if self.current_voltage != self.target_voltage {
            let diff = self.target_voltage - self.current_voltage;
            let step = diff.signum() * self.voltage_step.min(diff.abs());
            self.current_voltage += step;
            self.stats.voltage_transitions += 1;
        }

        // Track statistics
        if (self.current_voltage - self.min_voltage).abs() < 0.001 {
            self.stats.time_at_min_voltage += 1;
        }
        if (self.current_voltage - self.max_voltage).abs() < 0.001 {
            self.stats.time_at_max_voltage += 1;
        }
        if self.current_voltage < self.min_voltage || self.current_voltage > self.max_voltage {
            self.stats.voltage_violations += 1;
        }
    }

    pub fn set_target_voltage(&mut self, voltage: f32) -> Result<(), VoltageError> {
        if voltage < self.min_voltage || voltage > self.max_voltage {
            return Err(VoltageError::OutOfRange);
        }
        self.target_voltage = voltage;
        Ok(())
    }

    pub fn get_current_voltage(&self) -> f32 {
        self.current_voltage
    }

    pub fn is_stable(&self) -> bool {
        (self.current_voltage - self.target_voltage).abs() < 0.001
    }

    pub fn get_voltage_margin(&self) -> f32 {
        let to_min = self.current_voltage - self.min_voltage;
        let to_max = self.max_voltage - self.current_voltage;
        to_min.min(to_max)
    }
}

#[derive(Debug)]
pub enum VoltageError {
    OutOfRange,
    TransitionFailed,
    NoiseMarginViolation,
}
