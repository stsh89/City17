use crate::{command_line::CommandLine, operations::InstallSqlxCliOperation};
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub struct CliState {
    pub command_line: CommandLine,
}

#[derive(Subcommand)]
pub enum Commands {
    InstallSqlxCli,
}

impl Commands {
    pub fn execute(self, cli_state: CliState) -> eyre::Result<()> {
        let CliState { command_line } = cli_state;

        match self {
            Self::InstallSqlxCli => InstallSqlxCliOperation {
                command_line: &command_line,
            }
            .execute()?,
        };

        Ok(())
    }
}
