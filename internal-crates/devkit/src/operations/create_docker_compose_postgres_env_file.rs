use super::{DockerComposeDatabaseEnv, NewDockerComposeDatabaseEnv, OperationalError};
use crate::internal_operations::{
    DOCKER_COMPOSE_POSTGRES_ENV_FILE_NAME, GetDockerComposeFileLocationOperation,
    LocateWorkspaceCargoToml,
};

pub struct CreateDockerComposePostgresEnvFileOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> CreateDockerComposePostgresEnvFileOperation<'a, CL>
where
    CL: LocateWorkspaceCargoToml,
{
    pub fn execute(
        &self,
        new_settings: NewDockerComposeDatabaseEnv,
    ) -> Result<(), OperationalError> {
        let docker_compose_file_location = GetDockerComposeFileLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        let docker_compose_directory = docker_compose_file_location.parent();

        let postgres_env_file_path =
            docker_compose_directory.join(DOCKER_COMPOSE_POSTGRES_ENV_FILE_NAME);

        if postgres_env_file_path.exists() {
            return Err(OperationalError::AlreadyExists(format!(
                "file `{}`",
                postgres_env_file_path.display()
            )));
        }

        let settings = DockerComposeDatabaseEnv::new(new_settings)?;

        std::fs::write(&postgres_env_file_path, settings.env_string()).map_err(eyre::Error::new)?;

        Ok(())
    }
}
