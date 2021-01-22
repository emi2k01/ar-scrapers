CREATE TABLE episode_pages (
    id INTEGER PRIMARY KEY,
    anime_id INTEGER,
    number INTEGER,
    url TEXT,
    html TEXT,
    html_len INTEGER,
    FOREIGN KEY(anime_id) REFERENCES anime_pages(id)
)