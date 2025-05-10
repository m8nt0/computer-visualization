// Configuration management

/// A trait for devices that can be configured
pub trait Configurable {
    /// Get a configuration value by key
    fn get_config(&self, key: &str) -> Option<String>;
    
    /// Set a configuration value
    fn set_config(&mut self, key: &str, value: &str) -> Result<(), String>;
    
    /// Reset configuration to defaults
    fn reset_config(&mut self) -> Result<(), String>;
    
    /// Get all configuration keys
    fn get_config_keys(&self) -> Vec<String>;
    
    /// Check if a configuration key exists
    fn has_config(&self, key: &str) -> bool {
        self.get_config(key).is_some()
    }
    
    /// Export configuration to a string
    fn export_config(&self) -> String {
        let mut result = String::new();
        
        for key in self.get_config_keys() {
            if let Some(value) = self.get_config(&key) {
                result.push_str(&format!("{}={}\n", key, value));
            }
        }
        
        result
    }
    
    /// Import configuration from a string
    fn import_config(&mut self, config: &str) -> Result<(), String> {
        for line in config.lines() {
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }
            
            if let Some(pos) = line.find('=') {
                let key = line[..pos].trim();
                let value = line[pos+1..].trim();
                
                self.set_config(key, value)?;
            } else {
                return Err(format!("Invalid configuration line: {}", line));
            }
        }
        
        Ok(())
    }
    
    /// Check if configuration is valid
    fn validate_config(&self) -> Result<(), String> {
        // Default implementation assumes valid
        Ok(())
    }
    
    /// Get available configuration options with descriptions
    fn get_config_options(&self) -> Vec<(String, String)> {
        // Default implementation returns empty vector
        Vec::new()
    }
}
