use thirtyfour::prelude::*;
use thirtyfour::common::capabilities::chrome::ChromeCapabilities;
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
    driver_args: Vec<String>,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // loads options file
    let opts: Opts = load_opts();
    println!("{:?}\n", opts);

    // loads chrome options
    let caps: ChromeCapabilities = load_caps(opts.clone());
    println!("{:?}\n", caps);
    
    // start new webdriver
    let driver: WebDriver = WebDriver::new("http://localhost:4444", &caps).await?;
    
    // navigate to url
    driver.get(opts.clone().base_url).await?;

    // check if user is logged in
    let is_logged_in = !is_logged_in(&driver, opts.clone()).await?;

    if !is_logged_in {
        println!("not logged in!\n");
    } else {
        println!("logged in!\n")
    }

    Ok(())
}

// loads options file
fn load_opts() -> Opts {
    println!("loading options file...\n");

    let file_str: String = fs::read_to_string("data/options.json").expect("unable to read file!");
    
    serde_json::from_str(&file_str).expect("unable to parse file!")
}

// loads chrome options
fn load_caps(opts: Opts) -> ChromeCapabilities {
    println!("loading chrome options...\n");

    let mut caps: ChromeCapabilities = DesiredCapabilities::chrome();
    for opt in opts.driver_args {
        caps.add_chrome_arg(&opt).expect("successfully added argument {:?}");
    }

    caps
}

// check if user is logged in
// if not logged in, the website redirects to an auth0 login page
async fn is_logged_in(driver: &WebDriver, opts: Opts) -> WebDriverResult<bool> {
    let url = driver.current_url().await?;

    Ok(!opts.base_url.contains(&url))
}
