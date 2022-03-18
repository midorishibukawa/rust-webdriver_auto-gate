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
    csv_file_path: String,
    parcel_id: String,
    error_message: String,
    driver_args: Vec<String>,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // loads options file
    let opts: Opts = load_opts();

    // loads chrome options
    let caps: ChromeCapabilities = load_caps(opts.clone().driver_args);
    
    // starts new webdriver
    let driver: WebDriver = WebDriver::new("http://localhost:4444", &caps).await?;
    
    // navigates to url
    driver.get(opts.clone().base_url).await?;

    // checks if user is logged in
    let is_logged_in = !is_logged_in(&driver, opts.clone().base_url).await?;

    // if user is not logged in, logs user in
    if !is_logged_in {
        println!("not logged in!\n");
        login(&driver, opts.clone().username, opts.clone().password).await?;
    } else {
        println!("already logged in!\n")
    }
    
    // switches to correct tab
    switch_tab(&driver).await?;

    // searches parcel by id
    search_by_id(&driver, opts.parcel_id).await?;

    // driver.quit().await?;

    Ok(())
}

// loads options file
fn load_opts() -> Opts {
    println!("loading options file...\n");

    let file_str: String = fs::read_to_string("data/options.json").expect("unable to read file!");
    
    serde_json::from_str(&file_str).expect("unable to parse file!")
}

// loads chrome options
fn load_caps(driver_args: Vec<String>) -> ChromeCapabilities {
    println!("loading chrome options...\n");

    let mut caps: ChromeCapabilities = DesiredCapabilities::chrome();
    for arg in driver_args {
        caps.add_chrome_arg(&arg).expect("successfully added argument {:?}");
    }

    caps
}

// checks if user is logged in
// if not logged in, the website redirects to an auth0 login page
async fn is_logged_in(driver: &WebDriver, base_url: String) -> WebDriverResult<bool> {
    let url = driver.current_url().await?;

    Ok(!base_url.contains(&url))
}

// logs user in
async fn login(driver: &WebDriver, username: String, password: String) -> WebDriverResult<()> {
    println!("logging in...\n");

    let login_form: WebElement = driver.find_element(By::Tag("form")).await?;

    let username_input: WebElement = login_form.find_element(By::Id("username")).await?;
    let password_input: WebElement = login_form.find_element(By::Id("password")).await?;
    let form_submit_btn: WebElement = login_form.find_element(By::Tag("button[type=\"submit\"]")).await?;
    
    write(username_input, username).await?;
    write(password_input, password).await?;

    form_submit_btn.click().await?;

    Ok(())
}

// switches to correct tab
async fn switch_tab(driver: &WebDriver) -> WebDriverResult<()> {
    println!("switching to correct tab...\n");

    let tab: WebElement = driver.query(By::XPath("//span[text()=\"Sent and pending\"]/..")).first().await?;
    tab.click().await?;

    Ok(())
}

// searches parcel by id
async fn search_by_id(driver: &WebDriver, parcel_id: String) -> WebDriverResult<()> {
    println!("searching parcel by id...\n");

    let search_input: WebElement = driver.query(By::Tag("input[placeholder=\"Search by ID\"]")).first().await?;

    write(search_input, parcel_id.clone()).await?;

    let parcel_td: WebElement = driver.query(By::XPath(&format!("//td[text()=\"{}\"]", &parcel_id))).first().await?;

    parcel_td.click().await?;

    Ok(())
}

// clears input and writes
async fn write(input_elem: WebElement<'_>, input_text: String) -> WebDriverResult<()> {
    println!("writing {} to element {}", input_text, input_elem.element_id);

    input_elem.clear().await?;
    input_elem.send_keys(input_text).await?;

    Ok(())
}
