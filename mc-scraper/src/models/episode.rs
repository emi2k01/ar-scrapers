use anyhow::Result;
use scrap::{Html, Selector};

use super::server::Server;

pub struct Episode {
    pub title: String,
    pub servers: Vec<Server>,
}

impl Episode {
    pub fn extract(doc: &Html) -> Result<Self> {
        let title_sel = "div.Title-epi";
        let title_sel = Selector::parse(title_sel).unwrap();
        let title = doc
            .select(&title_sel)
            .next()
            .unwrap()
            .text()
            .next()
            .unwrap()
            .to_string();

        let servers = Server::extract_many(doc)?;

        Ok(Self { title, servers })
    }
}
