-- Add migration script here
CREATE TABLE IF NOT EXISTS deleted_songs (
    id TEXT PRIMARY KEY NOT NULL,
    name VARCHAR,
    song_name VARCHAR NOT NULL,
    song_id VARCHAR UNSIGNED NOT NULL,
    download_url VARCHAR NOT NULL,
    level_id BIGINT SIGNED,
    state VARCHAR NOT NULL
);
