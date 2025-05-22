#[derive(Debug, thiserror::Error)]
pub enum OperationalError {
    #[error("{0} already exists")]
    AlreadyExists(String),

    #[error(transparent)]
    Internal(#[from] eyre::Report),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error("validation error: {0}")]
    Validation(String),
}
