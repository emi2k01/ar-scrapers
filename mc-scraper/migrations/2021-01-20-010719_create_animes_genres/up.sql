CREATE TABLE animes_genres (
    id INTEGER PRIMARY KEY,
    anime_id INTEGER,
    genre_id INTEGER,
    FOREIGN KEY(anime_id) REFERENCES animes(id),
    FOREIGN KEY(genre_id) REFERENCES genres(id)
)
