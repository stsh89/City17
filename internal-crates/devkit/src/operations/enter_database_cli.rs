use super::{
    DockerComposeDatabaseEnv, FileLocation, GetDockerComposeDatabaseEnvOperation, OperationalError,
};
use crate::internal_operations::{GetDockerComposeFileLocationOperation, LocateWorkspaceCargoToml};

pub trait EnterDatabaseCli {
    fn enter_database_cli(
        &self,
        docker_compose_file_location: FileLocation,
        database_env: DockerComposeDatabaseEnv,
    ) -> Result<(), OperationalError>;
}

pub struct EnterDatabaseCliOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> EnterDatabaseCliOperation<'a, CL>
where
    CL: EnterDatabaseCli + LocateWorkspaceCargoToml,
{
    pub fn execute(&self) -> Result<(), OperationalError> {
        let docker_compose_file_location = GetDockerComposeFileLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        let database_env = GetDockerComposeDatabaseEnvOperation {
            command_line: self.command_line,
        }
        .execute()?;

        self.command_line
            .enter_database_cli(docker_compose_file_location, database_env)
    }
}
