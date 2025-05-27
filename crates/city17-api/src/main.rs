use clap::Parser;
use repo::Repo;

mod api;
mod operations;
mod repo;
mod tracer;

#[macro_use]
extern crate rocket;

const DEFAULT_PORT: u16 = 8000;

struct AppState {
    repo: Repo,
}

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    pub database_url: String,

    #[arg(long)]
    pub port: Option<u16>,
}

#[rocket::main]
async fn main() -> eyre::Result<()> {
    let _guard = tracer::init_subscriber();

    Cli::try_parse()
        .map_err(|err| err.exit())?
        .execute()
        .await?;

    Ok(())
}

impl Cli {
    pub async fn execute(self) -> eyre::Result<()> {
        let Self { database_url, port } = self;

        let port = port.unwrap_or(DEFAULT_PORT);
        let pool = sqlx::PgPool::connect(&database_url).await?;
        let repo = Repo::new(pool);
        let state = AppState { repo };

        let _rocket = rocket::build()
            .mount("/symbols", routes![api::symbols::index])
            .manage(state)
            .configure(rocket::Config::figment().merge(("port", port)))
            .launch()
            .await?;

        todo!()
    }
}
