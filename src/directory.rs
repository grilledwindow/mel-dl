use std::{fs, io};
use std::path::Path;

pub fn move_files(new: &Path) -> io::Result<()> {
    for entry in fs::read_dir("C:\\Users\\xa\\Desktop\\.mel-dl")? {
        let old = entry?.path();
        let new = new.join(old.file_name().unwrap());
        fs::rename(old, new)?;
    }
    Ok(())
}