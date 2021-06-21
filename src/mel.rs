use std::path::Path;

pub struct Module {
    pub course_nth: u8,                    // the order of the course on the website
    pub learning_materials_tab_id: String, // the tab id for the course's learning materials tab
    pub learning_materials_nth: u8,        // the order of the first week's learning materials
    pub max_depth: u8,                     // the maximum depth to search for files within folders
    pub download_path: &'static Path,      // the download path
}

impl Module {
    pub fn new(
        course_nth: u8,
        learning_materials_tab_id: &'static str,
        learning_materials_nth: u8,
        max_depth: u8,
        download_path: &'static Path,
    ) -> Self {
        Self {
            course_nth,
            learning_materials_tab_id: format!("paletteItem:_{}_1", learning_materials_tab_id),
            learning_materials_nth,
            max_depth,
            download_path,
        }
    }
}
/*
    MeL
    Login
    Course
    Learning Materials
    Download
    Print downloaded files
*/
