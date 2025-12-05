pub mod grpc {
    tonic::include_proto!("noah");
}

pub use grpc::*;

#[derive(Debug, thiserror::Error)]
pub enum NoahError {
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}

pub type Result<T> = std::result::Result<T, NoahError>;
