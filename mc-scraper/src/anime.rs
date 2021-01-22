struct Anime {
    title: String,
    synopsis: String,
    status: String,
    release_date: String,
    kind: String,
    genres: Vec<String>,
    episodes: Vec<Episode>,
}

struct Episode {
    name: String,
    servers: Vec<Server>,
}

struct Server {
    name: String,
    url: String,
}
