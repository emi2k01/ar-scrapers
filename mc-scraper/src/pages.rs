use diesel::{SqliteConnection, RunQueryDsl};

use crate::schema::{browse_pages, anime_pages, episode_pages};

#[derive(Queryable)]
pub struct BrowsePage {
    pub id: i32,
    pub page: i32,
    pub url: String,
    pub html: String,
    pub html_len: i32,
}

#[derive(Insertable)]
#[table_name = "browse_pages"]
pub struct NewBrowsePage {
    page: i32,
    url: String,
    html: String,
    html_len: i32,
}

impl NewBrowsePage {
    pub fn new(page: i32, url: String, html: String) -> Self {
        let html_len = html.as_bytes().len() as i32;
        Self {
            page,
            url,
            html,
            html_len,
        }
    }
}

#[derive(Queryable)]
pub struct AnimePage {
    pub id: i32,
    pub title: String,
    pub url: String,
    pub html: String,
    pub html_len: i32,
}

#[derive(Insertable)]
#[table_name = "anime_pages"]
pub struct NewAnimePage {
    title: String,
    url: String,
    html: String,
    html_len: i32,
}

impl NewAnimePage {
    pub fn new(title: String, url: String, html: String) -> Self {
        let html_len = html.as_bytes().len() as i32;
        Self {
            title,
            url,
            html,
            html_len,
        }
    }
}

#[derive(Queryable)]
pub struct EpisodePage {
    pub id: i32,
    pub anime_id: i32,
    pub number: i32,
    pub url: String,
    pub html: String,
    pub html_len: i32,
}

#[derive(Insertable)]
#[table_name = "episode_pages"]
pub struct NewEpisodePage {
    anime_id: i32,
    number: i32,
    url: String,
    html: String,
    html_len: i32,
}

impl NewEpisodePage {
    pub fn new(anime_id: i32, number: i32, url: String, html: String) -> Self {
        let html_len = html.as_bytes().len() as i32;
        Self {
            anime_id,
            number,
            url,
            html,
            html_len,
        }
    }
}

macro_rules! impl_insert {
    ($insertable:ident, $table:expr) => {
        impl $insertable {
            pub fn insert(self, db_con: &SqliteConnection) -> anyhow::Result<()> {
                diesel::insert_into($table)
                    .values(&self)
                    .execute(db_con)?;

                Ok(())
            }
        }
    };
}

impl_insert!(NewBrowsePage, browse_pages::table);
impl_insert!(NewAnimePage, anime_pages::table);
impl_insert!(NewEpisodePage, episode_pages::table);
