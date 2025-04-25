// Thermal paste implementation
pub struct ThermalPaste {
    name: String,
    thermal_conductivity: f32, // W/(m·K)
    applied_thickness: f32, // mm
    surface_area: f32, // cm²
    age_days: u32, // Age affects performance
}

impl ThermalPaste {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            thermal_conductivity: 8.5, // Standard thermal paste
            applied_thickness: 0.1, // 0.1mm is typical
            surface_area: 25.0, // 25 cm²
            age_days: 0,
        }
    }
    
    pub fn with_specs(name: &str, conductivity: f32, thickness: f32, area: f32) -> Self {
        Self {
            name: name.to_string(),
            thermal_conductivity: conductivity,
            applied_thickness: thickness,
            surface_area: area,
            age_days: 0,
        }
    }
    
    pub fn get_heat_transfer_coefficient(&self) -> f32 {
        // Calculate heat transfer coefficient based on thermal conductivity and thickness
        // Higher is better, decays with age
        let age_factor = 1.0 - (self.age_days as f32 / 1000.0).min(0.3); // Max 30% degradation
        let thickness_factor = 0.1 / self.applied_thickness.max(0.01); // Thinner is better
        
        self.thermal_conductivity * thickness_factor * age_factor
    }
    
    pub fn apply(&mut self, thickness: f32, area: f32) {
        self.applied_thickness = thickness;
        self.surface_area = area;
        self.age_days = 0; // Reset age after reapplication
    }
    
    pub fn age(&mut self, days: u32) {
        self.age_days += days;
    }
    
    pub fn get_efficiency(&self) -> f32 {
        // Calculate efficiency percentage (0-100%)
        let htc = self.get_heat_transfer_coefficient();
        let max_htc = self.thermal_conductivity * 10.0; // Theoretical max
        
        (htc / max_htc * 100.0).min(100.0)
    }
    
    pub fn get_name(&self) -> &str {
        &self.name
    }
    
    pub fn needs_replacement(&self) -> bool {
        self.age_days > 730 || self.get_efficiency() < 50.0 // 2 years or bad efficiency
    }
} 