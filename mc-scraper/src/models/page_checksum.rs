use adler::adler32_slice;
use anyhow::Result;

use crate::db::DB;

pub struct PageChecksum {
    id: i64,
    url: String,
    checksum: i64,
}

impl PageChecksum {
    pub fn from_page(url: String, body: &str) -> Self {
        let checksum = adler32_slice(body.as_bytes()).into();

        Self {
            id: 0,
            url,
            checksum,
        }
    }

    /// Returns true if the PageChecksum did not exist
    pub async fn is_new(&self) -> Result<bool, sqlx::Error> {
        let row_res = sqlx::query_as!(
            PageChecksum,
            "SELECT * FROM page_checksums WHERE url = ?",
            self.url
        )
        .fetch_one(DB.get().unwrap())
        .await;

        match row_res {
            Ok(_) => Ok(false),
            Err(sqlx::Error::RowNotFound) => Ok(true),
            Err(e) => Err(e),
        }
    }

    /// Returns true if the PageChecksum existed but the checksum changed
    pub async fn changed(&self) -> Result<bool, sqlx::Error> {
        let row_res = sqlx::query_as!(
            PageChecksum,
            "SELECT * FROM page_checksums WHERE url = ?",
            self.url,
        )
        .fetch_one(DB.get().unwrap())
        .await;

        match row_res {
            Ok(existing) => Ok(existing.checksum != self.checksum),
            Err(e) => Err(e),
        }
    }

    pub async fn insert(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO page_checksums (url, checksum) VALUES (?, ?)",
            self.url,
            self.checksum
        )
        .execute(DB.get().unwrap())
        .await?;

        Ok(())
    }

    pub async fn update(&self) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE page_checksums SET checksum = ? WHERE url = ?",
            self.checksum,
            self.url,
        )
        .execute(DB.get().unwrap())
        .await?;

        Ok(())
    }
}
