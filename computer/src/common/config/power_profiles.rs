// Energy consumption and management profiles
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerProfile {
    Performance,
    Balanced,
    PowerSaver,
    Custom,
}

impl PowerProfile {
    pub fn as_str(&self) -> &'static str {
        match self {
            PowerProfile::Performance => "Performance",
            PowerProfile::Balanced => "Balanced",
            PowerProfile::PowerSaver => "Power Saver",
            PowerProfile::Custom => "Custom",
        }
    }
}

#[derive(Debug, Clone)]
pub struct PowerSettings {
    profile: PowerProfile,
    cpu_power_limit: Option<u32>,       // In watts
    cpu_performance_mode: CpuMode,
    display_brightness: u8,             // 0-100%
    display_timeout: Duration,
    sleep_timeout: Duration,
    hibernate_timeout: Option<Duration>,
    wifi_power_mode: WifiPowerMode,
    background_activity: bool,
    thermal_policy: ThermalPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuMode {
    Maximum,
    Variable,
    PowerEfficient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiPowerMode {
    Maximum,
    Standard,
    LowPower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThermalPolicy {
    Passive,    // Reduce power before increasing fan speed
    Active,     // Increase fan speed before reducing power
    Balanced,   // Balance between the two approaches
}

impl Default for PowerSettings {
    fn default() -> Self {
        Self {
            profile: PowerProfile::Balanced,
            cpu_power_limit: None,
            cpu_performance_mode: CpuMode::Variable,
            display_brightness: 70,
            display_timeout: Duration::from_secs(10 * 60),    // 10 minutes
            sleep_timeout: Duration::from_secs(30 * 60),      // 30 minutes
            hibernate_timeout: Some(Duration::from_secs(3 * 60 * 60)), // 3 hours
            wifi_power_mode: WifiPowerMode::Standard,
            background_activity: true,
            thermal_policy: ThermalPolicy::Balanced,
        }
    }
}

impl PowerSettings {
    pub fn new(profile: PowerProfile) -> Self {
        match profile {
            PowerProfile::Performance => Self {
                profile,
                cpu_power_limit: None,
                cpu_performance_mode: CpuMode::Maximum,
                display_brightness: 100,
                display_timeout: Duration::from_secs(30 * 60), // 30 minutes
                sleep_timeout: Duration::from_secs(3 * 60 * 60), // 3 hours
                hibernate_timeout: None,
                wifi_power_mode: WifiPowerMode::Maximum,
                background_activity: true,
                thermal_policy: ThermalPolicy::Active,
            },
            PowerProfile::PowerSaver => Self {
                profile,
                cpu_power_limit: Some(15),
                cpu_performance_mode: CpuMode::PowerEfficient,
                display_brightness: 40,
                display_timeout: Duration::from_secs(2 * 60), // 2 minutes
                sleep_timeout: Duration::from_secs(5 * 60), // 5 minutes
                hibernate_timeout: Some(Duration::from_secs(30 * 60)), // 30 minutes
                wifi_power_mode: WifiPowerMode::LowPower,
                background_activity: false,
                thermal_policy: ThermalPolicy::Passive,
            },
            PowerProfile::Balanced => Self::default(),
            PowerProfile::Custom => Self::default(),
        }
    }
    
    pub fn profile(&self) -> PowerProfile {
        self.profile
    }
    
    pub fn cpu_power_limit(&self) -> Option<u32> {
        self.cpu_power_limit
    }
    
    pub fn set_cpu_power_limit(&mut self, limit: Option<u32>) {
        self.cpu_power_limit = limit;
        self.profile = PowerProfile::Custom;
    }
    
    pub fn cpu_performance_mode(&self) -> CpuMode {
        self.cpu_performance_mode
    }
    
    pub fn set_cpu_performance_mode(&mut self, mode: CpuMode) {
        self.cpu_performance_mode = mode;
        self.profile = PowerProfile::Custom;
    }
    
    pub fn display_brightness(&self) -> u8 {
        self.display_brightness
    }
    
    pub fn set_display_brightness(&mut self, brightness: u8) {
        self.display_brightness = brightness.min(100);
        self.profile = PowerProfile::Custom;
    }
    
    pub fn display_timeout(&self) -> Duration {
        self.display_timeout
    }
    
    pub fn set_display_timeout(&mut self, timeout: Duration) {
        self.display_timeout = timeout;
        self.profile = PowerProfile::Custom;
    }
    
    pub fn sleep_timeout(&self) -> Duration {
        self.sleep_timeout
    }
    
    pub fn set_sleep_timeout(&mut self, timeout: Duration) {
        self.sleep_timeout = timeout;
        self.profile = PowerProfile::Custom;
    }
    
    pub fn hibernate_timeout(&self) -> Option<Duration> {
        self.hibernate_timeout
    }
    
    pub fn set_hibernate_timeout(&mut self, timeout: Option<Duration>) {
        self.hibernate_timeout = timeout;
        self.profile = PowerProfile::Custom;
    }
    
    pub fn wifi_power_mode(&self) -> WifiPowerMode {
        self.wifi_power_mode
    }
    
    pub fn set_wifi_power_mode(&mut self, mode: WifiPowerMode) {
        self.wifi_power_mode = mode;
        self.profile = PowerProfile::Custom;
    }
    
    pub fn background_activity(&self) -> bool {
        self.background_activity
    }
    
    pub fn set_background_activity(&mut self, enabled: bool) {
        self.background_activity = enabled;
        self.profile = PowerProfile::Custom;
    }
    
    pub fn thermal_policy(&self) -> ThermalPolicy {
        self.thermal_policy
    }
    
    pub fn set_thermal_policy(&mut self, policy: ThermalPolicy) {
        self.thermal_policy = policy;
        self.profile = PowerProfile::Custom;
    }
    
    pub fn estimated_battery_life_multiplier(&self) -> f64 {
        match self.profile {
            PowerProfile::Performance => 0.7,
            PowerProfile::Balanced => 1.0,
            PowerProfile::PowerSaver => 1.5,
            PowerProfile::Custom => {
                // Rough estimation based on settings
                let mut multiplier = 1.0;
                
                // CPU mode impact
                match self.cpu_performance_mode {
                    CpuMode::Maximum => multiplier *= 0.7,
                    CpuMode::Variable => multiplier *= 1.0,
                    CpuMode::PowerEfficient => multiplier *= 1.3,
                }
                
                // Display brightness impact (rough approximation)
                multiplier *= 1.0 - (self.display_brightness as f64 * 0.3 / 100.0);
                
                // WiFi impact
                match self.wifi_power_mode {
                    WifiPowerMode::Maximum => multiplier *= 0.9,
                    WifiPowerMode::Standard => multiplier *= 1.0,
                    WifiPowerMode::LowPower => multiplier *= 1.1,
                }
                
                // Background activity impact
                if !self.background_activity {
                    multiplier *= 1.1;
                }
                
                multiplier
            }
        }
    }
} 