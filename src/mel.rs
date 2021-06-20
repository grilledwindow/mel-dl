use std::path::Path;

pub struct Module {
    pub course_nth: u8,
    pub learning_materials_tab_id: String,
    pub learning_materials_nth: u8,
    pub max_depth: u8,
    pub download_path: &'static Path,
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
