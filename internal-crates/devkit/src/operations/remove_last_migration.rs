use super::{FileLocation, FolderLocation, OperationalError};
use crate::internal_operations::{GetWorkspaceLocationOperation, LocateWorkspaceCargoToml};
use std::ops::Deref;

pub struct RemoveLastMigrationOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> RemoveLastMigrationOperation<'a, CL>
where
    CL: LocateWorkspaceCargoToml,
{
    pub fn execute(&self, relative_crate_path: &str) -> Result<(), OperationalError> {
        let workspace_location = GetWorkspaceLocationOperation {
            command_line: self.command_line,
        }
        .execute()?;

        let crate_path = FolderLocation::new(
            workspace_location
                .join(relative_crate_path)
                .join("migrations"),
        )?;

        let mut locations = std::fs::read_dir(crate_path.deref())
            .map_err(eyre::Error::new)?
            .map(|entry| {
                let path = entry.map_err(eyre::Error::new)?.path();
                FileLocation::new(path)
            })
            .collect::<Result<Vec<FileLocation>, OperationalError>>()?;

        if locations.is_empty() {
            return Ok(());
        }

        locations.sort_by(|a, b| a.cmp(b));

        // Remove up and down migrations
        for _ in 0..=1 {
            if let Some(location) = locations.pop() {
                std::fs::remove_file(location.deref()).map_err(eyre::Error::new)?;
            }
        }

        Ok(())
    }
}
