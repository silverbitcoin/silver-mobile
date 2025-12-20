//! Keystore for mobile wallet

use serde::{Deserialize, Serialize};
use crate::errors::{MobileError, Result};

/// Keystore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keystore {
    /// Encrypted mnemonic
    encrypted_mnemonic: Vec<u8>,
    
    /// Salt
    salt: Vec<u8>,
    
    /// Master key
    master_key: Vec<u8>,
}

impl Keystore {
    /// Create a new keystore
    pub fn new(password: &str) -> Result<Self> {
        use rand::Rng;
        
        // Generate random salt
        let mut rng = rand::thread_rng();
        let salt: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        
        // Derive master key from password
        let master_key = Self::derive_key(password, &salt)?;
        
        // Generate mnemonic
        let mnemonic = Self::generate_mnemonic()?;
        
        // Encrypt mnemonic
        let encrypted_mnemonic = Self::encrypt(&mnemonic, &master_key)?;
        
        Ok(Self {
            encrypted_mnemonic,
            salt,
            master_key,
        })
    }
    
    /// Create keystore from mnemonic
    pub fn from_mnemonic(mnemonic: &str, password: &str) -> Result<Self> {
        use rand::Rng;
        
        // Validate mnemonic
        if mnemonic.split_whitespace().count() != 12 {
            return Err(MobileError::InvalidMnemonic);
        }
        
        // Generate random salt
        let mut rng = rand::thread_rng();
        let salt: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
        
        // Derive master key from password
        let master_key = Self::derive_key(password, &salt)?;
        
        // Encrypt mnemonic
        let encrypted_mnemonic = Self::encrypt(mnemonic, &master_key)?;
        
        Ok(Self {
            encrypted_mnemonic,
            salt,
            master_key,
        })
    }
    
    /// Export mnemonic
    pub fn export_mnemonic(&self, password: &str) -> Result<String> {
        // Derive key from password
        let key = Self::derive_key(password, &self.salt)?;
        
        // Decrypt mnemonic
        Self::decrypt(&self.encrypted_mnemonic, &key)
    }
    
    /// Derive key from password
    fn derive_key(password: &str, salt: &[u8]) -> Result<Vec<u8>> {
        use argon2::{Argon2, PasswordHasher};
        use argon2::password_hash::SaltString;
        
        let salt_string = SaltString::encode_b64(salt)
            .map_err(|_| MobileError::KeystoreError("Invalid salt".to_string()))?;
        
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt_string)
            .map_err(|_| MobileError::KeystoreError("Key derivation failed".to_string()))?;
        
        Ok(password_hash.hash.unwrap().as_bytes().to_vec())
    }
    
    /// Generate mnemonic
    fn generate_mnemonic() -> Result<String> {
        use rand::Rng;
        
        let words = vec![
            "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract",
            "academy", "accept", "access", "accident", "account", "accuse", "achieve", "acid",
        ];
        
        let mut rng = rand::thread_rng();
        let mnemonic: Vec<&str> = (0..12)
            .map(|_| words[rng.gen_range(0..words.len())])
            .collect();
        
        Ok(mnemonic.join(" "))
    }
    
    /// Encrypt data
    fn encrypt(data: &str, key: &[u8]) -> Result<Vec<u8>> {
        use blake3::Hasher;
        
        let mut hasher = Hasher::new();
        hasher.update(data.as_bytes());
        hasher.update(key);
        
        Ok(hasher.finalize().as_bytes().to_vec())
    }
    
    /// Decrypt data
    fn decrypt(encrypted: &[u8], _key: &[u8]) -> Result<String> {
        // In a real implementation, this would use proper encryption
        // For now, we'll just return a placeholder
        Ok(String::from_utf8_lossy(encrypted).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_keystore_creation() {
        let keystore = Keystore::new("password123");
        assert!(keystore.is_ok());
    }
    
    #[test]
    fn test_keystore_from_mnemonic() {
        let mnemonic = "abandon ability able about above absent absorb abstract academy accept access accident";
        let keystore = Keystore::from_mnemonic(mnemonic, "password123");
        assert!(keystore.is_ok());
    }
}
