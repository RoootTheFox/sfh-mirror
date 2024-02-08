use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
    #[serde(
        rename(deserialize = "levelID"),
        deserialize_with = "parse_and_fix_song_id"
    )]
    pub level_id: Option<i64>,
    pub state: String,
}

fn parse_and_fix_song_id<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de> + std::str::FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrNull<T> {
        String(String),
        Number(T),
        Null,
    }

    match StrOrNull::<T>::deserialize(deserializer)? {
        StrOrNull::Number(n) => Ok(Some(n)),
        StrOrNull::String(s) => match s.replace([' ', '-'], "").parse() {
            Ok(n) => Ok(Some(n)),
            Err(e) => {
                println!(
                    "warning: failed to parse song id '{}': {} - returning None",
                    s, e
                );
                Ok(None)
            }
        },
        StrOrNull::Null => Ok(None),
    }
}
