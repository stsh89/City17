use crate::operations::{InstallSqlxCli, Result};
use std::process::Command;

const CARGO_PROGRAM: &str = "cargo";

pub struct CommandLine;

impl CommandLine {
    pub fn initialize() -> Self {
        CommandLine {}
    }
}

impl InstallSqlxCli for CommandLine {
    /// Execute `cargo install sqlx-cli` command.
    fn install_sqlx_cli(&self) -> Result<()> {
        let status = Command::new(CARGO_PROGRAM)
            .args(["install", "sqlx-cli"])
            .status()
            .map_err(eyre::Error::new);

        match status {
            Ok(status) if status.success() => Ok(()),
            _ => Err(eyre::eyre!(
                "failed to install sqlx-cli, try running `cargo install sqlx-cli`"
            )
            .into()),
        }
    }
}
