use super::{DockerComposeDatabaseEnv, FolderLocation, GetDockerComposeDatabaseEnvOperation};
use crate::{
    error::OperationalError,
    internal_operations::{GetWorkspaceLocationOperation, LocateWorkspaceCargoToml},
};

pub trait RunMigrations {
    fn run_migrations(
        &self,
        database_env: DockerComposeDatabaseEnv,
        crate_path: FolderLocation,
    ) -> Result<(), OperationalError>;
}

pub struct RunMigrationsOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> RunMigrationsOperation<'a, CL>
where
    CL: RunMigrations + LocateWorkspaceCargoToml,
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

        self.command_line.run_migrations(database_env, crate_path)?;

        Ok(())
    }
}
