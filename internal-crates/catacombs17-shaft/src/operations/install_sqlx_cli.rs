use super::Result;

pub trait InstallSqlxCli {
    fn install_sqlx_cli(&self) -> Result<()>;
}

pub struct InstallSqlxCliOperation<'a, CL> {
    pub command_line: &'a CL,
}

impl<'a, CL> InstallSqlxCliOperation<'a, CL>
where
    CL: InstallSqlxCli,
{
    pub fn execute(&self) -> Result<()> {
        self.command_line.install_sqlx_cli()?;

        Ok(())
    }
}
