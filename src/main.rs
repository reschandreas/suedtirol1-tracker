use serde::Deserialize;
use std::io::Write;
use std::fs::OpenOptions;
use std::{thread, time};

#[derive(Debug, Deserialize)]
struct Song {
    artist: String,
    title: String
}

#[derive(Debug, Deserialize)]
struct ApiResult {
    past: Song,
    present: Option<Song>,
    future: Song
}

fn main() {
    let mut result = String::from("");
    loop {
        let re = get_json("http://www.suedtirol1.it/routing/acc_fun_interaktiv/api/v1/playlist/index.json?v=1").unwrap();
        if re != result {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("/var/tmp/suedtirol1")
                .unwrap();

            if let Err(e) = writeln!(file, "{}", re) {
                eprintln!("Couldn't write to file: {}", e);
            }
            result = re;
        }
        thread::sleep(time::Duration::from_millis(1000 * 60));
    }
}

fn get_json(url: &str) -> Result<String, Box <dyn std::error::Error>> {
    let body = reqwest::blocking::get(url)?.text()?;
    let res : ApiResult = serde_json::from_str(&body).expect("JSON was not well-formatted");
    println!("{:?}", res);
    Ok(body)
}
