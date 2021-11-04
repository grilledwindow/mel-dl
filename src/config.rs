use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct Module<'a> {
    // module name
    pub name: &'a str,

    // the order of the course on the website
    pub course_nth: u8,

    // first week appears on top, defaults to true
    #[serde(default = "default_true")]
    pub folder_order_ascending: bool,

    // img used, defaults to folder
    #[serde(default = "default_img_alt")]
    pub img_alt: &'a str,

    // when the folders start, defaults to 1
    #[serde(default = "default_week_start")]
    pub week_start: u8,

    // name of materials tab, defaults to "Learning Materials"
    #[serde(default = "default_materials")]
    pub materials: &'a str,

}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct SettingsRead<'a> {
    #[serde(default = "default_true")]
    pub order_ascending: bool,
    pub path: String,
    pub bin: String,
    pub temp_download_folder: &'a str,
}


#[derive(Debug)]
pub struct Settings<'a> {
    pub order_ascending: bool,
    pub path: &'a Path,
    pub bin: &'a Path,
    pub temp_download_folder: &'a Path,
}

impl<'a> Settings<'a> {
    pub fn from(sr: &'a SettingsRead) -> Self {
        Self {
            order_ascending: sr.order_ascending,
            path: &Path::new(&sr.path),
            bin: &Path::new(&sr.bin),
            temp_download_folder: &Path::new(sr.temp_download_folder),
        }
    }
}


fn default_true() -> bool {
    true
}

fn default_img_alt<'a>() -> &'a str {
    "folder"
}

fn default_week_start() -> u8 {
    0
}

fn default_materials<'a>() -> &'a str {
    "Learning Materials"
}