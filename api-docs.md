## sfh-mirror API documentation

### all values are in JSON.

## Data Types

### Song
| field        | type   | description                                                                                   | optional? |
|--------------|--------|-----------------------------------------------------------------------------------------------|-----------|
| id           | string | The internal unique ID for the song                                                           | no        |
| name         | string | The name of the level that this song is intended for.                                         | yes       |
| song_name    | string | The name of the song.                                                                         | no        |
| song_id      | string | The ID by the song as used in-game. Usually a number, but can be a string for default levels. | no        |
| download_url | string | The direct download URL for the song file.                                                    | no        |
| level_id     | int    | The in-game ID of the level that this song is meant for.                                      | yes       |
| state        | string | The SFH state of the song. Can be any of: `rated`, `unrated`, `mashup`, `challenge`, `remix`  | no        |

## Endpoints
**The current API version is `v1`.** When this version gets superseded, this documentation will be updated.<br>
The API will always support the last 2 API versions, unless there's a **very good reason** to drop support.

### `GET /api/v1/get_song/<id>`
**Description: get a song by it's unique ID**
| parameter | type | description | optional? |
|-----------|------|-------------|-----------|
| id | string | The song's unique ID. **THIS IS NOT the in-game song id!** | no |

**Example request:**
`GET /api/v1/get_song/654eba69eaa65fe484bdb0c1`<br>
**response:**
```json
{
  "id": "654eba69eaa65fe484bdb0c1",
  "name": "Overthinker",
  "song_name": "INZO - Overthinker",
  "song_id": "1252153",
  "download_url": "https://cdn-sfh.rooot.gay/654eba69eaa65fe484bdb0c1.mp3",
  "level_id": 95106737,
  "state": "rated"
}
```

---
### `GET /api/v1/get_songs_for_level/<level_id>`
**Description: get a list of songs for a specific level**
| parameter | type | description | optional? |
|-----------|------|-------------|-----------|
| level_id | string | The in-game level ID to search songs for | no |

**Example request:**
`GET /api/v1/get_songs_for_level/58825144`<br>
**response:**
```json
[
  {
    "id": "64f54c6ceba5efcdadf78c0a",
    "name": "Xo",
    "song_name": "EDEN - xo (Aaron Musslewhite Cover & Remake)",
    "song_id": "766165",
    "download_url": "https://cdn-sfh.rooot.gay/64f54c6ceba5efcdadf78c0a.mp3",
    "level_id": 58825144,
    "state": "rated"
  },
  {
    "id": "64f54c6ceba5efcdadf78c0b",
    "name": "Xo",
    "song_name": "EDEN - xo (Aaron Musslewhite Cover & Remake) x blackbear - idfc",
    "song_id": "766165",
    "download_url": "https://cdn-sfh.rooot.gay/64f54c6ceba5efcdadf78c0b.mp3",
    "level_id": 58825144,
    "state": "mashup"
  }
]
```

---
### `GET /api/v1/get_songs_with_id/8800`
**Description: get a list of songs that have a specific in-game ID (e.g. replacement/NONG songs)**
| parameter | type | description | optional? |
|-----------|------|-------------|-----------|
| song_id | string | The **in-game song ID** to find songs for. | no |

> [!tip]
> You are probably looking for this and NOT `get_songs_for_level` if you want to find a replacement song for a level,
> since levels always have the correct song ID attached to them; while the database may not know about the specific level ID
> that you're searching songs for. *You usually shouldn't have to query both endpoints.*

**Example request:**
`GET /api/v1/get_songs_with_id/8800`<br>
**response:**
```json
[
  {
    "id": "64f54c6ceba5efcdadf78a0e",
    "name": "Murder Mitten",
    "song_name": "I See Stars - Murder Mitten",
    "song_id": "8800",
    "download_url": "https://cdn-sfh.rooot.gay/64f54c6ceba5efcdadf78a0e.mp3",
    "level_id": null,
    "state": "challenge"
  }
]
```
