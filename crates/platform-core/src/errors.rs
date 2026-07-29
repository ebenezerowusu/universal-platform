use thiserror::Error;

pub type PlatformResult<T> = Result<T, PlatformError>;

#[derive(Debug, Error)]
pub enum PlatformError {
    #[error("configuration error: {0}")]
    Configuration(String),

    #[error("validation error: {0}")]
    Validation(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("tenant isolation error: {0}")]
    TenantIsolation(String),

    #[error("infrastructure error: {0}")]
    Infrastructure(String),

    #[error("unexpected platform error: {0}")]
    Unexpected(String),
}
