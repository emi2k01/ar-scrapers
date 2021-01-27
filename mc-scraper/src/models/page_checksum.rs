use std::ops::{Deref, DerefMut};

use adler::adler32_slice;
use anyhow::Result;

use crate::db::DB;

pub struct PageChecksum {
    row: PageChecksumRow,
    is_new: bool,
    changed: bool,
}

impl PageChecksum {
    pub async fn from_page(url: String, body: &str) -> Result<Self, sqlx::Error> {
        let existing_res = PageChecksumRow::from_url(&url).await;
        let checksum = adler32_slice(body.as_bytes()).into();

        let (row, is_new, changed) = match existing_res {
            Ok(existing) => {
                let changed = existing.checksum != checksum;
                (existing, false, changed)
            }
            Err(sqlx::Error::RowNotFound) => (
                PageChecksumRow {
                    id: 0,
                    url,
                    checksum,
                },
                true,
                true,
            ),
            Err(e) => return Err(e),
        };

        Ok(Self {
            row,
            is_new,
            changed,
        })
    }

    /// Returns true if the PageChecksum did not exist
    pub fn is_new(&self) -> bool {
        self.is_new
    }

    /// Returns true if the PageChecksum existed but the checksum changed
    pub fn changed(&self) -> bool {
        self.changed
    }
}

impl Deref for PageChecksum {
    type Target = PageChecksumRow;

    fn deref(&self) -> &Self::Target {
        &self.row
    }
}

impl DerefMut for PageChecksum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.row
    }
}

pub struct PageChecksumRow {
    id: i64,
    url: String,
    checksum: i64,
}

impl PageChecksumRow {
    async fn from_url(url: &str) -> Result<Self, sqlx::Error> {
        let row_res = sqlx::query_as!(
            PageChecksumRow,
            "SELECT * FROM page_checksums WHERE url = ?",
            url,
        )
        .fetch_one(DB.get().unwrap())
        .await;

        match row_res {
            Ok(existing) => Ok(existing),
            Err(e) => Err(e),
        }
    }

    /// Inserts the PageChecksumRow into the table and sets self.id to the inserted row id
    pub async fn insert(&mut self) -> Result<(), sqlx::Error> {
        debug!("querying PageChecksum::insert`");

        let id = sqlx::query!(
            "INSERT INTO page_checksums (url, checksum) VALUES (?, ?)",
            self.url,
            self.checksum
        )
        .execute(DB.get().unwrap())
        .await?
        .last_insert_rowid();

        self.id = id;

        Ok(())
    }

    pub async fn update(&self) -> Result<(), sqlx::Error> {
        debug!("querying PageChecksum::update`");

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
