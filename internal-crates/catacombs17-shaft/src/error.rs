#[derive(Debug, thiserror::Error)]
pub enum OperationalError {
    #[error("{0} already exists")]
    AlreadyExists(String),

    #[error(transparent)]
    Internal(#[from] eyre::Report),

    #[error("{0}")]
    Validation(String),
}
