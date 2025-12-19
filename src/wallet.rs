//! Mobile wallet implementation

use serde::{Deserialize, Serialize};
use crate::errors::{MobileError, Result};
use crate::account::Account;
use crate::transaction::MobileTransaction;
use crate::keystore::Keystore;

/// Mobile wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileWallet {
    /// Wallet ID
    id: String,
    
    /// Accounts
    accounts: Vec<Account>,
    
    /// Active account index
    active_account: usize,
    
    /// Keystore
    keystore: Keystore,
    
    /// Balance
    balance: u64,
    
    /// Transaction history
    transaction_history: Vec<MobileTransaction>,
}

impl MobileWallet {
    /// Create a new wallet
    pub fn new(password: &str) -> Result<Self> {
        if password.len() < 8 {
            return Err(MobileError::InvalidPassword);
        }
        
        let keystore = Keystore::new(password)?;
        let account = Account::new(0)?;
        
        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            accounts: vec![account],
            active_account: 0,
            keystore,
            balance: 0,
            transaction_history: Vec::new(),
        })
    }
    
    /// Create wallet from mnemonic
    pub fn from_mnemonic(mnemonic: &str, password: &str) -> Result<Self> {
        if password.len() < 8 {
            return Err(MobileError::InvalidPassword);
        }
        
        let keystore = Keystore::from_mnemonic(mnemonic, password)?;
        let account = Account::new(0)?;
        
        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            accounts: vec![account],
            active_account: 0,
            keystore,
            balance: 0,
            transaction_history: Vec::new(),
        })
    }
    
    /// Get wallet ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get balance
    pub fn balance(&self) -> u64 {
        self.balance
    }
    
    /// Set balance
    pub fn set_balance(&mut self, balance: u64) {
        self.balance = balance;
    }
    
    /// Get active account
    pub fn active_account(&self) -> &Account {
        &self.accounts[self.active_account]
    }
    
    /// Get all accounts
    pub fn accounts(&self) -> &[Account] {
        &self.accounts
    }
    
    /// Add account
    pub fn add_account(&mut self) -> Result<()> {
        let account = Account::new(self.accounts.len() as u32)?;
        self.accounts.push(account);
        Ok(())
    }
    
    /// Create a transaction
    pub fn create_transaction(
        &self,
        recipient: &str,
        amount: u64,
        fee: u64,
    ) -> Result<MobileTransaction> {
        if amount + fee > self.balance {
            return Err(MobileError::InsufficientBalance);
        }
        
        let account = self.active_account();
        let transaction = MobileTransaction::new(
            account.address().to_string(),
            recipient.to_string(),
            amount,
            fee,
        )?;
        
        Ok(transaction)
    }
    
    /// Add transaction to history
    pub fn add_transaction(&mut self, transaction: MobileTransaction) {
        self.transaction_history.push(transaction);
    }
    
    /// Get transaction history
    pub fn transaction_history(&self) -> Vec<MobileTransaction> {
        self.transaction_history.clone()
    }
    
    /// Export mnemonic
    pub fn export_mnemonic(&self, password: &str) -> Result<String> {
        self.keystore.export_mnemonic(password)
    }
}

// UUID support
mod uuid {
    use std::fmt;
    
    pub struct Uuid([u8; 16]);
    
    impl Uuid {
        pub fn new_v4() -> Self {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            let mut bytes = [0u8; 16];
            rng.fill(&mut bytes);
            Uuid(bytes)
        }
    }
    
    impl fmt::Display for Uuid {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                self.0[0], self.0[1], self.0[2], self.0[3],
                self.0[4], self.0[5],
                self.0[6], self.0[7],
                self.0[8], self.0[9],
                self.0[10], self.0[11], self.0[12], self.0[13], self.0[14], self.0[15]
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_wallet_creation() {
        let wallet = MobileWallet::new("password123");
        assert!(wallet.is_ok());
    }
    
    #[test]
    fn test_wallet_balance() {
        let mut wallet = MobileWallet::new("password123").unwrap();
        wallet.set_balance(1000);
        assert_eq!(wallet.balance(), 1000);
    }
}
