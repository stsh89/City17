#[derive(Debug, thiserror::Error)]
pub enum OperationalError {
    #[error(transparent)]
    Internal(#[from] eyre::Report),

    #[error("invalid argument: {0}")]
    InvalidArgument(String),

    #[error(transparent)]
    Repo(#[from] sqlx::Error),

    #[error("validation error: {0}")]
    Validation(String),
}
