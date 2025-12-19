//! Error types for mobile wallet

use thiserror::Error;

/// Mobile wallet errors
#[derive(Error, Debug, Clone)]
pub enum MobileError {
    #[error("No wallet loaded")]
    NoWalletLoaded,
    
    #[error("Invalid password")]
    InvalidPassword,
    
    #[error("Wallet creation failed: {0}")]
    WalletCreationFailed(String),
    
    #[error("Invalid mnemonic")]
    InvalidMnemonic,
    
    #[error("Insufficient balance")]
    InsufficientBalance,
    
    #[error("Invalid transaction")]
    InvalidTransaction,
    
    #[error("Transaction failed: {0}")]
    TransactionFailed(String),
    
    #[error("Sync failed: {0}")]
    SyncFailed(String),
    
    #[error("Biometric authentication failed")]
    BiometricAuthFailed,
    
    #[error("Keystore error: {0}")]
    KeystoreError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Cryptographic error: {0}")]
    CryptoError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
}

/// Result type for mobile wallet operations
pub type Result<T> = std::result::Result<T, MobileError>;
