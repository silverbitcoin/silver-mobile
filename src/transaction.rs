//! Mobile transactions

use serde::{Deserialize, Serialize};
use crate::errors::{MobileError, Result};

/// Mobile transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileTransaction {
    /// Transaction ID
    pub id: String,
    
    /// From address
    pub from: String,
    
    /// To address
    pub to: String,
    
    /// Amount
    pub amount: u64,
    
    /// Fee
    pub fee: u64,
    
    /// Status
    pub status: TransactionStatus,
    
    /// Timestamp
    pub timestamp: u64,
}

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionStatus {
    /// Pending
    Pending,
    
    /// Confirmed
    Confirmed,
    
    /// Failed
    Failed,
}

impl MobileTransaction {
    /// Create a new transaction
    pub fn new(from: String, to: String, amount: u64, fee: u64) -> Result<Self> {
        if from.is_empty() || to.is_empty() {
            return Err(MobileError::InvalidTransaction);
        }
        
        if amount == 0 {
            return Err(MobileError::InvalidTransaction);
        }
        
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(from.as_bytes());
        hasher.update(to.as_bytes());
        hasher.update(&amount.to_le_bytes());
        
        let id = format!("tx_{}", hex::encode(hasher.finalize().as_bytes()));
        
        Ok(Self {
            id,
            from,
            to,
            amount,
            fee,
            status: TransactionStatus::Pending,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
    
    /// Get transaction ID
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Get total amount (amount + fee)
    pub fn total(&self) -> u64 {
        self.amount + self.fee
    }
    
    /// Set status
    pub fn set_status(&mut self, status: TransactionStatus) {
        self.status = status;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transaction_creation() {
        let tx = MobileTransaction::new(
            "silver_abc123".to_string(),
            "silver_def456".to_string(),
            1000,
            100,
        );
        assert!(tx.is_ok());
    }
    
    #[test]
    fn test_transaction_total() {
        let tx = MobileTransaction::new(
            "silver_abc123".to_string(),
            "silver_def456".to_string(),
            1000,
            100,
        ).unwrap();
        
        assert_eq!(tx.total(), 1100);
    }
}
