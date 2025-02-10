use ethers::{
    types::transaction::eip2718::TypedTransactionError,
    utils::{hex::FromHexError, ConversionError},
};
use std::fmt;

#[derive(Debug)]
pub struct SetupError {
    pub message: String,
}

impl SetupError {
    pub fn new(message: &str) -> Self {
        SetupError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for SetupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Setup Error: {}", self.message)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for SetupError {
    fn from(error: Box<dyn std::error::Error + Send + Sync>) -> Self {
        SetupError {
            message: format!("{}", error),
        }
    }
}

impl From<ConversionError> for SetupError {
    fn from(error: ConversionError) -> Self {
        SetupError {
            message: format!("Conversion Error: {}", error),
        }
    }
}

impl From<TypedTransactionError> for SetupError {
    fn from(error: TypedTransactionError) -> Self {
        SetupError {
            message: format!("Conversion Error: {}", error),
        }
    }
}

impl From<FromHexError> for SetupError {
    fn from(error: FromHexError) -> Self {
        SetupError {
            message: format!("Hex Conversion Error: {}", error),
        }
    }
}

impl From<String> for SetupError {
    fn from(message: String) -> Self {
        SetupError { message }
    }
}

impl From<&str> for SetupError {
    fn from(message: &str) -> Self {
        SetupError {
            message: message.to_string(),
        }
    }
}
