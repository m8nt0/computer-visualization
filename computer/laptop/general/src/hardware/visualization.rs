use super::types::PhysicalAddress;

/// Trait that defines the visualization interface for hardware components
pub trait HardwareVisualizer {
    /// Update the visualizer with the current state of the hardware component
    fn update(&mut self, component: &dyn HardwareComponent);
    
    /// Render the visual representation of the hardware component
    fn render(&self);
}

/// Trait that defines the interface for hardware components that can be visualized
pub trait HardwareComponent {
    /// Get the current state of the component for visualization
    fn get_state(&self) -> ComponentState;
    
    /// Get the physical address of the component
    fn get_address(&self) -> PhysicalAddress;
    
    /// Get the name of the component
    fn get_name(&self) -> &str;
}

/// Structure representing the state of a hardware component
pub struct ComponentState {
    pub is_active: bool,
    pub power_consumption: f32,
    pub temperature: f32,
    pub utilization: f32,
    pub error_state: Option<String>,
    pub custom_data: Vec<u8>, // For component-specific data
} 