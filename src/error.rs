use serde::{Serialize, Deserialize};

/// Application error types
#[derive(Debug)]
pub enum AppError {
    /// Configuration error
    ConfigError(String),
    /// WebSocket connection error
    ConnectionError(String),
    /// RPC error
    RpcError(String),
    /// Parsing error
    ParseError(String),
    /// Server error
    ServerError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            AppError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            AppError::RpcError(msg) => write!(f, "RPC error: {}", msg),
            AppError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AppError::ServerError(msg) => write!(f, "Server error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

/// Result type alias for application errors
pub type AppResult<T> = Result<T, AppError>;

