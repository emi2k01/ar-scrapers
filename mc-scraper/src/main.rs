#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

use anyhow::Result;
use fetchers::fetch_pages_from_anchors;
use futures::{stream, StreamExt};
use models::{Anime, AnimeUrl, Episode, EpisodeUrl, PageChecksum};
use scrap::{Html, Selector};
use sqlx::SqlitePool;

mod db;
mod fetchers;
mod models;
mod scraper;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env("LOG").init();

    let database_url = dotenv::var("DATABASE_URL")?;
    db::DB
        .set(SqlitePool::connect(&database_url).await?)
        .unwrap();

    scrape().await?;

    Ok(())
}
