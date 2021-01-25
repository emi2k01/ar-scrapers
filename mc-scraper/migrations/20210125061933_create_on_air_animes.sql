CREATE TABLE on_air_animes (
    id INTEGER PRIMARY KEY NOT NULL,
    anime_id INTEGER NOT NULL,
    url TEXT NOT NULL,

    FOREIGN KEY (anime_id) REFERENCES animes(id)
)