use crate::{
    error::OperationalError,
    internal_operations::LocateWorkspaceCargoToml,
    operations::{
        CreateAndStartContainers, CreateDatabase, CreateMigration, CreateMigrationParameters,
        DockerComposeDatabaseEnv, FileLocation, FolderLocation, GetDockerComposeConfig,
        InstallSqlxCli, RunMigrations, StopAndRemoveContainers,
    },
};
use eyre::Context;
use std::{ops::Deref, path::Path, process::Command};

const CARGO_PROGRAM: &str = "cargo";
const DOCKER_PROGRAM: &str = "docker";
const SQLX_PROGRAM: &str = "sqlx";

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
        docker_compose_file_location: &FileLocation,
    ) -> Result<(), OperationalError> {
        let error_message = "failed to create and start containers";

        let status = Command::new(DOCKER_PROGRAM)
            .args(["compose", "up", "-d"])
            .current_dir(docker_compose_file_location.parent().deref())
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

impl CreateDatabase for CommandLine {
    /// Execute `sqlx database create` command.
    fn create_database(
        &self,
        database_env: &DockerComposeDatabaseEnv,
    ) -> Result<(), OperationalError> {
        let error_message = format!(
            "failed to create database, try running `sqlx database create --database-url {}`",
            database_env.database_url(),
        );

        let status = Command::new(SQLX_PROGRAM)
            .args([
                "database",
                "create",
                "--database-url",
                &database_env.database_url(),
            ])
            .status()
            .map_err(eyre::Error::new)
            .wrap_err_with(|| error_message.clone())?;

        if status.success() {
            Ok(())
        } else {
            Err(eyre::eyre!(error_message).into())
        }
    }
}

impl CreateMigration for CommandLine {
    /// Execute `sqlx migrate add` command.
    fn create_migration(
        &self,
        create_migration_parameters: CreateMigrationParameters,
    ) -> Result<(), OperationalError> {
        let CreateMigrationParameters {
            migration_name,
            crate_path,
        } = create_migration_parameters;

        let error_message = format!(
            "failed to create migration, try running `sqlx migrate add -r {}`",
            migration_name,
        );

        let status = Command::new(SQLX_PROGRAM)
            .args(["migrate", "add", "-r", &migration_name])
            .current_dir(crate_path.deref())
            .status()
            .map_err(eyre::Error::new)
            .wrap_err_with(|| error_message.clone())?;

        if status.success() {
            Ok(())
        } else {
            Err(eyre::eyre!(error_message).into())
        }
    }
}

impl GetDockerComposeConfig for CommandLine {
    /// Execute `docker compose config` command.
    fn get_docker_compose_config(
        &self,
        docker_compose_file_location: &FileLocation,
    ) -> Result<String, OperationalError> {
        let error_message = "failed to get docker compose config";

        let output = Command::new(DOCKER_PROGRAM)
            .args(["compose", "config"])
            .current_dir(docker_compose_file_location.parent().deref())
            .output()
            .map_err(eyre::Error::new)
            .wrap_err_with(|| error_message)?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
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
    fn locate_workspace_cargo_toml(&self) -> Result<FileLocation, OperationalError> {
        let error_message = "failed to locate workspace Cargo.toml";

        let output = Command::new(CARGO_PROGRAM)
            .args(["locate-project", "--workspace", "--message-format", "plain"])
            .output()
            .map_err(eyre::Error::new)
            .wrap_err_with(|| error_message)?;

        if output.status.success() {
            let location = String::from_utf8_lossy(&output.stdout).to_string();

            FileLocation::from_str(location.trim())
        } else {
            Err(eyre::eyre!(error_message).into())
        }
    }
}

impl RunMigrations for CommandLine {
    /// Execute `sqlx migrate run` command.
    fn run_migrations(
        &self,
        database_env: DockerComposeDatabaseEnv,
        crate_path: FolderLocation,
    ) -> Result<(), OperationalError> {
        let error_message = format!(
            "failed to run migrations, try running `sqlx migrate run --database-url {}`",
            database_env.database_url()
        );

        let status = Command::new(SQLX_PROGRAM)
            .args([
                "migrate",
                "run",
                "--database-url",
                &database_env.database_url(),
            ])
            .current_dir(crate_path.deref())
            .status()
            .map_err(eyre::Error::new)
            .wrap_err_with(|| error_message.clone())?;

        if status.success() {
            Ok(())
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
