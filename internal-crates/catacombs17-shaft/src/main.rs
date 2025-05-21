mod cli;
mod command_line;
mod error;
mod internal_operations;
mod operations;

use clap::Parser;
use cli::{Cli, CliState};
use command_line::CommandLine;

fn main() -> eyre::Result<()> {
    let Cli { command } = Cli::try_parse().map_err(|err| err.exit())?;

    command.execute(CliState {
        command_line: CommandLine::initialize(),
    })?;

    Ok(())
}
