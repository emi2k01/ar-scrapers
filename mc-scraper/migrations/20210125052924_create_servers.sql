CREATE TABLE servers (
    id INTEGER PRIMARY KEY NOT NULL,
    episode_id INTEGER NOT NULL,
    name TEXT DEFAULT 'Generic',
    url TEXT NOT NULL,

    FOREIGN KEY (episode_id) REFERENCES episodes(id)
)