use super::{
    DOCKER_COMPOSE_POSTGRES_ENV_FILE_PATH, DockerComposeDatabaseSettings,
    NewDockerComposeDatabaseSettings,
};
use crate::{
    error::OperationalError,
    internal_operations::{GetWorkspaceLocationOperation, LocateWorkspaceCargoToml},
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
        new_settings: NewDockerComposeDatabaseSettings,
    ) -> Result<(), OperationalError> {
        let workspace_location = GetWorkspaceLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        let file_path = workspace_location.join(DOCKER_COMPOSE_POSTGRES_ENV_FILE_PATH);

        if file_path.exists() {
            return Err(OperationalError::AlreadyExists(format!(
                "file `{}`",
                file_path.display()
            )));
        }

        let settings = DockerComposeDatabaseSettings::new(new_settings)?;

        std::fs::write(&file_path, settings.env_string()).map_err(eyre::Error::new)?;

        Ok(())
    }
}
