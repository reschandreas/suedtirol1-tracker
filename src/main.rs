#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::models::{ApiResult, Log, ParsedResult, Song};
use chrono::Utc;
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;
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
            Ok(api) => api.unparsed,
            Err(_) => String::new(),
        };

        let new_entry = parsed.present;

        if last_entry != new_result {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("/var/tmp/suedtirol1")
                .unwrap();

            last_entry = new_result.clone();

            let mut log = Log {
                date: Utc::now().to_string(),
                data: new_entry.clone(),
                in_db: false,
            };
            if let Some(data) = new_entry {
                if self::db::add_log(Utc::now().naive_utc(), data).is_some() {
                    log.in_db = true;
                }
            }
            if let Err(e) = writeln!(file, "{}", serde_json::to_string(&log).unwrap()) {
                eprintln!("Couldn't write to file {}", e);
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
                            result.unparsed = b;
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
            parsed.past = Some(Song {
                artist: past.artist.clone(),
                title: if new {
                    remove_new(past.title.clone())
                } else {
                    past.title.clone()
                },
                is_new: new,
            })
        }
        if let Some(present) = &api_result.present {
            let new = is_new(&present.title);
            parsed.present = Some(Song {
                artist: present.artist.clone(),
                title: if new {
                    remove_new(present.title.clone())
                } else {
                    present.title.clone()
                },
                is_new: new,
            })
        }
        if let Some(future) = &api_result.future {
            let new = is_new(&future.title);
            parsed.future = Some(Song {
                artist: future.artist.clone(),
                title: if new {
                    remove_new(future.title.clone())
                } else {
                    future.title.clone()
                },
                is_new: new,
            })
        }
    }
    parsed
}

fn remove_new(mut title: String) -> String {
    title.split_off(title.len() - 1 - 4 - 1);
    title
}

fn is_new(title: &str) -> bool {
    title.contains("*NEU*")
}
