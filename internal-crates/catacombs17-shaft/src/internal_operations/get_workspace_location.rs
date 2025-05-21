use crate::error::OperationalError;
use eyre::OptionExt;
use std::path::PathBuf;

pub trait LocateWorkspaceCargoToml {
    fn locate_workspace_cargo_toml(&self) -> Result<PathBuf, OperationalError>;
}

pub struct GetWorkspaceLocationOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> GetWorkspaceLocationOperation<'a, CL>
where
    CL: LocateWorkspaceCargoToml,
{
    pub fn execute(&self) -> Result<PathBuf, OperationalError> {
        let workspace_cargo_toml_location = self.command_line.locate_workspace_cargo_toml()?;

        let workspace_location = workspace_cargo_toml_location
            .parent()
            .ok_or_eyre(eyre::eyre!(
                "corrupted workspace Cargo.toml location: `{}`",
                workspace_cargo_toml_location.display()
            ))?;

        Ok(workspace_location.to_path_buf())
    }
}
