pub struct Anime {
    pub title: String,
    pub synopsis: String,
    pub status: String,
    pub release_date: String,
    pub kind: String,
    pub genres: Vec<String>,
    pub episodes: Vec<Episode>,
}

pub struct Episode {
    pub name: String,
    pub servers: Vec<Server>,
}

pub struct Server {
    pub name: String,
    pub url: String,
}
