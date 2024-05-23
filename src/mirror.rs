use futures::StreamExt;
use rocket::tokio;
use sqlx::pool::PoolConnection;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Error, Pool, Sqlite};

pub(crate) async fn sync(pool: &Pool<Sqlite>) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    let response = reqwest::get("https://api.songfilehub.com/songs")
        .await?
        .json::<Vec<crate::types::Song>>()
        .await?;

    // clone for later use
    let remote_songs = response.clone();

    let mut tasks = vec![];

    tokio::fs::create_dir_all("songs").await?;

    for song in response {
        let download_url = song.download_url.clone();
        let song_id = song.id.clone();
        let conn = pool.acquire().await?;

        let file_path = format!("songs/{}.mp3", song_id);
        if tokio::fs::try_exists(&file_path).await? {
            // we still insert the song into the db, in case it wasn't in there yet
            insert_song_into_db(conn, song).await?;
            continue;
        }

        tasks.push(tokio::spawn(async move {
            println!("downloading song: {} ({})", song_id, download_url);
            let mut file = tokio::fs::File::create(&file_path).await?;
            let mut response = match reqwest::get(&download_url).await {
                Ok(response) => {
                    if !response.status().is_success() {
                        tokio::fs::remove_file(file_path).await?;
                        return Err(anyhow::anyhow!("failed to download song: {}", song_id));
                    }
                    response
                }
                Err(e) => {
                    tokio::fs::remove_file(file_path).await?;
                    return Err(e.into());
                }
            }
            .bytes_stream();

            while let Some(item) = response.next().await {
                match tokio::io::copy(&mut item?.as_ref(), &mut file).await {
                    Ok(_) => {}
                    Err(e) => {
                        tokio::fs::remove_file(file_path).await?;
                        return Err(e.into());
                    }
                };
            }

            insert_song_into_db(conn, song).await?;
            println!("added song: {}", song_id);

            Ok::<_, anyhow::Error>(())
        }));
    }

    // run 'em all!
    let results = futures::future::try_join_all(tasks).await?;

    for result in results {
        match result {
            Ok(_) => {}
            Err(e) => {
                println!("failed to download song: {}", e);
            }
        }
    }

    sqlx::query!("REPLACE INTO state (key, value) VALUES ('initial_sync_finished', 'true')")
        .execute(&mut *conn)
        .await?;

    // after we downloaded all songs, check if we have any that upstream doesn't have
    let local_songs = sqlx::query_as!(crate::types::Song, "SELECT * FROM songs")
        .fetch_all(&mut *conn)
        .await?;

    tokio::fs::create_dir_all("deleted").await?;
    for local_song in local_songs {
        if !remote_songs.iter().any(|s| s.id == local_song.id) {
            println!("deleting song: {}", local_song.id);

            // move file to deleted folder
            tokio::fs::rename(
                format!("songs/{}.mp3", local_song.id),
                format!("deleted/{}.mp3", local_song.id),
            )
            .await?;
            // move into deleted table
            sqlx::query!("REPLACE INTO deleted_songs (id, name, song_name, song_id, download_url, level_id, state) VALUES (?, ?, ?, ?, ?, ?, ?)",
                local_song.id, local_song.name, local_song.song_name, local_song.song_id, local_song.download_url, local_song.level_id, local_song.state)
                .execute(&mut *conn)
                .await?;
            sqlx::query!("DELETE FROM songs WHERE id = ?", local_song.id)
                .execute(&mut *conn)
                .await?;
        }
    }

    println!("sync completed!");

    Ok(())
}

async fn insert_song_into_db(
    mut conn: PoolConnection<Sqlite>,
    song: crate::types::Song,
) -> Result<SqliteQueryResult, Error> {
    let url = format!(
        "{}/{}.mp3",
        <String as AsRef<str>>::as_ref(&crate::PUBLIC_URL_PREFIX),
        song.id
    );

    sqlx::query!(
        "REPLACE INTO songs (id, name, song_name, song_id, download_url, level_id, state)
                    VALUES (?, ?, ?, ?, ?, ?, ?)",
        song.id,
        song.name,
        song.song_name,
        song.song_id,
        url,
        song.level_id,
        song.state,
    )
    .execute(&mut *conn)
    .await
}
