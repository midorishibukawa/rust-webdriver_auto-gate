use csv;
use thirtyfour::prelude::*;
use thirtyfour::common::capabilities::chrome::ChromeCapabilities;
use tokio;
use serde::{Serialize, Deserialize};
use serde_json;
use std::io::Error;
use std::fs;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Opts {
    profile: String,
    base_url: String,
    username: String,
    password: String,
    csv_path: String,
    post_id: String,
    error_message: String,
    driver_args: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ParcelInfo {
    receiver: String,
    description: String,
    code: String,
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // loads options file
    let opts: Opts = load_opts();

    // loads csv file into a hashmap
    let post_info: HashMap<String, Vec<ParcelInfo>> = load_csv(opts.clone().csv_path, opts.clone().post_id)?;
    println!("{:?}", post_info);

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
    search_by_id(&driver, opts.post_id).await?;

    loop {
        let parcel_list: Vec::<WebElement> = driver.query(
            By::XPath(
                &format!("//td[text()=\"{}\"]", opts.error_message)
            )
        ).all().await?;

        if parcel_list.len() == 0 {
            // if no parcel is fixable, find next button
            let next_btn = driver.find_element(By::ClassName("p-paginator-next")).await?;
            let next_btn_class = next_btn.class_name().await?.unwrap();

            // if next button is disabled, break loop
            if next_btn_class.contains("disabled") { break; }

            // if next button is not disabled, click it and restart loop
            next_btn.click().await?;
            continue;
        }

        // loads parcel info and code input element
        let (parcel_info, code_td): (ParcelInfo, WebElement) =  load_parcel_info(&driver, parcel_list.first().unwrap()).await?;
        let code: String = code_td.inner_html().await?;
        println!("{:?}", parcel_info);
        println!("{}", code);
    }

    // driver.quit().await?;

    Ok(())
}

// loads options file
fn load_opts() -> Opts {
    println!("loading options file...\n");

    let file_str: String = fs::read_to_string("data/options.json").expect("unable to read file!");
    
    serde_json::from_str(&file_str).expect("unable to parse file!")
}

// loads csv file into a hashmap
fn load_csv(csv_path: String, post_id: String) -> Result<HashMap<String, Vec<ParcelInfo>>, Error> {
    println!("loading csv file...\n");

    let mut rdr = csv::ReaderBuilder::new()
        .from_path(
            &format!("{}/{}.csv", &csv_path, &post_id)
        ).expect("couldn't read csv file!");

    let mut csv: HashMap<String, Vec<ParcelInfo>> = HashMap::new();

    for res in rdr.deserialize() {
        let parcel_info: ParcelInfo = res?;
        if !csv.contains_key(&parcel_info.receiver) {
            csv.insert(parcel_info.clone().receiver, Vec::new());
        }
        csv.get_mut(&parcel_info.receiver).unwrap().push(parcel_info);
    }
    
    Ok(csv)
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
async fn search_by_id(driver: &WebDriver, post_id: String) -> WebDriverResult<()> {
    println!("searching parcel by id...\n");

    let search_input: WebElement = driver.query(By::Tag("input[placeholder=\"Search by ID\"]")).first().await?;

    write(search_input, post_id.clone()).await?;

    let parcel_td: WebElement = driver.query(By::XPath(&format!("//td[text()=\"{}\"]", &post_id))).first().await?;

    parcel_td.click().await?;

    Ok(())
}

// loads parcel info and code input element
async fn load_parcel_info<'a>(driver: &'a WebDriver, parcel_td: &WebElement<'_>) -> WebDriverResult<(ParcelInfo, WebElement<'a>)> {
    parcel_td.click().await?;

    let parcel_info_tr: Vec<WebElement> = driver.query(
        By::XPath(
            "//span[@class=\"p-column-title\"][text()=\"Description\"]/../../../../tbody/tr"
        )
    ).all().await?;
    let receiver_td: WebElement = driver.query(
        By::XPath(
            "//div[@class=\"p-datatable-header\"][text()=\"Receiver\"]/..//td[text()=\"Sender\"]/following-sibling::*"
        )
    ).first().await?;

    let description: String = parcel_info_tr.first().unwrap().inner_html().await?;
    let receiver: String = receiver_td.inner_html().await?;
    let code: String = "".to_string();
    let code_td: WebElement = parcel_info_tr.last().unwrap().clone();

    Ok((
        ParcelInfo {
            description,
            receiver,
            code,
        },
        code_td,
    ))
}

// clears input and writes
async fn write(input_elem: WebElement<'_>, input_text: String) -> WebDriverResult<()> {
    println!("writing {} to element {}", input_text, input_elem.element_id);

    input_elem.clear().await?;
    input_elem.send_keys(input_text).await?;

    Ok(())
}
