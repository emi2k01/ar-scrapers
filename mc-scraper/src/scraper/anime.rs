use anyhow::{format_err, Result};
use reqwest::Client;

use crate::anime::Anime;

use super::util;

const ANIME_SECTION: &'static str = "anime";

pub async fn scrape_animes_from_browse_body(client: &Client, browse_body: &str) -> Result<()> {
    let doc = scrap::Html::parse_document(browse_body);

    let link_anime_sel = scrap::Selector::parse("a.link-anime").unwrap();

    let links = doc.select(&link_anime_sel);

    for link in links {
        let link_url = link
            .value()
            .attr("href")
            .ok_or(format_err!("a.link-anime does not have `href` attribute"))?
            .to_string();

        scrape_anime_from_url(client, link_url).await?;
    }

    Ok(())
}

async fn scrape_anime_from_url(client: &Client, url: String) -> Result<()> {
    let (url, body) = util::fetch_page(client, url).await?;

    if util::update_or_insert(&url, &body, ANIME_SECTION)? {
        let anime = extract_anime(&body);
    }

    todo!()
}

fn extract_anime(body: &str) -> Result<Anime> {
    let parse_sel = |sel| {
        scrap::Selector::parse(sel).map_err(|e| format_err!("error parsing sel [{}]: {:?}", sel, e))
    };

    let title_sel = parse_sel("h1.Title")?;
    let synopsis_sel = parse_sel("div.Description")?;
    let status_sel = parse_sel("div.Type")?;
    let rel_date_and_kind_sel = parse_sel("div.after_title")?;
    let genres_sel = parse_sel("div.generos a")?;
    let episodes_sel = parse_sel("div.SerieCaps a")?;

    todo!()
}
