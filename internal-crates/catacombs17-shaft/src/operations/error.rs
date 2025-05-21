#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Internal(#[from] eyre::Report),
}
