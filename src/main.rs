#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::models::{ApiResult, Log, ParsedResult, Song};
use chrono::Utc;
use rand::Rng;
use std::{thread, time};

mod db;
mod models;

fn main() {
    let mut last_entry = String::new();
    loop {
        let api_result = get_json(
            "http://www.suedtirol1.it/routing/acc_fun_interaktiv/api/v1/playlist/index.json?v=1",
        );

        let parsed = from_api_result(&api_result);

        let new_result = match api_result {
            Ok(api) => api.unparsed.unwrap(),
            Err(_) => String::new(),
        };

        let new_entry = parsed.present;

        if last_entry != new_result {
            last_entry = new_result.clone();

            let mut log = Log {
                date: Utc::now().to_string(),
                data: new_entry.clone(),
                in_db: false,
            };
            if let Some(data) = new_entry {
                if self::db::add_log(Utc::now().naive_utc(), &data).is_some() {
                    log.in_db = true;
                }
            }
        }

        thread::sleep(time::Duration::from_secs(
            30 + rand::thread_rng().gen_range(0, 30),
        ));
    }
}

fn get_json(url: &str) -> Result<ApiResult, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url);

    match response {
        Ok(response) => {
            let body = response.text();
            match body {
                Ok(b) => {
                    let result: std::result::Result<ApiResult, serde_json::error::Error> =
                        serde_json::from_str(&b);

                    match result {
                        Ok(mut result) => {
                            result.unparsed = Some(b);
                            Ok(result)
                        }
                        Err(e) => Err(Box::new(e)),
                    }
                }
                Err(e) => Err(Box::new(e)),
            }
        }
        Err(e) => Err(Box::new(e)),
    }
}

fn from_api_result(api_result: &Result<ApiResult, Box<dyn std::error::Error>>) -> ParsedResult {
    let mut parsed = ParsedResult {
        past: None::<Song>,
        present: None::<Song>,
        future: None::<Song>,
    };
    if let Ok(api_result) = api_result {
        if let Some(past) = &api_result.past {
            let new = is_new(&past.title);
            let mut song = Song {
                artist: past.artist.clone(),
                title: remove_new(past.title.clone()),
                is_new: new,
            };
            fix_artist(&mut song);
            parsed.past = Some(song)
        }
        if let Some(present) = &api_result.present {
            let new = is_new(&present.title);
            let mut song = Song {
                artist: present.artist.clone(),
                title: remove_new(present.title.clone()),
                is_new: new,
            };
            fix_artist(&mut song);
            parsed.present = Some(song)
        }
        if let Some(future) = &api_result.future {
            let new = is_new(&future.title);
            let mut song = Song {
                artist: future.artist.clone(),
                title: remove_new(future.title.clone()),
                is_new: new,
            };
            fix_artist(&mut song);
            parsed.future = Some(song)
        }
    }
    parsed
}

fn fix_artist(song: &mut Song) {
    if song.artist.eq("MP") {
        let tmp = song.title.clone();
        let split = tmp.split(" - ").collect::<Vec<&str>>();
        if split.len() == 2 {
            let artist = split[0];
            let title = split[1];
            song.title = String::from(title);
            song.artist = String::from(artist);
        }
    }
    if song.title.ends_with('-') {
        song.title.truncate(song.title.len() - 1);
    }
}

fn remove_new(mut title: String) -> String {
    if title.contains("*NEU*") {
        title.truncate(title.len() - 1 - 4 - 1);
    } else if title.contains("* NEU*") {
        title.truncate(title.len() - 1 - 5 - 1);
    }
    title
}

fn is_new(title: &str) -> bool {
    title.contains("*NEU*") || title.contains("* NEU*")
}
