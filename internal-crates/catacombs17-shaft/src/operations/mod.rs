mod create_and_start_containers;
mod create_docker_compose_postgres_env_file;
mod install_sqlx_cli;
mod stop_and_remove_containers;

pub use create_and_start_containers::*;
pub use create_docker_compose_postgres_env_file::*;
pub use install_sqlx_cli::*;
pub use stop_and_remove_containers::*;

use crate::error::OperationalError;

const POSTGRES_USER_KEY: &str = "POSTGRES_USER";
const POSTGRES_PASSWORD_KEY: &str = "POSTGRES_PASSWORD";
const POSTGRES_DB_KEY: &str = "POSTGRES_DB";

pub struct DockerComposeDatabaseSettings {
    pub username: String,
    pub password: String,
    pub database_name: String,
}

#[derive(Default)]
pub struct NewDockerComposeDatabaseSettings {
    pub username: String,
    pub password: String,
    pub database_name: String,
}

impl DockerComposeDatabaseSettings {
    fn new(settings: NewDockerComposeDatabaseSettings) -> Result<Self, OperationalError> {
        let NewDockerComposeDatabaseSettings {
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
