use crate::{
    command_line::CommandLine,
    operations::{
        CreateAndStartContainersOperation, CreateDatabaseOperation,
        CreateDockerComposePostgresEnvFileOperation, GetDockerComposeConfigOperation,
        InstallSqlxCliOperation, NewDockerComposeDatabaseEnv, StopAndRemoveContainersOperation,
    },
};
use clap::{Parser, Subcommand};

const POSTGRES_USER: &str = "gordon";
const POSTGRES_DB: &str = "city17_dev";

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
    CreateAndStartContainers,
    CreateDatabase,
    CreateDockerComposePostgresEnvFile,
    GetDockerComposeConfig,
    InstallSqlxCli,
    StopAndRemoveContainers,
}

impl Commands {
    pub fn execute(self, cli_state: CliState) -> eyre::Result<()> {
        let CliState { command_line } = cli_state;

        match self {
            Self::CreateAndStartContainers => CreateAndStartContainersOperation {
                command_line: &command_line,
            }
            .execute()?,
            Self::CreateDatabase => CreateDatabaseOperation {
                command_line: &command_line,
            }
            .execute()?,
            Self::CreateDockerComposePostgresEnvFile => {
                CreateDockerComposePostgresEnvFileOperation {
                    command_line: &command_line,
                }
                .execute(NewDockerComposeDatabaseEnv {
                    username: POSTGRES_USER.to_string(),
                    password: generate_password(16),
                    database_name: POSTGRES_DB.to_string(),
                })?
            }
            Self::GetDockerComposeConfig => {
                let config = GetDockerComposeConfigOperation {
                    command_line: &command_line,
                }
                .execute()?;

                println!("{config}");
            }
            Self::InstallSqlxCli => InstallSqlxCliOperation {
                command_line: &command_line,
            }
            .execute()?,
            Self::StopAndRemoveContainers => StopAndRemoveContainersOperation {
                command_line: &command_line,
            }
            .execute()?,
        };

        Ok(())
    }
}

fn generate_password(length: usize) -> String {
    use rand::{Rng, distr::Alphanumeric};

    let rng = rand::rng();

    let password: String = rng
        .sample_iter(Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    password
}
