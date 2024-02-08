use crate::types::Song;
use crate::Db;
use rocket::get;
use rocket::serde::json::Json;

use rocket_db_pools::Connection;

#[get("/api/v1/get_song/<id>")]
pub(crate) async fn get_song(mut conn: Connection<Db>, id: &str) -> Json<Song> {
    let song = sqlx::query_as_unchecked!(
        Song,
        "SELECT *
            FROM songs
            WHERE id = ?",
        id
    )
    .fetch_one(&mut **conn)
    .await
    .unwrap();

    song.into()
}

#[get("/api/v1/get_songs_for_level/<level_id>")]
pub(crate) async fn get_songs_for_level(
    mut conn: Connection<Db>,
    level_id: &str,
) -> Json<Vec<Song>> {
    let song = sqlx::query_as_unchecked!(
        Song,
        "SELECT *
            FROM songs
            WHERE level_id = ?",
        level_id
    )
    .fetch_all(&mut **conn)
    .await
    .unwrap();

    song.into()
}

#[get("/api/v1/get_songs_with_id/<song_id>")]
pub(crate) async fn get_songs_with_id(mut conn: Connection<Db>, song_id: &str) -> Json<Vec<Song>> {
    let song = sqlx::query_as_unchecked!(
        Song,
        "SELECT *
            FROM songs
            WHERE song_id = ?",
        song_id
    )
    .fetch_all(&mut **conn)
    .await
    .unwrap();

    song.into()
}
