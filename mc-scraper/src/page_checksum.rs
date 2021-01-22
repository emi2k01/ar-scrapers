pub struct PageChecksum {
    pub id: i32,
    pub section: String,
    pub url: String,
    pub checksum: i32,
}

impl PageChecksum {
    pub fn insert(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub fn update(&self) -> anyhow::Result<()> {
        todo!()
    }
}

pub fn browse_checksums() -> anyhow::Result<Vec<PageChecksum>> {
    page_checksums("browse")
}

pub fn anime_checksums() -> anyhow::Result<Vec<PageChecksum>> {
    page_checksums("anime")
}

pub fn episode_checksums() -> anyhow::Result<Vec<PageChecksum>> {
    page_checksums("episode")
}

pub fn page_checksums(section: &str) -> anyhow::Result<Vec<PageChecksum>> {
    todo!()
}

pub fn select_by_url(section: &str, url: &str) -> Option<PageChecksum> {
    todo!()
}
