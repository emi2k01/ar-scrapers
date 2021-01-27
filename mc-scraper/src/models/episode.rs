use anyhow::{Context, Result};
use scrap::{Html, Selector};

use crate::db::DB;

use super::server::Server;

pub struct Episode {
    pub id: i64,
    pub anime_id: i64,
    pub title: String,
    pub servers: Vec<Server>,
}

impl Episode {
    pub fn extract(doc: &Html, anime_id: i64) -> Result<Self> {
        let title_sel = "h1.Title-epi";
        let title_sel = Selector::parse(title_sel).unwrap();
        let title = doc
            .select(&title_sel)
            .next()
            .context(format!("anime_id: {}, doc: {:#?}", anime_id, doc))
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string();

        let servers = Server::extract_many(doc)?;

        Ok(Self {
            id: 0,
            anime_id,
            title,
            servers,
        })
    }

    pub async fn insert(&mut self) -> Result<(), sqlx::Error> {
        debug!("inserting episode {}", self.title);

        let id = sqlx::query!(
            "INSERT INTO episodes (anime_id, title) VALUES (?, ?)",
            self.anime_id,
            self.title
        )
        .execute(DB.get().unwrap())
        .await?
        .last_insert_rowid();

        self.id = id;

        for server in &self.servers {
            sqlx::query!(
                "INSERT INTO servers (episode_id, name, url) VALUES (?, ?, ?)",
                self.id,
                server.name,
                server.url
            )
            .execute(DB.get().unwrap())
            .await?;
        }

        Ok(())
    }
}
