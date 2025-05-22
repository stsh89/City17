use crate::{
    error::OperationalError,
    internal_operations::{GetWorkspaceLocationOperation, LocateWorkspaceCargoToml},
};

use super::{DockerComposeDatabaseEnv, FolderLocation, GetDockerComposeDatabaseEnvOperation};

pub trait RevertMigration {
    fn revert_migration(
        &self,
        database_env: DockerComposeDatabaseEnv,
        crate_path: FolderLocation,
    ) -> Result<(), OperationalError>;
}

pub struct RevertMigrationOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> RevertMigrationOperation<'a, CL>
where
    CL: RevertMigration + LocateWorkspaceCargoToml,
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
            .revert_migration(database_env, crate_path)?;

        Ok(())
    }
}
