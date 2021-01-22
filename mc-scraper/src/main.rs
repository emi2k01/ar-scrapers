#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

use anyhow::Result;
use clap::Clap;
use diesel::prelude::*;
use dotenv::var;

use downloader::Downloader;
use opts::{DownloaderOpts, DownloaderSubCommand, Opts, SubCommand};

mod anime;
mod downloader;
mod opts;
mod schema;
mod pages;

const DEFAULT_USER_AGENT: &'static str =
    "Mozilla/5.0 (X11; Linux x86_64; rv:83.0) Gecko/20100101 Firefox/83.0";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let opts = Opts::parse();

    let database_url = var("DATABASE_URL")?;
    let db_con = diesel::sqlite::SqliteConnection::establish(&database_url)?;

    match opts.subcmd {
        SubCommand::Download(ref opts) => download_pages(opts, &db_con).await?,
    }

    Ok(())
}

async fn download_pages(opts: &DownloaderOpts, db_con: &SqliteConnection) -> Result<()> {
    let default_user_agent = String::from(DEFAULT_USER_AGENT);
    let user_agent = var("AR_USER_AGENT").unwrap_or(default_user_agent);

    let mut downloader = Downloader::new(&user_agent, opts, db_con);

    match opts.only {
        Some(DownloaderSubCommand::DownloadAnimePages) => downloader.download_anime_pages().await?,
        Some(DownloaderSubCommand::DownloadBrowsePages) => downloader.download_browse_pages().await?,
        None => downloader.download_all().await?,
    }

    Ok(())
}
