use crate::error::OperationalError;
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

        match workspace_cargo_toml_location.parent() {
            Some(location) => Ok(location.to_path_buf()),
            None => Err(eyre::eyre!(
                "corrupted workspace Cargo.toml location: `{}`",
                workspace_cargo_toml_location.display()
            )
            .into()),
        }
    }
}
