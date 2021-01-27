use anyhow::Result;
use scrap::Html;

use crate::db::DB;

use super::AnimeUrl;

pub struct Anime {
    pub id: i64,
    pub title: String,
    pub synopsis: String,
    pub status: String,
    pub release_date: String,
    pub kind: String,
    pub genres: Vec<String>,
}

impl Anime {
    pub fn extract(doc: &Html) -> Result<Self> {
        let parse_sel = |sel| {
            scrap::Selector::parse(sel)
                .map_err(|e| format_err!("error parsing sel [{}]: {:?}", sel, e))
        };

        let no_text_error = |el| format_err!("no text node in element in {}", el);

        let extract_first_text = |mut el: scrap::html::Select, el_sel| -> Result<String> {
            Ok(el
                .next()
                .ok_or(format_err!("no element matches: {}", el_sel))?
                .text()
                .next()
                .ok_or(no_text_error(el_sel))?
                .to_string())
        };

        let title_sel = parse_sel("h1.Title")?;
        let synopsis_sel = parse_sel("div.Description")?;
        let status_sel = parse_sel("div.Type")?;
        let rel_date_and_kind_sel = parse_sel("div.after-title")?;
        let genres_sel = parse_sel("div.generos a")?;

        let title = extract_first_text(doc.select(&title_sel), "title")?;
        let synopsis = extract_first_text(doc.select(&synopsis_sel), "synopsis")?;
        let status = extract_first_text(doc.select(&status_sel), "status")?;

        let rel_date_and_kind = doc
            .select(&rel_date_and_kind_sel)
            .next()
            .unwrap()
            .text()
            .nth(3)
            .unwrap()
            .trim();

        debug!("release date and kind: {}", rel_date_and_kind);

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
            id: 0,
            title,
            synopsis,
            status,
            release_date,
            kind,
            genres,
        })
    }

    pub async fn insert(&mut self) -> Result<(), sqlx::Error> {
        let id = sqlx::query!(
            "INSERT INTO animes (title, synopsis, status, release_date, kind) VALUES (?, ?, ?, ?, ?)",
            self.title,
            self.synopsis,
            self.status,
            self.release_date,
            self.kind,
        ).execute(DB.get().unwrap())
        .await?
        .last_insert_rowid();

        self.id = id;

        for genre in &self.genres {
            let genre_id_res = sqlx::query!("SELECT id FROM genres WHERE genre = ?", genre)
                .fetch_optional(DB.get().unwrap())
                .await?;

            let genre_id = match genre_id_res {
                Some(record) => record.id,
                None => sqlx::query!("INSERT INTO genres (genre) VALUES (?)", genre)
                    .execute(DB.get().unwrap())
                    .await?
                    .last_insert_rowid(),
            };

            sqlx::query!(
                "INSERT INTO animes_genres (anime_id, genre_id) VALUES (?, ?)",
                id,
                genre_id,
            )
            .execute(DB.get().unwrap())
            .await?;
        }

        Ok(())
    }

    pub async fn select_by_url(url: &str) -> Result<Self, sqlx::Error> {
        let anime_url = AnimeUrl::select_by_url(url).await?;
        let anime_dyn = sqlx::query!("SELECT * FROM animes WHERE id = ?", anime_url.id)
            .fetch_one(DB.get().unwrap())
            .await?;

        let genres_ids = sqlx::query!(
            "SELECT (genre_id) FROM animes_genres WHERE anime_id = ?",
            anime_dyn.id,
        )
        .fetch_all(DB.get().unwrap())
        .await?;

        let mut genres = Vec::with_capacity(5);
        for genre_id in genres_ids {
            let genre = sqlx::query!("SELECT (genre) FROM genres WHERE id = ?", genre_id.genre_id)
                .fetch_one(DB.get().unwrap())
                .await?;
            genres.push(genre.genre);
        }

        Ok(Anime {
            id: anime_dyn.id,
            title: anime_dyn.title,
            synopsis: anime_dyn.synopsis,
            status: anime_dyn.status,
            release_date: anime_dyn.release_date,
            kind: anime_dyn.kind,
            genres,
        })
    }
}
