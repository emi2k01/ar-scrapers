use crate::db::DB;

pub struct EpisodeUrl {
    id: i64,
    episode_id: i64,
    url: String,
}

impl EpisodeUrl {
    pub fn new(url: String, episode_id: i64) -> Self {
        Self {
            id: 0,
            episode_id,
            url,
        }
    }

    pub async fn insert(&mut self) -> Result<(), sqlx::Error> {
        let id = sqlx::query!(
            "INSERT INTO episodes_urls (episode_id, url) VALUES (?, ?)",
            self.episode_id,
            self.url,
        )
        .execute(DB.get().unwrap())
        .await?
        .last_insert_rowid();

        self.id = id;

        Ok(())
    }

    pub async fn exists(&self) -> Result<bool, sqlx::Error> {
        Ok(
            sqlx::query!("SELECT id from episodes_urls where url = ?", self.url)
                .fetch_optional(DB.get().unwrap())
                .await?
                .is_some(),
        )
    }
}
