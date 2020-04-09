use self::models::*;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insert_or_get_song(conn: &PgConnection, song: NewSong) -> Option<Song> {
    use self::schema::songs::dsl::*;
    use schema::songs;

    let _rows = diesel::insert_into(songs::table)
        .values(&song)
        .on_conflict_do_nothing()
        .execute(conn);

    songs
        .filter(title.eq(song.title))
        .filter(artist.eq(song.artist))
        .load::<Song>(conn)
        .expect("Error getting song")
        .pop()
}

pub fn insert_log(conn: &PgConnection, log: NewLog) -> Option<Log> {
    use schema::logs;

    diesel::insert_into(logs::table)
        .values(&log)
        .get_results(conn)
        .expect("Error inserting log")
        .pop()
}

pub fn add_log(date: chrono::NaiveDateTime, song: crate::models::Song) -> Option<Log> {
    let conn = establish_connection();

    let new_song = NewSong {
        title: &song.title,
        artist: &song.artist,
    };

    match insert_or_get_song(&conn, new_song) {
        Some(new_song) => {
            let new_log = NewLog {
                date,
                song: new_song.id,
                is_new: song.is_new,
            };

            insert_log(&conn, new_log)
        }
        None => {
            eprintln!("Something's wrong I can feel it");
            None::<Log>
        }
    }
}
