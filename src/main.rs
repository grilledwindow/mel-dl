use serde_json::Value;
use std::path::Path;
use std::time::Duration;
use thirtyfour::prelude::*;
use thirtyfour_query::ElementPoller;
use tokio;

mod mel;
use mel::*;

mod driver {
    pub mod download;
    mod utils;
}
use driver::download::*;

mod config;
use config::Config;

mod login;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    let v: Value = serde_json::from_str(
        r#"{
        "download.default_directory": "C:\\Users\\xa\\Downloads",
        "download.prompt_for_download": false
    }"#,
    )?;
    caps.add_chrome_option("prefs", v)?;

    caps.add_chrome_option(
        "binary",
        "C:\\Program Files (x86)\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
    )?;

    const URL: &str = "https://mel.np.edu.sg/auth-saml/saml/login?apId=_155_1&redirectUrl=https://mel.np.edu.sg/ultra";

    let mut driver = WebDriver::new("http://localhost:4444", &mut caps).await?;

    driver
        .set_implicit_wait_timeout(Duration::new(0, 0))
        .await?;

    let poller =
        ElementPoller::TimeoutWithInterval(Duration::new(20, 0), Duration::from_millis(800));
    driver.config_mut().set("ElementPoller", poller)?;

    driver.get(URL).await?;

    driver.sign_in().await?;

    let applied_analytics = Module::new(1, 1, Path::new("C:\\Users\\xa\\Desktop\\AA"), true);
    let config = Config::new().set_folder_no(None);
    
    driver.download_files(&applied_analytics, &config).await?;

    Ok(())
}
