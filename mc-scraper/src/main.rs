#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

use anyhow::Result;
use clap::Clap;
use dotenv::var;
use sqlx::sqlite::SqlitePool;

use opts::{Opts, SubCommand};

mod anime;
mod db;
mod opts;
mod page_checksum;
mod scraper;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let database_url = var("DATABASE_URL")?;
    db::DB
        .set(SqlitePool::connect(&database_url).await?)
        .unwrap();

    let opts = Opts::parse();

    match opts.subcmd {
        SubCommand::Scrape => scrape().await?,
    }

    Ok(())
}

async fn scrape() -> Result<()> {
    scraper::scrape().await?;

    Ok(())
}
