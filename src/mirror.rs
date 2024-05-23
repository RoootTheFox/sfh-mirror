use futures::StreamExt;
use rocket::tokio;
use sqlx::mysql::MySqlQueryResult;
use sqlx::pool::PoolConnection;
use sqlx::{Error, MySql, Pool};

pub(crate) async fn check_initial_sync(pool: &Pool<MySql>) -> anyhow::Result<bool> {
    let mut conn = pool.acquire().await?;

    let initial_sync_finished =
        sqlx::query!("SELECT `key`, `value` FROM `state` WHERE  `key`='initial_sync_finished'",)
            .fetch_one(&mut *conn)
            .await?;

    Ok(initial_sync_finished.value.unwrap() == "true")
}

pub(crate) async fn initial_sync(pool: &Pool<MySql>) -> anyhow::Result<()> {
    let mut conn = pool.acquire().await?;

    let response = reqwest::get("https://api.songfilehub.com/songs")
        .await?
        .json::<Vec<crate::types::Song>>()
        .await?;

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

    sqlx::query!("UPDATE `sfh`.`state` SET `value`='true' WHERE  `key`='initial_sync_finished'")
        .execute(&mut *conn)
        .await?;

    println!("initial sync completed!");

    Ok(())
}

async fn insert_song_into_db(
    mut conn: PoolConnection<MySql>,
    song: crate::types::Song,
) -> Result<MySqlQueryResult, Error> {
    let url = format!(
        "{}/{}.mp3",
        <String as AsRef<str>>::as_ref(&crate::PUBLIC_URL_PREFIX),
        song.id
    );

    if sqlx::query_as!(
        crate::types::Song,
        "SELECT *
            FROM songs
            WHERE id = ?",
        song.id
    )
    .fetch_one(&mut *conn)
    .await
    .is_err()
    {
        sqlx::query!(
            "INSERT INTO songs  (id, name, song_name, song_id, download_url, level_id, state)
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
    } else {
        sqlx::query(
            "SELECT *
            FROM songs",
        )
        .execute(&mut *conn)
        .await
    }
}
