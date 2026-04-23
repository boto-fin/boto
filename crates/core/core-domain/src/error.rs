use crate::CoreId;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    #[error("validation failed: {reason}")]
    Validation { reason: String },

    #[error("business rule violated: {rule}")]
    BusinessRule { rule: String },

    #[error("not found: {0}")]
    NotFound(CoreId),

    #[error("infrastructure failure: {0}")]
    Infrastructure(String),
}

impl CoreError {
    pub fn infrastructure<E: std::error::Error>(err: E) -> Self {
        Self::Infrastructure(err.to_string())
    }
}
