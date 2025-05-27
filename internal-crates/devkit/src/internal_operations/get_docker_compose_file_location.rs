use super::{
    DOCKER_COMPOSE_FILE_NAME, DOCKER_DIRECTORY_NAME, GetWorkspaceLocationOperation,
    LocateWorkspaceCargoToml,
};
use crate::operations::{FileLocation, OperationalError};

pub struct GetDockerComposeFileLocationOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> GetDockerComposeFileLocationOperation<'a, CL>
where
    CL: LocateWorkspaceCargoToml,
{
    pub fn execute(&self) -> Result<FileLocation, OperationalError> {
        let workspace_location = GetWorkspaceLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        let docker_compose_file_location = workspace_location
            .join(DOCKER_DIRECTORY_NAME)
            .join(DOCKER_COMPOSE_FILE_NAME);

        FileLocation::new(docker_compose_file_location)
    }
}
