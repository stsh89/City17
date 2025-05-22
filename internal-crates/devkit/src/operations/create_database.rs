use super::{DockerComposeDatabaseEnv, GetDockerComposeDatabaseEnvOperation};
use crate::{error::OperationalError, internal_operations::LocateWorkspaceCargoToml};

pub trait CreateDatabase {
    fn create_database(
        &self,
        database_env: &DockerComposeDatabaseEnv,
    ) -> Result<(), OperationalError>;
}

pub struct CreateDatabaseOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> CreateDatabaseOperation<'a, CL>
where
    CL: CreateDatabase + LocateWorkspaceCargoToml,
{
    pub fn execute(&self) -> Result<(), OperationalError> {
        let database_env = GetDockerComposeDatabaseEnvOperation {
            command_line: self.command_line,
        }
        .execute()?;

        self.command_line.create_database(&database_env)
    }
}
