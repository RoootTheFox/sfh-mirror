use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_option_number_from_string;

#[derive(sqlx::FromRow)]
pub struct DBState {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Song {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub name: Option<String>,
    #[serde(rename(deserialize = "songName"))]
    pub song_name: String,
    #[serde(rename(deserialize = "songID"))]
    pub song_id: String,
    #[serde(rename(deserialize = "downloadUrl"))]
    pub download_url: String,
    #[serde(default)]
    #[serde(rename(deserialize = "levelID"), deserialize_with = "deserialize_option_number_from_string")]
    pub level_id: Option<String>,
}