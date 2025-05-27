use super::{FileLocation, OperationalError};
use crate::internal_operations::{GetDockerComposeFileLocationOperation, LocateWorkspaceCargoToml};

pub trait GetDockerComposeConfig {
    fn get_docker_compose_config(
        &self,
        docker_compose_file_location: &FileLocation,
    ) -> Result<String, OperationalError>;
}

pub struct GetDockerComposeConfigOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> GetDockerComposeConfigOperation<'a, CL>
where
    CL: GetDockerComposeConfig + LocateWorkspaceCargoToml,
{
    pub fn execute(&self) -> Result<String, OperationalError> {
        let docker_compose_file_location = GetDockerComposeFileLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        self.command_line
            .get_docker_compose_config(&docker_compose_file_location)
    }
}
