use anyhow::Result;
use reqwest::Client;
use scrap::{Html, Selector};

use super::{
    anime::scrape_animes_from_browse_body,
    util::{self, fetch_pages},
};

const BROWSE_URL: &'static str = "https://monoschinos2.com/animes";
const BROWSE_SECTION: &'static str = "browse";

pub async fn scrape_browse_pages(client: &Client) -> Result<()> {
    let num_browse_pages = number_of_browse_pages(client).await?;
    let urls = build_browse_urls(num_browse_pages);

    let pages = fetch_pages(client, urls).await;

    for (url, body) in &pages {
        if util::update_or_insert(url, body, BROWSE_SECTION)? {
            scrape_animes_from_browse_body(client, body).await?;
        }
    }

    Ok(())
}

fn build_browse_urls(num_pages: u32) -> Vec<String> {
    let mut urls = Vec::with_capacity(num_pages as usize);

    for i in 1..=num_pages {
        urls.push(format!("{Url}?page={page}", Url = BROWSE_URL, page = i));
    }

    urls
}

async fn number_of_browse_pages(client: &Client) -> Result<u32> {
    info!(
        "getting the total number of browse pages from {}",
        BROWSE_URL
    );

    let browse_page_html = client
        .get(BROWSE_URL)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let browse_doc = Html::parse_document(&browse_page_html);

    let last_page_sel = Selector::parse("li.page-item:nth-last-child(2) > a.page-link")
        .map_err(|e| anyhow::format_err!("could not parse {:#?}", e))?;

    let last_page_a = browse_doc
        .select(&last_page_sel)
        .next()
        .ok_or(format_err!("could not find the last page button element"))?;

    let last_page_text = last_page_a
        .text()
        .next()
        .ok_or(format_err!("last page button element doesn't have text"))?;

    let last_page = last_page_text.parse()?;

    Ok(last_page)
}
