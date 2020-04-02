use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::{thread, time};

#[derive(Debug, Deserialize, Serialize)]
struct Log {
    date: String,
    data: ApiResult,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Song {
    artist: String,
    title: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ApiResult {
    past: Song,
    present: Option<Song>,
    future: Song,
}

fn main() {
    let mut result = String::from("");
    loop {
        let re = get_json(
            "http://www.suedtirol1.it/routing/acc_fun_interaktiv/api/v1/playlist/index.json?v=1",
        )
        .unwrap();
        if re.past.title != result {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("/var/tmp/suedtirol1")
                .unwrap();
            result = re.past.title.clone();
            let log = Log {
                date: Utc::now().to_string(),
                data: re,
            };

            if let Err(e) = writeln!(file, "{}", serde_json::to_string(&log).unwrap()) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
        thread::sleep(time::Duration::from_millis(1000 * 60));
    }
}

fn get_json(url: &str) -> Result<ApiResult, Box<dyn std::error::Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    let res: ApiResult = serde_json::from_str(&body).expect("JSON was not well-formatted");
    Ok(res)
}
