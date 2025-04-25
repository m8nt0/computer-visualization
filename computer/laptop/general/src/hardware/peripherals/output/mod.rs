// Output peripherals modules
pub mod display;
pub mod speaker;

// Export main peripherals for convenience
pub use self::display::Display;
pub use self::speaker::Speaker; 