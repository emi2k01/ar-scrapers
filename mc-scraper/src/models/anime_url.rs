use crate::db::DB;

pub struct AnimeUrl {
    pub id: i64,
    pub anime_id: i64,
    pub url: String,
}

impl AnimeUrl {
    pub fn new(url: String, anime_id: i64) -> Self {
        Self {
            id: 0,
            url,
            anime_id,
        }
    }

    pub async fn insert(&mut self) -> Result<(), sqlx::Error> {
        let id = sqlx::query!(
            "INSERT INTO animes_urls (anime_id, url) VALUES (?, ?)",
            self.anime_id,
            self.url,
        )
        .execute(DB.get().unwrap())
        .await?
        .last_insert_rowid();

        self.id = id;

        Ok(())
    }

    pub async fn select_by_url(url: &str) -> Result<Self, sqlx::Error> {
        Ok(
            sqlx::query_as!(AnimeUrl, "SELECT * FROM animes_urls WHERE url = ?", url)
                .fetch_one(DB.get().unwrap())
                .await?,
        )
    }
}
