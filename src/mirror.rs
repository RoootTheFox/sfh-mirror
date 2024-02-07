use crate::types::DBState;
use futures::StreamExt;
use rocket::tokio;
use sqlx::{Pool, Sqlite};

pub(crate) async fn check_initial_sync(pool: Pool<Sqlite>) -> anyhow::Result<bool> {
    let mut conn = pool.acquire().await?;

    let initial_sync_finished = sqlx::query_as!(
        DBState,
        "SELECT key, value
            FROM state
            WHERE key = 'initial_sync_finished'",
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap_or(DBState {
        key: "initial_sync_finished".to_string(),
        value: "false".to_string(),
    });

    Ok(initial_sync_finished.value == "true")
}

pub(crate) async fn initial_sync(pool: Pool<Sqlite>) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    let reqwest = reqwest::Client::new();
    let response = reqwest
        .get("https://api.songfilehub.com/songs")
        .send()
        .await?
        .json::<Vec<crate::types::Song>>()
        .await?;

    // download 10 songs at a time
    let mut tasks = vec![];

    tokio::fs::create_dir_all("songs").await?;

    for song in response {
        let download_url = song.download_url.clone();
        let song_id = song.id.clone();
        let mut conn = pool.acquire().await?;

        let file_path = format!("songs/{}.mp3", song_id);
        if tokio::fs::try_exists(&file_path).await? {
            println!("song already exists: {}", song_id);
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
            }.bytes_stream();

            while let Some(item) = response.next().await {
                match tokio::io::copy(&mut item?.as_ref(), &mut file).await {
                    Ok(_) => {},
                    Err(e) => {
                        tokio::fs::remove_file(file_path).await?;
                        return Err(e.into());
                    }
                };
            }

            println!("finished downloading song: {}", song_id);

            sqlx::query!(
                "REPLACE INTO songs (id, name, song_name, song_id, download_url, level_id)\
                VALUES (?, ?, ?, ?, ?, ?)",
                song.id,
                song.name,
                song.song_name,
                song.song_id,
                song.download_url,
                song.level_id,
            )
            .execute(&mut *conn)
            .await?;
            println!("inserted song into db: {}", song_id);

            Ok::<_, anyhow::Error>(())
        }));
    }

    // run 'em all!
    let results = futures::future::try_join_all(tasks).await?;

    println!("checking for errors");
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

    Ok(())
}
