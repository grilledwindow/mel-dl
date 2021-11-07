use clap::{App, Arg};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;
use std::time::Duration;
use thirtyfour::prelude::*;
use thirtyfour_query::ElementPoller;
use tokio;

mod driver {
    pub mod download;
    mod utils;
}
use driver::download::*;

mod config;
use config::{Module, Settings, SettingsRead};

mod directory;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let settings_data = read_to_string("settings.json").expect("Error reading settings.json");
    let settings_read: SettingsRead =
        serde_json::from_str(&settings_data).expect("Error deserializing settings.json");
    let settings = Settings::from(&settings_read);
    let download_path = Path::new(settings.path).join(settings.temp_download_folder);

    let modules_data = read_to_string("modules.json").expect("Error reading modules.json");
    let modules_vec: Vec<Module> =
        serde_json::from_str(&modules_data).expect("Error deserializing modules.json");

    let mut modules = HashMap::<&str, Module>::new();
    for module in modules_vec {
        modules.entry(module.name).or_insert(module);
    }

    const URL: &str = "https://mel.np.edu.sg/auth-saml/saml/login?apId=_155_1&redirectUrl=https://mel.np.edu.sg/ultra";

    let mut caps = DesiredCapabilities::chrome();

    // specify the temporary download location.
    // the files will be moved into a specific folder later.
    let v: Value = serde_json::json!({
        "download.prompt_for_download": false,
        "download.default_directory": download_path.to_string_lossy(),
    });

    caps.add_chrome_option("prefs", v)?;

    // specify the binary path
    caps.add_chrome_option("binary", settings.bin)?;

    let matches = App::new("mel-dl")
        .version("0.1.0")
        .author("Xavier Ang <xaveang@gmail.com>")
        .about("Teaches argument parsing")
        .arg(
            Arg::with_name("module")
                .short("m")
                .long("module")
                .takes_value(true)
                .help("Module to download"),
        )
        .arg(
            Arg::with_name("folder")
                .short("f")
                .long("folder-no")
                .takes_value(true)
                .help("Folder to download"),
        )
        .get_matches();

    let module_str = matches.value_of("module").unwrap_or("");
    if !modules.contains_key(module_str) {
        panic!("Module: {} not found", module_str);
    }
    let module = &modules[module_str];
    println!("Module: {}", module_str);

    let folder = if let Some(folder_str) = matches.value_of("folder") {
        let folder = folder_str.parse::<u8>().expect("Folder must be a number!");
        println!("Folder: {}", folder);
        folder
    } else {
        println!("No folder specified, defaulting to 1");
        1
    };

    let mut driver = WebDriver::new("http://localhost:4444", &mut caps).await?;

    driver
        .set_implicit_wait_timeout(Duration::new(0, 0))
        .await?;

    let poller =
        ElementPoller::TimeoutWithInterval(Duration::new(20, 0), Duration::from_millis(800));
    driver.config_mut().set("ElementPoller", poller)?;

    driver.get(URL).await?;

    driver.sign_in().await?;

    driver.download_files(&module, folder, &settings).await?;

    Ok(())
}
