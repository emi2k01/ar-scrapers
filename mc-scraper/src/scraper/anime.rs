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

    let doc = scrap::Html::parse_document(body);

    let no_text_error = |el| format_err!("no text node in element in {}");

    let extract_first_text = |mut el: scrap::html::Select, el_sel| -> Result<String> {
        Ok(el
            .next()
            .ok_or(format_err!("no element matches"))?
            .text()
            .next()
            .ok_or(no_text_error(el_sel))?
            .to_string())
    };

    let title = extract_first_text(doc.select(&title_sel), "title")?;
    let synopsis = extract_first_text(doc.select(&synopsis_sel), "synopsis")?;
    let status = extract_first_text(doc.select(&status_sel), "status")?;

    let rel_date_and_kind =
        extract_first_text(doc.select(&rel_date_and_kind_sel), "release date and kind")?;
    let mut rel_date_and_kind = rel_date_and_kind.split(" | ");

    let release_date = rel_date_and_kind
        .next()
        .ok_or(format_err!("no release date"))?
        .to_string();

    let kind = rel_date_and_kind
        .next()
        .ok_or(no_text_error("no kind"))?
        .to_string();

    let genres_res = doc
        .select(&genres_sel)
        .map(|el| el.text().next().ok_or(no_text_error("genres")))
        .collect::<Vec<_>>();

    let mut genres = Vec::with_capacity(genres_res.len());
    for genre_res in genres_res {
        match genre_res {
            Ok(genre_str) => genres.push(genre_str.to_string()),
            Err(e) => return Err(e),
        }
    }

    Ok(Anime {
        title,
        synopsis,
        status,
        release_date,
        kind,
        genres,
    })
}
