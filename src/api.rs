use crate::types::Song;
use crate::Db;
use rocket::get;
use rocket_db_pools::Connection;

#[get("/api/v1/get_song/<id>")]
pub(crate) async fn get_song(mut conn: Connection<Db>, id: &str) -> String {
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

    format!("song id: {} -> {:?}", id, song).to_string()
}
