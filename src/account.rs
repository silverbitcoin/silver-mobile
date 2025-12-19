//! Account management for mobile wallet

use serde::{Deserialize, Serialize};
use crate::errors::Result;

/// Account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account index
    pub index: u32,
    
    /// Account name
    pub name: String,
    
    /// Address
    pub address: String,
    
    /// Public key
    pub public_key: Vec<u8>,
    
    /// Balance
    pub balance: u64,
}

impl Account {
    /// Create a new account
    pub fn new(index: u32) -> Result<Self> {
        use blake3::Hasher;
        
        // Generate address from index
        let mut hasher = Hasher::new();
        hasher.update(&index.to_le_bytes());
        let address_bytes = hasher.finalize().as_bytes().to_vec();
        let address = format!("silver_{}", hex::encode(&address_bytes[..8]));
        
        // Generate public key
        let mut key_hasher = Hasher::new();
        key_hasher.update(&index.to_le_bytes());
        key_hasher.update(b"public_key");
        let public_key = key_hasher.finalize().as_bytes().to_vec();
        
        Ok(Self {
            index,
            name: format!("Account {}", index),
            address,
            public_key,
            balance: 0,
        })
    }
    
    /// Get account index
    pub fn index(&self) -> u32 {
        self.index
    }
    
    /// Get account name
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Set account name
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    /// Get address
    pub fn address(&self) -> &str {
        &self.address
    }
    
    /// Get public key
    pub fn public_key(&self) -> &[u8] {
        &self.public_key
    }
    
    /// Get balance
    pub fn balance(&self) -> u64 {
        self.balance
    }
    
    /// Set balance
    pub fn set_balance(&mut self, balance: u64) {
        self.balance = balance;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_account_creation() {
        let account = Account::new(0);
        assert!(account.is_ok());
    }
    
    #[test]
    fn test_account_properties() {
        let account = Account::new(0).unwrap();
        assert_eq!(account.index(), 0);
        assert!(!account.address().is_empty());
    }
}
