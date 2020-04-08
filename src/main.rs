use chrono::Utc;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::{thread, time};

#[derive(Debug, Deserialize, Serialize)]
struct Log {
    date: String,
    data: Option<Song>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Song {
    artist: String,
    title: String,
    is_new: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ParsedResult {
    past: Option<Song>,
    present: Option<Song>,
    future: Option<Song>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ApiResult {
    past: Option<ApiSong>,
    present: Option<ApiSong>,
    future: Option<ApiSong>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ApiSong {
    artist: String,
    title: String,
}

fn main() {
    let mut last_entry = String::new();
    loop {
        let api_result = get_json(
            "http://www.suedtirol1.it/routing/acc_fun_interaktiv/api/v1/playlist/index.json?v=1",
        );
        let parsed = from_api_result(api_result);
        
        let new_entry = parsed.present;

        let current_title = match &new_entry {
            Some(new_entry) => new_entry.title.clone(),
            None => String::new()
        };

        if  last_entry != current_title {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("/var/tmp/suedtirol1")
                .unwrap();

            last_entry = current_title.clone();

            let log = Log {
                date: Utc::now().to_string(),
                data: new_entry,
            };

            if let Err(e) = writeln!(file, "{}", serde_json::to_string(&log).unwrap()) {
                eprintln!("Couldn't write to file {}", e);
            }
        }

        thread::sleep(time::Duration::from_secs(30 + rand::thread_rng().gen_range(0, 30)));
    }
}

fn get_json(url: &str) -> Result<ApiResult, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url);

    match response {
        Ok(response) => {
            let body = response.text();
            match body {
                Ok(b) => {
                    let result = serde_json::from_str(&b);

                    match result {
                        Ok(result) => return Ok(result),
                        Err(e) => return Err(Box::new(e)),
                    }
                }
                Err(e) => return Err(Box::new(e)),
            }
        }
        Err(e) => return Err(Box::new(e)),
    }
}

fn from_api_result(api_result: Result<ApiResult, Box<dyn std::error::Error>>) -> ParsedResult {
    let mut parsed = ParsedResult {
        past: None::<Song>,
        present: None::<Song>,
        future: None::<Song>,
    };

    match api_result {
        Ok(api_result) => {
            match api_result.past {
                Some(past) => {
                    let new = is_new(&past.title);
                    parsed.past = Some(Song {
                        artist: past.artist,
                        title: if new {
                            remove_new(past.title)
                        } else {
                            past.title
                        },
                        is_new: new,
                    })
                }
                None => {}
            }
            match api_result.present {
                Some(present) => {
                    let new = is_new(&present.title);
                    parsed.present = Some(Song {
                        artist: present.artist,
                        title: if new {
                            remove_new(present.title.clone())
                        } else {
                            present.title
                        },
                        is_new: new,
                    })
                }
                None => {}
            }
            match api_result.future {
                Some(future) => {
                    let new = is_new(&future.title);
                    parsed.future = Some(Song {
                        artist: future.artist,
                        title: if new {
                            remove_new(future.title.clone())
                        } else {
                            future.title
                        },
                        is_new: new,
                    })
                }
                None => {}
            }
        }
        Err(_) => {}
    }
    parsed
}

fn remove_new(mut title: String) -> String {
    title.split_off(title.len() - 1 - 4 - 1);
    title
}

fn is_new(title: &String) -> bool {
    let regex = Regex::new(r"\*NEU\*").unwrap();

    if regex.is_match(title) {
        true
    } else {
        false
    }
}
