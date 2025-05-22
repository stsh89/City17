use super::{
    DockerComposeDatabaseEnv, NewDockerComposeDatabaseEnv, POSTGRES_DB_KEY, POSTGRES_PASSWORD_KEY,
    POSTGRES_USER_KEY,
};
use crate::{
    error::OperationalError,
    internal_operations::{
        DOCKER_COMPOSE_POSTGRES_ENV_FILE_NAME, GetDockerComposeFileLocationOperation,
        LocateWorkspaceCargoToml,
    },
};

pub struct GetDockerComposeDatabaseEnvOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> GetDockerComposeDatabaseEnvOperation<'a, CL>
where
    CL: LocateWorkspaceCargoToml,
{
    pub fn execute(&self) -> Result<DockerComposeDatabaseEnv, OperationalError> {
        let docker_compose_file_location = GetDockerComposeFileLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        let docker_compose_directory = docker_compose_file_location.parent();

        let postgres_env_file_path =
            docker_compose_directory.join(DOCKER_COMPOSE_POSTGRES_ENV_FILE_NAME);

        let mut new_database_env = NewDockerComposeDatabaseEnv::default();

        for line in std::fs::read_to_string(&postgres_env_file_path)
            .map_err(eyre::Error::new)?
            .lines()
        {
            let (key, value) = line.split_once('=').ok_or_else(|| {
                eyre::eyre!(
                    "corrupted docker compose postgres env file: `{}`",
                    postgres_env_file_path.display()
                )
            })?;

            match key {
                POSTGRES_USER_KEY => new_database_env.username = value.to_string(),
                POSTGRES_PASSWORD_KEY => new_database_env.password = value.to_string(),
                POSTGRES_DB_KEY => new_database_env.database_name = value.to_string(),
                _ => {}
            }
        }

        DockerComposeDatabaseEnv::new(new_database_env)
    }
}
