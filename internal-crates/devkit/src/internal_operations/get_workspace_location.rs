use crate::{
    error::OperationalError,
    operations::{FileLocation, FolderLocation},
};

pub trait LocateWorkspaceCargoToml {
    fn locate_workspace_cargo_toml(&self) -> Result<FileLocation, OperationalError>;
}

pub struct GetWorkspaceLocationOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> GetWorkspaceLocationOperation<'a, CL>
where
    CL: LocateWorkspaceCargoToml,
{
    pub fn execute(&self) -> Result<FolderLocation, OperationalError> {
        let workspace_cargo_toml_location = self.command_line.locate_workspace_cargo_toml()?;

        Ok(workspace_cargo_toml_location.parent())
    }
}
