use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Log {
    pub date: String,
    pub data: Option<Song>,
    pub in_db: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    pub artist: String,
    pub title: String,
    pub is_new: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ParsedResult {
    pub past: Option<Song>,
    pub present: Option<Song>,
    pub future: Option<Song>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiResult {
    pub unparsed: String,
    pub past: Option<ApiSong>,
    pub present: Option<ApiSong>,
    pub future: Option<ApiSong>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ApiSong {
    pub artist: String,
    pub title: String,
}
