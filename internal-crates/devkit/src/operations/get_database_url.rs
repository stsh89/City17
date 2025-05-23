use super::GetDockerComposeDatabaseEnvOperation;
use crate::{error::OperationalError, internal_operations::LocateWorkspaceCargoToml};

pub struct GetDatabaseUrlOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> GetDatabaseUrlOperation<'a, CL>
where
    CL: LocateWorkspaceCargoToml,
{
    pub fn execute(&self) -> Result<String, OperationalError> {
        let database_env = GetDockerComposeDatabaseEnvOperation {
            command_line: self.command_line,
        }
        .execute()?;

        Ok(database_env.database_url())
    }
}
