#[derive(sqlx::FromRow)]
pub struct DBState {
    pub key: String,
    pub value: String,
}
