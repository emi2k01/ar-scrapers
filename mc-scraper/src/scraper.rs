use anyhow::Result;
use fetchers::{fetch_browse_pages, fetch_pages_from_anchors};
use futures::{stream, StreamExt};
use scrap::{Html, Selector};

use crate::{
    fetchers,
    models::{Anime, AnimeUrl, Episode, EpisodeUrl, PageChecksum},
};

pub async fn scrape() -> Result<()> {
    let browse_pages = fetch_browse_pages().await;

    for (_browse_url, browse_body) in browse_pages {
        let animes_pages = fetch_pages_from_anchors(&browse_body, "a.link-anime").await;

        stream::iter(animes_pages)
            .map(|(anime_url, anime_body)| {
                tokio::spawn(async { scrape_anime(anime_url, anime_body).await.unwrap() })
            })
            .buffer_unordered(1)
            .collect::<Vec<_>>()
            .await;
    }

    Ok(())
}

async fn scrape_anime(anime_url: String, anime_body: String) -> Result<()> {
    let mut anime_page_checksum = PageChecksum::from_page(anime_url.clone(), &anime_body).await?;

    if anime_page_checksum.is_new() {
        debug!("page {} is new", anime_url);

        let mut anime = {
            let anime_doc = Html::parse_document(&anime_body);
            Anime::extract(&anime_doc)?
        };

        anime.insert().await?;

        let mut anime_url = AnimeUrl::new(anime_url, anime.id);
        anime_url.insert().await?;

        let episodes_pages = fetch_pages_from_anchors(&anime_body, "div.SerieCaps a.item").await;

        for (episode_url, episode_body) in episodes_pages {
            let mut episode = {
                let episode_doc = Html::parse_document(&episode_body);
                Episode::extract(&episode_doc, anime.id)?
            };

            episode.insert().await?;

            let mut episode_url = EpisodeUrl::new(episode_url, episode.id);
            episode_url.insert().await?;
        }

        anime_page_checksum.insert().await?;
    } else if anime_page_checksum.changed() {
        debug!("page {} changed", anime_url);

        let episodes_urls_iter = {
            let anime_doc = Html::parse_document(&anime_body);
            let episodes_urls_sel = Selector::parse("div.SerieCaps a.item").unwrap();

            anime_doc
                .select(&episodes_urls_sel)
                .filter_map(|el| el.value().attr("href"))
                .map(|url| url.to_string())
                .collect::<Vec<_>>()
        };

        let new_episodes_urls = stream::iter(episodes_urls_iter)
            .filter(|url| {
                let url = url.clone();
                async {
                    let exists = EpisodeUrl::new(url, 0).exists().await.unwrap();
                    !exists
                }
            })
            .collect::<Vec<_>>()
            .await;

        let anime_id = AnimeUrl::select_by_url(&anime_url).await?.anime_id;

        let new_episodes_pages = fetchers::fetch_pages(new_episodes_urls).await;
        for (episode_url, episode_body) in new_episodes_pages {
            let mut episode = {
                let episode_doc = Html::parse_document(&episode_body);
                Episode::extract(&episode_doc, anime_id)?
            };
            episode.insert().await?;

            let mut episode_url = EpisodeUrl::new(episode_url, episode.id);
            episode_url.insert().await?;
        }

        anime_page_checksum.update().await?;
    } else {
        debug!("page {} didn't change", anime_url);
    }

    Ok(())
}
