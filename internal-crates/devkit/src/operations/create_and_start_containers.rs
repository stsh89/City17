use super::FileLocation;
use crate::{
    error::OperationalError,
    internal_operations::{GetDockerComposeFileLocationOperation, LocateWorkspaceCargoToml},
};

pub trait CreateAndStartContainers {
    fn create_and_start_containers(
        &self,
        docker_compose_file_location: &FileLocation,
    ) -> Result<(), OperationalError>;
}

pub struct CreateAndStartContainersOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> CreateAndStartContainersOperation<'a, CL>
where
    CL: LocateWorkspaceCargoToml + CreateAndStartContainers,
{
    pub fn execute(&self) -> Result<(), OperationalError> {
        let docker_compose_file_location = GetDockerComposeFileLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        self.command_line
            .create_and_start_containers(&docker_compose_file_location)?;

        Ok(())
    }
}
