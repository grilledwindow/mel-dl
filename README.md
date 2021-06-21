# mel-dl
This project aims to automate the downloading of weekly files from my school's website [MeL](https://mel.np.edu.sg). It is somewhat working but there are still things to be worked on.
## Prerequisites
You need to have [Rust](https://www.rust-lang.org/tools/install), [Chromedriver](https://chromedriver.chromium.org/downloads) and a Chromium-based browser installed.

## Setup
Run the chromedriver:
`chromedriver --port=4444`

Build and run (in a separate tab):
`cargo run`

## Program
struct Module:
```
pub struct Module {
    pub course_nth: u8,                    // the order of the course on the website
    pub learning_materials_tab_id: String, // the tab id for the course's learning materials tab
    pub learning_materials_nth: u8,        // the order of the first week's learning materials
    pub max_depth: u8,                     // the maximum depth to search for files within folders
    pub download_path: &'static Path,      // the download path
}
```

Create a Module:
`let applied_analytics = Module::new(1, "694428", 1, 2, Path::new("C:\\Users\\xa\\Desktop\\AA"));`

Download files:
`driver.download_files(&applied_analytics, 2).await?;`

## Configuration
Download preferences:
```
let v: Value = serde_json::from_str(
    r#"{
    "download.default_directory": "C:\\Users\\xa\\Downloads",
    "download.prompt_for_download": false
}"#,
)?;
caps.add_chrome_option("prefs", v)?;
```

Browser binary location:
```
caps.add_chrome_option(
    "binary",
    "C:\\Program Files (x86)\\BraveSoftware\\Brave-Browser\\Application\\brave.exe",
)?;
```