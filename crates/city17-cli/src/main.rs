use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
}

fn main() -> eyre::Result<()> {
    let Cli { command } = Cli::try_parse().map_err(|err| err.exit()).unwrap();

    match command {
        Commands::List => println!("{{}}"),
    }

    Ok(())
}
