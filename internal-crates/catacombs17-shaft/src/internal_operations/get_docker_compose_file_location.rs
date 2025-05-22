use crate::error::OperationalError;
use std::path::PathBuf;

use super::{
    DOCKER_COMPOSE_FILE_NAME, DOCKER_DIRECTORY_NAME, GetWorkspaceLocationOperation,
    LocateWorkspaceCargoToml,
};

pub struct GetDockerComposeFileLocationOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> GetDockerComposeFileLocationOperation<'a, CL>
where
    CL: LocateWorkspaceCargoToml,
{
    pub fn execute(&self) -> Result<PathBuf, OperationalError> {
        let workspace_location = GetWorkspaceLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        let docker_compose_file_location = workspace_location
            .join(DOCKER_DIRECTORY_NAME)
            .join(DOCKER_COMPOSE_FILE_NAME);

        Ok(docker_compose_file_location)
    }
}
