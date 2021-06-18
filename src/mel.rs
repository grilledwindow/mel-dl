use std::path::Path;

pub enum Modules {
    AppliedAnalytics,
    WebAppDev,
    MobileAppDev,
    Calculus,
}

pub struct Module {
    pub module: Modules,
    pub course_nth: u8,
    pub learning_materials_tab_id: String,
    pub learning_materials_nth: u8,
    pub download_structure: &'static[&'static str],
    pub download_path: &'static Path,
}

impl Module {
    pub fn new(
        module: Modules,
        course_nth: u8,
        learning_materials_tab_id: &'static str,
        learning_materials_nth: u8,
        download_structure: &'static[&'static str],
        download_path: &'static Path,
    ) -> Self {
        Self {
            module,
            course_nth,
            learning_materials_tab_id: format!("paletteItem:_{}_1", learning_materials_tab_id),
            learning_materials_nth,
            download_structure,
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
