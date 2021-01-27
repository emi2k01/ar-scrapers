CREATE TABLE episodes_urls (
    id INTEGER PRIMARY KEY NOT NULL,
    url TEXT NOT NULL,
    episode_id INTEGER NOT NULL,

    FOREIGN KEY (episode_id) REFERENCES episodes(id)
)
