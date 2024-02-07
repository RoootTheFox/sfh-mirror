use crate::types::DBState;
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

    // todo

    sqlx::query!("INSERT INTO state (key, value) VALUES ('initial_sync_finished', 'true')")
        .execute(&mut *conn)
        .await?;

    Ok(())
}
