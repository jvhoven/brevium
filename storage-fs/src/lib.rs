use core::{storage::Saveable, storage::Storage};
use std::{
    fs::{create_dir_all, File},
    io::Write,
};

#[derive(Debug)]
pub struct FileStorage {
    dir_path: String,
}

impl FileStorage {
    pub fn new() -> Result<Self, String> {
        println!("Now using FileStorage");
        // TODO: base directory should come from the config
        if !std::path::Path::new("/tmp/brevium").is_dir() {
            create_dir_all("/tmp/brevium")
                .map_err(|e| format!("failed to create directory: {}", e))?;
        }

        Ok(Self {
            dir_path: String::from("/tmp/brevium"),
        })
    }
}

impl Storage for FileStorage {
    fn add_item(&self, item: &impl Saveable) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = format!("{}/{}.txt", self.dir_path, "test");
        let mut file = File::create(file_path)?;
        file.write_all(item.get_content().as_bytes())?;

        Ok(())
    }
}
