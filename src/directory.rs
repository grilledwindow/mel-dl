use std::{fs, io};

use super::Settings;

pub fn move_files(settings: &Settings, folder: &str, week: &str) -> io::Result<()> {
    let mut week_path = settings.path.join(folder);
    if let Err(_) = fs::read_dir(&week_path) {
        println!(
            "Directory not found: {0:?}\nCreating directory: {0:?}",
            week_path
        );
        fs::create_dir(&week_path)?
    };
    week_path = week_path.join(week);
    if let Err(_) = fs::read_dir(&week_path) {
        println!(
            "Directory not found: {0:?}\nCreating directory:  {0:?}",
            week_path
        );
        fs::create_dir(&week_path)?
    };

    println!("\nMoving files to: {:?}", week_path);

    let temp_download_path = settings.path.join(settings.temp_download_folder);
    for entry in fs::read_dir(temp_download_path)? {
        let old = entry?.path();
        let new = week_path.join(old.file_name().unwrap());

        println!("Moving {:?}\n    to {:?}", old, new);
        fs::rename(old, new)?;
    }
    Ok(())
}
