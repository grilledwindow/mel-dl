use std::path::Path;

pub struct Module {
    pub course_nth: u8,                    // the order of the course on the website
    pub max_depth: u8,                     // maximum depth to search for files within folders
    pub download_path: &'static Path,      // download path
    pub folder_order_ascending: bool,      // first week appears on top
}

impl Module {
    pub fn new(
        course_nth: u8,
        max_depth: u8,
        download_path: &'static Path,
        folder_order_ascending: bool,
    ) -> Self {
        Self {
            course_nth,
            max_depth,
            download_path,
            folder_order_ascending,
        }
    }
}