use super::{
    DockerComposeDatabaseEnv, FolderLocation, GetDockerComposeDatabaseEnvOperation,
    OperationalError,
};
use crate::internal_operations::{GetWorkspaceLocationOperation, LocateWorkspaceCargoToml};

pub trait CreateQueryMetadata {
    fn create_query_metadata(
        &self,
        crate_path: FolderLocation,
        database_env: DockerComposeDatabaseEnv,
    ) -> Result<(), OperationalError>;
}

pub struct CreateQueryMetadataOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> CreateQueryMetadataOperation<'a, CL>
where
    CL: CreateQueryMetadata + LocateWorkspaceCargoToml,
{
    pub fn execute(&self, relative_crate_path: &str) -> Result<(), OperationalError> {
        let workspace_location = GetWorkspaceLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        let crate_path = FolderLocation::new(workspace_location.join(relative_crate_path))?;

        let database_env = GetDockerComposeDatabaseEnvOperation {
            command_line: self.command_line,
        }
        .execute()?;

        self.command_line
            .create_query_metadata(crate_path, database_env)?;

        Ok(())
    }
}
