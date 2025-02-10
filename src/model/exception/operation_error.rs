use ethers::{
    types::transaction::eip2718::TypedTransactionError,
    utils::{hex::FromHexError, ConversionError},
};
use std::fmt;

#[derive(Debug)]
pub struct OperationError {
    pub message: String,
}

impl OperationError {
    pub fn new(message: &str) -> Self {
        OperationError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operation Error: {}", self.message)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for OperationError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        OperationError {
            message: format!("{}", error),
        }
    }
}

impl From<ConversionError> for OperationError {
    fn from(error: ConversionError) -> Self {
        OperationError {
            message: format!("Conversion Error: {}", error),
        }
    }
}

impl From<TypedTransactionError> for OperationError {
    fn from(error: TypedTransactionError) -> Self {
        OperationError {
            message: format!("Conversion Error: {}", error),
        }
    }
}

impl From<FromHexError> for OperationError {
    fn from(error: FromHexError) -> Self {
        OperationError {
            message: format!("Hex Conversion Error: {}", error),
        }
    }
}

impl From<String> for OperationError {
    fn from(message: String) -> Self {
        OperationError { message }
    }
}

impl From<&str> for OperationError {
    fn from(message: &str) -> Self {
        OperationError {
            message: message.to_string(),
        }
    }
}
