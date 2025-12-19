//! Security features for mobile wallet

use crate::errors::{MobileError, Result};
use serde::{Deserialize, Serialize};

/// Security manager
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SecurityManager {
    /// Biometric enabled
    biometric_enabled: bool,
    
    /// PIN enabled
    pin_enabled: bool,
}

impl SecurityManager {
    /// Create a new security manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            biometric_enabled: false,
            pin_enabled: false,
        })
    }
    
    /// Validate password
    pub fn validate_password(&self, password: &str) -> Result<()> {
        if password.len() < 8 {
            return Err(MobileError::InvalidPassword);
        }
        
        // Check for complexity
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        
        if !has_upper || !has_lower || !has_digit {
            return Err(MobileError::InvalidPassword);
        }
        
        Ok(())
    }
    
    /// Enable biometric authentication
    pub fn enable_biometric(&mut self) -> Result<()> {
        self.biometric_enabled = true;
        Ok(())
    }
    
    /// Disable biometric authentication
    pub fn disable_biometric(&mut self) {
        self.biometric_enabled = false;
    }
    
    /// Is biometric enabled
    pub fn is_biometric_enabled(&self) -> bool {
        self.biometric_enabled
    }
    
    /// Enable PIN
    pub fn enable_pin(&mut self) -> Result<()> {
        self.pin_enabled = true;
        Ok(())
    }
    
    /// Disable PIN
    pub fn disable_pin(&mut self) {
        self.pin_enabled = false;
    }
    
    /// Is PIN enabled
    pub fn is_pin_enabled(&self) -> bool {
        self.pin_enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_manager_creation() {
        let manager = SecurityManager::new();
        assert!(manager.is_ok());
    }
    
    #[test]
    fn test_password_validation() {
        let manager = SecurityManager::new().unwrap();
        
        // Valid password
        assert!(manager.validate_password("ValidPass123").is_ok());
        
        // Invalid password (too short)
        assert!(manager.validate_password("short").is_err());
    }
}
