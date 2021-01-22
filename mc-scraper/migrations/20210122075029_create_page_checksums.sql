CREATE TABLE page_checksums (
    id INTEGER PRIMARY KEY NOT NULL,
    section TEXT NOT NULL,
    url TEXT NOT NULL,
    checksum INTEGER NOT NULL
)
