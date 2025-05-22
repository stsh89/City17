#[derive(Debug, thiserror::Error)]
pub enum OperationalError {
    #[error("{0} already exists")]
    AlreadyExists(String),

    #[error(transparent)]
    Internal(#[from] eyre::Report),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("{0}")]
    Validation(String),
}
