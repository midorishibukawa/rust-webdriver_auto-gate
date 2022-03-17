use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:4444", &caps).await?;

    driver.get("https://wikipedia.org").await?;
    
    let elem_form = driver.find_element(By::Id("search-form")).await?;
    let elem_text = elem_form.find_element(By::Id("searchInput")).await?;

    elem_text.send_keys("selenium").await?;

    let elem_button = elem_form.find_element(By::Css("button[type='submit']")).await?;
    
    elem_button.click().await?;

    driver.find_element(By::ClassName("firstHeading")).await?;
    assert_eq!(driver.title().await?, "Selenium - Wikipedia");

    driver.quit().await?;

    Ok(())
}