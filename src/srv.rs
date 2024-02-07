use rocket::get;

#[get("/v")]
pub(crate) fn version() -> String {
    format!("{} {}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"))
}
