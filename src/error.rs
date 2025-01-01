use std::fmt;

#[derive(Debug)]
pub enum NodeError {
    BlockchainError(String),
    NetworkError(String),
    WalletError(String),
    ValidationError(String),
    StorageError(String),
}

impl fmt::Display for NodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeError::BlockchainError(msg) => write!(f, "Blockchain error: {}", msg),
            NodeError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            NodeError::WalletError(msg) => write!(f, "Wallet error: {}", msg),
            NodeError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            NodeError::StorageError(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl std::error::Error for NodeError {}

pub type NodeResult<T> = Result<T, NodeError>;
