// Cooling system components
pub mod fans;
pub mod heat_pipes;
pub mod thermal_paste;
pub mod thermal_sensors;
pub mod main;

// Export main cooling system
pub use self::main::CoolingSystem;
