use super::{FolderLocation, OperationalError};
use crate::internal_operations::{GetWorkspaceLocationOperation, LocateWorkspaceCargoToml};

pub trait CreateMigration {
    fn create_migration(
        &self,
        parameters: CreateMigrationParameters,
    ) -> Result<(), OperationalError>;
}

pub struct CreateMigrationParameters<'a> {
    pub migration_name: &'a str,
    pub crate_path: FolderLocation,
}

pub struct CreateMigrationOperation<'a, CL> {
    pub command_line: &'a CL,
}

pub struct MigrationParameters<'a> {
    pub migration_name: &'a str,
    pub crate_path: &'a str,
}

impl<'a, CL> CreateMigrationOperation<'a, CL>
where
    CL: CreateMigration + LocateWorkspaceCargoToml,
{
    pub fn execute(&self, parameters: MigrationParameters) -> Result<(), OperationalError> {
        let MigrationParameters {
            migration_name,
            crate_path,
        } = parameters;

        let workspace_location = GetWorkspaceLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        self.command_line
            .create_migration(CreateMigrationParameters {
                migration_name,
                crate_path: FolderLocation::new(workspace_location.join(crate_path))?,
            })?;

        Ok(())
    }
}
