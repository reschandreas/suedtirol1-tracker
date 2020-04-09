use super::schema::songs;
use super::schema::logs;

#[derive(Queryable)]
pub struct Song {
    pub id: i32,
    pub title: String,
    pub artist: String,
}

#[derive(Queryable)]
pub struct Log {
    pub date: chrono::NaiveDateTime,
    pub song: i32,
    pub is_new: bool,
}

#[derive(Insertable)]
#[table_name="songs"]
pub struct NewSong<'a> {
    pub title: &'a str,
    pub artist: &'a str,
}

#[derive(Insertable)]
#[table_name="logs"]
pub struct NewLog {
    pub date: chrono::NaiveDateTime,
    pub song: i32,
    pub is_new: bool,
}
