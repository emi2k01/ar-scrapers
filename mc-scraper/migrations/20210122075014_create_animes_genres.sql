CREATE TABLE animes_genres (
    id INTEGER PRIMARY KEY NOT NULL,
    anime_id INTEGER NOT NULL,
    genre_id INTEGER NOT NULL,

    FOREIGN KEY (anime_id) REFERENCES animes(id),
    FOREIGN KEY (genre_id) REFERENCES genres(id)
)