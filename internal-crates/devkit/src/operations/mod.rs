mod create_and_start_containers;
mod create_database;
mod create_docker_compose_postgres_env_file;
mod create_migration;
mod enter_database_cli;
mod get_docker_compose_config;
mod get_docker_compose_datbase_env;
mod install_sqlx_cli;
mod remove_last_migration;
mod revert_migration;
mod run_migrations;
mod stop_and_remove_containers;

pub use create_and_start_containers::*;
pub use create_database::*;
pub use create_docker_compose_postgres_env_file::*;
pub use create_migration::*;
pub use enter_database_cli::*;
pub use get_docker_compose_config::*;
pub use get_docker_compose_datbase_env::*;
pub use install_sqlx_cli::*;
pub use remove_last_migration::*;
pub use revert_migration::*;
pub use run_migrations::*;
pub use stop_and_remove_containers::*;

use crate::error::OperationalError;
use std::path::{Path, PathBuf};

const POSTGRES_USER_KEY: &str = "POSTGRES_USER";
const POSTGRES_PASSWORD_KEY: &str = "POSTGRES_PASSWORD";
const POSTGRES_DB_KEY: &str = "POSTGRES_DB";

pub struct FolderLocation(PathBuf);
pub struct FileLocation(PathBuf);

impl FileLocation {
    pub fn from_str(path: &str) -> Result<Self, OperationalError> {
        Self::new(path.into())
    }

    pub fn new(path: PathBuf) -> Result<Self, OperationalError> {
        if !path.is_file() {
            return Err(OperationalError::InvalidArgument(format!(
                "`{}` is not a file path",
                path.display()
            )));
        }

        Ok(FileLocation(path))
    }

    pub fn parent(&self) -> FolderLocation {
        FolderLocation(self.0.parent().unwrap().to_path_buf())
    }
}

impl FolderLocation {
    pub fn new(path: PathBuf) -> Result<Self, OperationalError> {
        if !path.is_dir() {
            return Err(OperationalError::InvalidArgument(format!(
                "`{}` is not a directory path",
                path.display()
            )));
        }

        Ok(FolderLocation(path))
    }
}

pub struct DockerComposeDatabaseEnv {
    pub username: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Default)]
pub struct NewDockerComposeDatabaseEnv {
    pub username: String,
    pub password: String,
    pub database_name: String,
}

impl DockerComposeDatabaseEnv {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@localhost:5432/{}",
            self.username, self.password, self.database_name
        )
    }

    fn new(settings: NewDockerComposeDatabaseEnv) -> Result<Self, OperationalError> {
        let NewDockerComposeDatabaseEnv {
            username,
            password,
            database_name,
        } = settings;

        if username.is_empty() {
            return Err(OperationalError::Validation(
                "PostgreSQL username cannot be empty".to_string(),
            ));
        }

        if password.is_empty() {
            return Err(OperationalError::Validation(
                "PostgreSQL password cannot be empty".to_string(),
            ));
        }

        if database_name.is_empty() {
            return Err(OperationalError::Validation(
                "PostgreSQL database name cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            username,
            password,
            database_name,
        })
    }

    fn env_string(&self) -> String {
        format!(
            "{}={}\n{}={}\n{}={}",
            POSTGRES_USER_KEY,
            self.username,
            POSTGRES_PASSWORD_KEY,
            self.password,
            POSTGRES_DB_KEY,
            self.database_name
        )
    }
}

impl std::ops::Deref for FileLocation {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::Deref for FolderLocation {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
