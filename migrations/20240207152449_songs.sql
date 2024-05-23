CREATE TABLE IF NOT EXISTS songs
(
    id TEXT,
    name TEXT,
    song_name TEXT NOT NULL,
    song_id TEXT NOT NULL,
    download_url TEXT NOT NULL,
    level_id INT UNSIGNED,
    state TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS `state` (
    `key` VARCHAR(255) NOT NULL,
    `value` TEXT
) 
