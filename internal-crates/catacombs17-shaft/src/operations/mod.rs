mod error;
mod install_sqlx_cli;

pub use error::Error;
pub use install_sqlx_cli::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;
