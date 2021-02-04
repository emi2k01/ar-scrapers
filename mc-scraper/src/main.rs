#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

use anyhow::Result;
use clap::Clap;
use opts::{Opts, SubCmd};
use sqlx::SqlitePool;

mod db;
mod fetchers;
mod models;
mod scraper;
mod opts;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env("LOG").init();

    let opts = Opts::parse();

    let database_url = dotenv::var("DATABASE_URL")?;
    db::DB
        .set(SqlitePool::connect(&database_url).await?)
        .unwrap();

    sqlx::migrate!("./migrations")
        .run(db::DB.get().unwrap())
        .await?;

    match opts.subcmd {
        SubCmd::Scrape => scraper::scrape().await?,
        SubCmd::Render(opts) => {
            
        }
    }

    Ok(())
}
