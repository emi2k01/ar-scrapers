CREATE TABLE episodes (
    id INTEGER PRIMARY KEY NOT NULL,
    anime_id INTEGER NOT NULL,
    title TEXT NOT NULL,

    FOREIGN KEY (anime_id) REFERENCES animes(id)
)
