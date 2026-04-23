use core_domain::CoreError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreApplicationError {
    #[error("domain error: {0}")]
    Domain(#[from] CoreError),

    #[error("not found")]
    NotFound,

    #[error("infrastructure failure")]
    Infrastructure {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl CoreApplicationError {
    pub fn infrastructure<E: std::error::Error + Send + Sync + 'static>(err: E) -> Self {
        Self::Infrastructure {
            source: Box::new(err),
        }
    }
}
