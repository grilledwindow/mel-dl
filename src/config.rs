use std::default::Default;

#[allow(dead_code)]
pub struct Config {
    prompt: bool,
    folder_no: u8,
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn get_prompt(&self) -> bool {
        self.prompt
    }

    pub(crate) fn set_prompt(mut self, enabled: bool) -> Self {
        self.prompt = enabled;
        self
    }

    pub fn current(&self) -> bool {
        self.folder_no == 0
    }

    pub fn get_folder_no(&self) -> u8 {
        self.folder_no
    }

    pub(crate) fn set_folder_no(mut self, folder_no: u8) -> Self {
        self.folder_no = folder_no;
        self
    }

}

impl Default for Config {
    fn default() -> Self {
        Self {
            prompt: false,
            folder_no: 0,
        }
    }
}