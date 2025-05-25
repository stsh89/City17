use crate::{
    command_line::CommandLine,
    operations::{
        CreateAndStartContainersOperation, CreateDatabaseOperation,
        CreateDockerComposePostgresEnvFileOperation, CreateMigrationOperation,
        CreateQueryMetadataOperation, EnterDatabaseCliOperation, GenerateUuidOperation,
        GetDatabaseUrlOperation, GetDockerComposeConfigOperation, InstallSqlxCliOperation,
        MigrationParameters, NewDockerComposeDatabaseEnv, RemoveLastMigrationOperation,
        RevertMigrationOperation, RunMigrationsOperation, StopAndRemoveContainersOperation,
        UuidKind,
    },
};
use clap::{Parser, Subcommand, ValueEnum};

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
    CreateMigration {
        #[arg(long)]
        name: String,

        #[arg(long)]
        crate_path: String,
    },
    CreateQueryMetadata {
        #[arg(long)]
        crate_path: String,
    },
    EnterDatabaseCli,
    GenerateUuid {
        #[arg(long, value_enum)]
        kind: UuidCliKind,
    },
    GetDockerComposeConfig,
    GetDatabaseUrl,
    InstallSqlxCli,
    RemoveLastMigration {
        #[arg(long)]
        crate_path: String,
    },
    RevertMigration {
        #[arg(long)]
        crate_path: String,
    },
    RunMigrations {
        #[arg(long)]
        crate_path: String,
    },
    StopAndRemoveContainers,
}

#[derive(Clone, ValueEnum)]
pub enum UuidCliKind {
    V4,
    V7,
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
            Self::CreateMigration { name, crate_path } => CreateMigrationOperation {
                command_line: &command_line,
            }
            .execute(MigrationParameters {
                migration_name: &name,
                crate_path: &crate_path,
            })?,

            Self::CreateQueryMetadata { crate_path } => CreateQueryMetadataOperation {
                command_line: &command_line,
            }
            .execute(&crate_path)?,
            Self::EnterDatabaseCli => EnterDatabaseCliOperation {
                command_line: &command_line,
            }
            .execute()?,
            Self::GenerateUuid { kind } => {
                let kind = match kind {
                    UuidCliKind::V4 => UuidKind::V4,
                    UuidCliKind::V7 => UuidKind::V7,
                };
                let uuid = GenerateUuidOperation {}.execute(kind);

                println!("{uuid}");
            }
            Self::GetDockerComposeConfig => {
                let config = GetDockerComposeConfigOperation {
                    command_line: &command_line,
                }
                .execute()?;

                println!("{config}");
            }
            Self::GetDatabaseUrl => {
                let database_url = GetDatabaseUrlOperation {
                    command_line: &command_line,
                }
                .execute()?;

                println!("{database_url}");
            }
            Self::InstallSqlxCli => InstallSqlxCliOperation {
                command_line: &command_line,
            }
            .execute()?,
            Self::RemoveLastMigration { crate_path } => RemoveLastMigrationOperation {
                command_line: &command_line,
            }
            .execute(&crate_path)?,
            Self::RevertMigration { crate_path } => RevertMigrationOperation {
                command_line: &command_line,
            }
            .execute(&crate_path)?,
            Self::RunMigrations { crate_path } => RunMigrationsOperation {
                command_line: &command_line,
            }
            .execute(&crate_path)?,
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
