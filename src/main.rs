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
    is_new: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ApiResult {
    past: Option<Song>,
    present: Option<Song>,
    future: Option<Song>,
}

fn main() {
    let regex = Regex::new(r"\*NEU\*").unwrap();
    let mut result = String::from("");
    loop {
        let re = get_json(
            "http://www.suedtirol1.it/routing/acc_fun_interaktiv/api/v1/playlist/index.json?v=1",
        );
        match re {
            Ok(mut re) => {
                let future = re.clone().future.unwrap().title;
                if re.past.is_none() && re.present.is_none() {
                    re.present = re.future.clone();
                }
                if future != result {
                    let mut copy = re.present.clone().unwrap();

                    copy.is_new = Some(if regex.is_match(&copy.title) {
                        copy.title.split_off(copy.title.len() - 1 - 4);
                        true
                    } else {
                        false
                    });

                    let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open("/var/tmp/suedtirol1")
                        .unwrap();
                    result = future.clone();
                    let log = Log {
                        date: Utc::now().to_string(),
                        data: Some(copy),
                    };
                    if let Err(e) = writeln!(file, "{}", serde_json::to_string(&log).unwrap()) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                }
            }
            Err(_) => {}
        }
        thread::sleep(time::Duration::from_millis(
            60000 + rand::thread_rng().gen_range(0, 30000),
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
