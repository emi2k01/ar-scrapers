use std::sync::Arc;

use anyhow::{Context, Result};
use diesel::{prelude::*, SqliteConnection};
use futures::stream::{self, StreamExt};
use reqwest::{Client, ClientBuilder};
use scraper::{Html, Selector};
use url::Url;

use crate::opts::DownloaderOpts;

const BROWSE_URL: &'static str = "https://monoschinos2.com/animes";
const BROWSE_RELATIVE_PATH: &'static str = "browse/";
const ANIMES_RELATIVE_PATH: &'static str = "animes/";

pub struct Downloader<'a> {
    client: Arc<Client>,
    opts: &'a DownloaderOpts,
    db_con: &'a SqliteConnection,
}

impl<'a> Downloader<'a> {
    pub fn new(user_agent: &str, opts: &'a DownloaderOpts, db_con: &'a SqliteConnection) -> Self {
        let client = ClientBuilder::new().user_agent(user_agent).build().unwrap();

        Self {
            client: Arc::new(client),
            opts,
            db_con,
        }
    }

    pub async fn download_all(&mut self) -> Result<()> {
        self.download_browse_pages().await?;

        Ok(())
    }

    pub async fn download_anime_pages(&self) -> Result<()> {
        info!("downloading all anime pages");

        let mut dir = tokio::fs::read_dir(&self.opts.out.join(BROWSE_RELATIVE_PATH)).await?;

        // Read html files in `browse/` directory and parse them
        let mut browse_pages = Vec::with_capacity(150);
        while let Some(entry) = dir.next_entry().await? {
            if let Some(ext) = entry.path().extension() {
                let ext = ext.to_str().ok_or(format_err!("not a utf8 extension"))?;
                if ext == "html" {
                    debug!("Reading {}", entry.path().to_str().unwrap());
                    let page_string = tokio::fs::read_to_string(&entry.path()).await?;
                    let page = Html::parse_document(&page_string);
                    browse_pages.push(page);
                }
            }
        }

        // 150 pages with 30 animes each one
        let mut animes = Vec::with_capacity(150 * 30);

        let anime_link_sel = Selector::parse(".link-anime")
            .map_err(|e| format_err!("could not parse css selector: {:?}", e))?;
        let anime_name_sel = Selector::parse("h3.Title")
            .map_err(|e| format_err!("could not parse css selector: {:?}", e))?;

        // Read all anime names and urls into `animes`
        for doc in &browse_pages {
            let anime_anchors = doc.select(&anime_link_sel);

            for anime_anchor in anime_anchors {
                let anime_name = anime_anchor
                    .select(&anime_name_sel)
                    .next()
                    .unwrap()
                    .text()
                    .next()
                    .unwrap()
                    .to_owned();
                let anime_url = anime_anchor.value().attr("href").unwrap().to_owned();

                animes.push((anime_name, anime_url));
            }
        }

        let animes_dir = self.opts.out.join("animes/");
        debug!(
            "creating $out/animes folder it it doesn't exist: {}",
            animes_dir.to_str().unwrap()
        );
        tokio::fs::create_dir_all(&animes_dir).await.unwrap();

        let mut futures_buffer = stream::iter(animes)
            // Map iterator's `Item` to a future
            .map(|(name, url)| {
                let client = Arc::clone(&self.client);

                let animes_dir = animes_dir.clone();

                // Run future parallelly
                tokio::spawn(async move {
                    debug!("requesting anime [{}] from [{}]", name, url);
                    let res = client.get(&url).send().await;

                    match res {
                        Ok(res) => {
                            let anime_name =
                                format!("{name}.html", name = name).replace('/', "---");
                            let anime_path = animes_dir.join(anime_name);

                            debug!("writing anime page to {}", anime_path.to_str().unwrap());
                            tokio::fs::write(&anime_path, res.text().await.unwrap())
                                .await
                                .with_context(|| {
                                    let error = format_err!(
                                        "error while saving file {}",
                                        anime_path.to_str().unwrap()
                                    );
                                    error!("{}", error);
                                    error
                                })
                                .unwrap();
                        }
                        Err(e) => error!(
                            "request error on anime [{}] with url [{}]\nCause: {}",
                            name, url, e
                        ),
                    }
                })
            })
            // Convert stream to `BufferUnordered` to poll at most 4 futures at the same time
            .buffer_unordered(4);

        // Poll futures
        loop {
            let res = futures_buffer.next().await;
            if res.is_none() {
                break;
            }
        }

        Ok(())
    }

    pub async fn download_browse_pages(&self) -> Result<()> {
        let num_of_pages = self.number_of_browse_pages().await?;

        info!("downloading {} browse pages", num_of_pages);

        // Make a Vec of urls
        let mut urls = Vec::with_capacity(num_of_pages as usize);
        for i in 1..=num_of_pages {
            let ith_page_url = Url::parse_with_params(BROWSE_URL, vec![("page", i.to_string())])?;
            urls.push(ith_page_url.to_string());
        }

        let browse_dir = self.opts.out.join(BROWSE_RELATIVE_PATH);
        debug!(
            "creating $out/browse folder it it doesn't exist: {}",
            browse_dir.to_str().unwrap()
        );
        tokio::fs::create_dir_all(&browse_dir).await?;

        // Convert `urls` to a stream
        let mut futures_buffer = stream::iter(urls.into_iter().enumerate())
            // Map the `Item` of the stream to a future
            .map(move |(page_num, url)| {
                // Add 1 because `Enumerate` iterator starts at 0
                let page_num = page_num as u32 + 1;

                let client = Arc::clone(&self.client);

                // Clone it for the future
                let browse_dir = browse_dir.clone();

                // Run future parallely
                tokio::spawn(async move {
                    debug!("requesting browse page {} [{}]", page_num, url);
                    let res = client.get(&url).send().await;

                    match res {
                        Ok(res) => {
                            let html = res.text().await.unwrap();

                            let browse_page_path = browse_dir
                                .join(format!("{page_num}.html", page_num = page_num.to_string()));

                            debug!(
                                "writing browse page to {}",
                                browse_page_path.to_str().unwrap()
                            );
                            tokio::fs::write(&browse_page_path, html.as_bytes())
                                .await
                                .unwrap();
                        }
                        Err(e) => error!(
                            "request error on browse page {} [{}]. Cause: {}",
                            page_num, url, e
                        ),
                    }
                })
            })
            // Convert stream to `BufferUnordered` to poll at most 4 futures at the same time
            .buffer_unordered(4);

        // Poll futures
        loop {
            let res = futures_buffer.next().await;
            if res.is_none() {
                break;
            }
        }

        Ok(())
    }

    async fn number_of_browse_pages(&self) -> Result<u32> {
        info!(
            "getting the total number of browse pages from {}",
            BROWSE_URL
        );

        let html = self
            .client
            .get(BROWSE_URL)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let browse_doc = Html::parse_document(&html);
        let sel = scraper::Selector::parse("li.page-item:nth-last-child(2) > a.page-link")
            .map_err(|e| anyhow::format_err!("could not parse {:#?}", e))?;

        let last_page_a = browse_doc
            .select(&sel)
            .next()
            .ok_or(format_err!("could not find the last page button element"))?;

        let last_page_text = last_page_a
            .text()
            .next()
            .ok_or(format_err!("last page button element doesn't have text"))?;

        let last_page = last_page_text.parse()?;

        Ok(last_page)
    }
}
