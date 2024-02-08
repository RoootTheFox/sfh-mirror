CREATE TEMPORARY TABLE songs_tmp(id,name,song_name,song_id,download_url,level_id,state);
INSERT INTO songs_tmp SELECT * FROM songs;
DROP TABLE songs;

CREATE TABLE IF NOT EXISTS songs (
    id TEXT PRIMARY KEY NOT NULL,
    name VARCHAR,
    song_name VARCHAR NOT NULL,
    song_id VARCHAR UNSIGNED NOT NULL,
    download_url VARCHAR NOT NULL,
    level_id BIGINT SIGNED,
    state VARCHAR NOT NULL
);

INSERT INTO songs SELECT * FROM songs_tmp;
DROP TABLE songs_tmp;
