mod create_and_start_containers;
mod create_database;
mod create_docker_compose_postgres_env_file;
mod get_docker_compose_config;
mod get_docker_compose_datbase_env;
mod install_sqlx_cli;
mod stop_and_remove_containers;

pub use create_and_start_containers::*;
pub use create_database::*;
pub use create_docker_compose_postgres_env_file::*;
pub use get_docker_compose_config::*;
pub use get_docker_compose_datbase_env::*;
pub use install_sqlx_cli::*;
pub use stop_and_remove_containers::*;

use crate::error::OperationalError;

const POSTGRES_USER_KEY: &str = "POSTGRES_USER";
const POSTGRES_PASSWORD_KEY: &str = "POSTGRES_PASSWORD";
const POSTGRES_DB_KEY: &str = "POSTGRES_DB";

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
