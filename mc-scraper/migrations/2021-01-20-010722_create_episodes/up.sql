CREATE TABLE episodes (
    id INTEGER PRIMARY KEY,
    anime_id INTEGER,
    name TEXT,
    FOREIGN KEY(anime_id) REFERENCES animes(id)
)
