CREATE TABLE IF NOT EXISTS songs (
    id TEXT PRIMARY KEY NOT NULL, /* id of the SFH song */
    name VARCHAR,
    song_name VARCHAR NOT NULL,
    song_id INT UNSIGNED NOT NULL, /* in-game song id */
    download_url VARCHAR NOT NULL, /* direct download link, this is subject to change in the future */
    level_id INT UNSIGNED /* some songs have a specific level ID attached to them */
);

CREATE TABLE IF NOT EXISTS state (
    key VARCHAR PRIMARY KEY NOT NULL,
    value VARCHAR NOT NULL
);