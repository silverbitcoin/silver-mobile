//! Mobile Wallet for SilverBitcoin
//!
//! This module provides a complete mobile wallet implementation for iOS and Android.
//! Features:
//! - Secure key management
//! - Transaction creation and signing
//! - Balance tracking
//! - Transaction history
//! - QR code support
//! - Biometric authentication

pub mod account;
pub mod errors;
pub mod keystore;
pub mod transaction;
pub mod wallet;
pub mod sync;
pub mod security;

pub use account::Account;
pub use errors::{MobileError, Result};
pub use keystore::Keystore;
pub use transaction::MobileTransaction;
pub use wallet::MobileWallet;
pub use sync::SyncManager;
pub use security::SecurityManager;

use std::sync::Arc;
use parking_lot::RwLock;

/// Mobile wallet version
pub const MOBILE_WALLET_VERSION: &str = "1.0.0";

/// Mobile wallet manager
#[derive(Clone, Debug)]
pub struct MobileWalletManager {
    /// Wallet
    wallet: Arc<RwLock<Option<MobileWallet>>>,
    
    /// Security manager
    security_manager: Arc<SecurityManager>,
    
    /// Sync manager
    sync_manager: Arc<SyncManager>,
}

impl MobileWalletManager {
    /// Create a new mobile wallet manager
    pub fn new() -> Result<Self> {
        Ok(Self {
            wallet: Arc::new(RwLock::new(None)),
            security_manager: Arc::new(SecurityManager::new()?),
            sync_manager: Arc::new(SyncManager::new()?),
        })
    }
    
    /// Create a new wallet
    pub fn create_wallet(&self, password: &str) -> Result<MobileWallet> {
        // Validate password
        self.security_manager.validate_password(password)?;
        
        // Create wallet
        let wallet = MobileWallet::new(password)?;
        
        // Store wallet
        let mut stored_wallet = self.wallet.write();
        *stored_wallet = Some(wallet.clone());
        
        Ok(wallet)
    }
    
    /// Import wallet from mnemonic
    pub fn import_wallet(&self, mnemonic: &str, password: &str) -> Result<MobileWallet> {
        // Validate password
        self.security_manager.validate_password(password)?;
        
        // Import wallet
        let wallet = MobileWallet::from_mnemonic(mnemonic, password)?;
        
        // Store wallet
        let mut stored_wallet = self.wallet.write();
        *stored_wallet = Some(wallet.clone());
        
        Ok(wallet)
    }
    
    /// Get active wallet
    pub fn get_wallet(&self) -> Result<MobileWallet> {
        let wallet = self.wallet.read();
        wallet
            .clone()
            .ok_or(MobileError::NoWalletLoaded)
    }
    
    /// Create a transaction
    pub fn create_transaction(
        &self,
        recipient: &str,
        amount: u64,
        fee: u64,
    ) -> Result<MobileTransaction> {
        let wallet = self.get_wallet()?;
        wallet.create_transaction(recipient, amount, fee)
    }
    
    /// Get balance
    pub fn get_balance(&self) -> Result<u64> {
        let wallet = self.get_wallet()?;
        Ok(wallet.balance())
    }
    
    /// Sync wallet
    pub fn sync(&self) -> Result<()> {
        let wallet = self.get_wallet()?;
        self.sync_manager.sync(&wallet)
    }
    
    /// Get transaction history
    pub fn get_transaction_history(&self) -> Result<Vec<MobileTransaction>> {
        let wallet = self.get_wallet()?;
        Ok(wallet.transaction_history())
    }
}

impl Default for MobileWalletManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            Self {
                wallet: Arc::new(RwLock::new(None)),
                security_manager: Arc::new(SecurityManager::new().unwrap_or_default()),
                sync_manager: Arc::new(SyncManager::new().unwrap_or_default()),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mobile_wallet_manager_creation() {
        let manager = MobileWalletManager::new();
        assert!(manager.is_ok());
    }
}
