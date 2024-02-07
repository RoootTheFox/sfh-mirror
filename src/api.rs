use rocket::get;

#[get("/api/v1/get_song/<id>")]
pub(crate) fn get_song(id: u32) -> String {
    format!("song id: {}", id).to_string()
}
