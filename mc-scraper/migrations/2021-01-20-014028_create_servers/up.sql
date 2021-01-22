CREATE TABLE servers (
    id INTEGER PRIMARY KEY,
    episode_id INTEGER,
    name TEXT,
    url TEXT,
    FOREIGN KEY(episode_id) REFERENCES episodes(id)
)
