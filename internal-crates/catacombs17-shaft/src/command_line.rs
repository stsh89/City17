use crate::{
    error::OperationalError,
    internal_operations::LocateWorkspaceCargoToml,
    operations::{CreateAndStartContainers, InstallSqlxCli, StopAndRemoveContainers},
};
use eyre::Context;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

const CARGO_PROGRAM: &str = "cargo";

pub struct CommandLine;

impl CommandLine {
    pub fn initialize() -> Self {
        CommandLine {}
    }
}

impl CreateAndStartContainers for CommandLine {
    /// Execute `docker compose up -d` command.
    fn create_and_start_containers(
        &self,
        docker_compose_file_location: &Path,
    ) -> Result<(), OperationalError> {
        let error_message = "failed to create and start containers";

        let status = Command::new("docker")
            .args(["compose", "up", "-d"])
            .current_dir(docker_compose_file_location.parent().unwrap())
            .status()
            .map_err(eyre::Error::new)
            .wrap_err_with(|| error_message)?;

        if status.success() {
            Ok(())
        } else {
            Err(eyre::eyre!(error_message).into())
        }
    }
}

impl InstallSqlxCli for CommandLine {
    /// Execute `cargo install sqlx-cli` command.
    fn install_sqlx_cli(&self) -> Result<(), OperationalError> {
        let error_message = "failed to install sqlx-cli, try running `cargo install sqlx-cli`";

        let status = Command::new(CARGO_PROGRAM)
            .args(["install", "sqlx-cli"])
            .status()
            .map_err(eyre::Error::new)
            .wrap_err_with(|| error_message)?;

        if status.success() {
            Ok(())
        } else {
            Err(eyre::eyre!(error_message).into())
        }
    }
}

impl LocateWorkspaceCargoToml for CommandLine {
    /// Execute `cargo locate-project --workspace --message-format plain` command.
    fn locate_workspace_cargo_toml(&self) -> Result<PathBuf, OperationalError> {
        let error_message = "failed to locate workspace Cargo.toml";

        let output = Command::new(CARGO_PROGRAM)
            .args(["locate-project", "--workspace", "--message-format", "plain"])
            .output()
            .map_err(eyre::Error::new)
            .wrap_err_with(|| error_message)?;

        if output.status.success() {
            let location = String::from_utf8_lossy(&output.stdout).to_string();

            Ok(Path::new(&location).to_path_buf())
        } else {
            Err(eyre::eyre!(error_message).into())
        }
    }
}

impl StopAndRemoveContainers for CommandLine {
    fn stop_and_remove_containers(
        &self,
        docker_compose_file_location: &Path,
    ) -> Result<(), OperationalError> {
        let error_message = "failed to stop and remove containers";

        let docker_compose_directory = docker_compose_file_location
            .parent()
            .ok_or_else(|| {
                eyre::eyre!(
                    "corrupted docker compose file location: `{}`",
                    docker_compose_file_location.display()
                )
            })?
            .to_path_buf();

        let status = Command::new("docker")
            .args(["compose", "down", "--volumes"])
            .current_dir(docker_compose_directory)
            .status()
            .map_err(eyre::Error::new)
            .wrap_err_with(|| error_message)?;

        if status.success() {
            Ok(())
        } else {
            Err(eyre::eyre!(error_message).into())
        }
    }
}
