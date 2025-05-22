use super::DOCKER_DIRECTORY_NAME;
use crate::{
    error::OperationalError,
    internal_operations::{GetWorkspaceLocationOperation, LocateWorkspaceCargoToml},
};
use std::path::Path;

pub trait StopAndRemoveContainers {
    fn stop_and_remove_containers(
        &self,
        docker_compose_file_parent_location: &Path,
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
        let workspace_location = GetWorkspaceLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        let docker_compose_file_parent_location = workspace_location.join(DOCKER_DIRECTORY_NAME);

        self.command_line
            .stop_and_remove_containers(docker_compose_file_parent_location.as_path())
    }
}
