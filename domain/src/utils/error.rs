use strum_macros::Display;
use uuid::Error as UuidError;

#[derive(Debug, thiserror::Error)]
#[allow(unused)]
pub enum AppError {
    #[error("Could not find resource")]
    NotFound,
    #[error("Could not get info from datasource")]
    DatasourceError(String),
    #[error("ServerError")]
    ServerError(String),
    #[error("No Extensions")]
    ErrorWithoutExtensions,
    #[error("ValidationError:  {reason:?}  {code:?}")]
    ValidationError { reason: String, code: String },
    #[error("File size exceeds the maximum limit {0}")]
    MaxFileSizeError(String),
    #[error("Content Type not allowed {0}")]
    ContentTypeError(String),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
}

#[derive(Debug, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
#[allow(unused)]
pub enum AppErrorRetry {
    None,
    Retry,
    WaitAndRetry,
    Cancel,
}

impl From<UuidError> for AppError {
    fn from(error: UuidError) -> Self {
        AppError::DatasourceError(error.to_string())
    }
}
