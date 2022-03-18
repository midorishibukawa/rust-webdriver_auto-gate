use thirtyfour::prelude::*;
use tokio;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Opts {
    profile: String,
    base_url: String,
    username: String,
    password: String,
    timeout: u32,
    csv_file_path: String,
    error_message: String,
    driver_opts: Vec<String>
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    println!("loading options file...");
    let file_str: String = fs::read_to_string("data/options.json").expect("unable to read file!");
    
    let opts: Opts = serde_json::from_str(&file_str).expect("unable to parse file!");

    println!("{:?}", opts);

    Ok(())
}