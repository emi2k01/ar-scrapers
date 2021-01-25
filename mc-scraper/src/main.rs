#[macro_use]
extern crate log;

#[macro_use]
extern crate anyhow;

use anyhow::Result;
use models::PageChecksum;
use scrap::Html;
use sqlx::SqlitePool;

mod db;
mod fetchers;
mod models;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let database_url = dotenv::var("DATABASE_URL")?;
    db::DB
        .set(SqlitePool::connect(&database_url).await?)
        .unwrap();

    Ok(())
}

async fn scrape() -> Result<()> {
    let browse_pages = fetchers::fetch_browse_pages().await;

    for (browse_url, browse_body) in browse_pages {
        let browse_doc = Html::parse_document(&browse_body);

        let animes_pages = fetchers::fetch_pages_from_anchors(&browse_doc, "a.link-anime").await;

        for (anime_url, anime_body) in animes_pages {
            let anime_page_checksum = PageChecksum::from_page(anime_url, &anime_body);
            if anime_page_checksum.is_new().await? {
                anime_page_checksum.insert().await?;
            } else {
                todo!("DETECT NEW EPISODES AND ADD THEM");
            }
        }
    }

    todo!()
}
