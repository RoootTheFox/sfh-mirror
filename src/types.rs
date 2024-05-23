use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::serde_json;
use rocket::{response, Request, Response};
use serde::ser::SerializeMap;
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
            Err(_e) => {
                /*println!(
                    "warning: failed to parse song id '{}': {} - returning None",
                    s, e
                );*/
                Ok(None)
            }
        },
        StrOrNull::Null => Ok(None),
    }
}

// Error types

#[derive(Debug, thiserror::Error)]
pub enum GenericError {
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
}

impl Serialize for GenericError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let GenericError::SqlxError(err) = self;

        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("error", &err.to_string())?;
        map.serialize_entry("status", &"error")?;
        map.end()
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for GenericError {
    fn respond_to(self, _req: &'r Request<'_>) -> response::Result<'o> {
        match self {
            GenericError::SqlxError(ref e) => {
                let mut builder = Response::build();
                let builder = builder
                    .header(ContentType::JSON)
                    .streamed_body(std::io::Cursor::new(serde_json::to_string(&self).unwrap()));

                if let sqlx::Error::RowNotFound = e {
                    builder.status(Status::NotFound);
                } else {
                    builder.status(Status::InternalServerError);
                }

                Ok(builder.finalize())
            }
        }
    }
}
