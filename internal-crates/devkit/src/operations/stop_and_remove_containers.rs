use crate::{
    error::OperationalError,
    internal_operations::{GetDockerComposeFileLocationOperation, LocateWorkspaceCargoToml},
};
use std::path::Path;

pub trait StopAndRemoveContainers {
    fn stop_and_remove_containers(
        &self,
        docker_compose_file_location: &Path,
    ) -> Result<(), OperationalError>;
}

pub struct StopAndRemoveContainersOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> StopAndRemoveContainersOperation<'a, CL>
where
    CL: StopAndRemoveContainers + LocateWorkspaceCargoToml,
{
    pub fn execute(&self) -> Result<(), OperationalError> {
        let docker_compose_file_location = GetDockerComposeFileLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        self.command_line
            .stop_and_remove_containers(&docker_compose_file_location)
    }
}
