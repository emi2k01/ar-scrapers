CREATE TABLE animes_urls (
    id INTEGER PRIMARY KEY NOT NULL,
    url TEXT NOT NULL,
    anime_id INTEGER NOT NULL,

    FOREIGN KEY (anime_id) REFERENCES animes(id)
)
