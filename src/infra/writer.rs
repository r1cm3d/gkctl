use crate::types::Writer;
use std::io::Write;
use std::fs::File;
use std::fs;
use std::path::PathBuf;
use log::info;

pub struct Fs {}

impl Writer for Fs {

    fn write(&self, file_name: &str, content: &str) -> Option<Box<dyn std::error::Error>> {
        info!("Extracting path of ({}).", file_name);
        let path = PathBuf::from(file_name);
        let dir = path.parent()?;

        info!("Creating directory ({}).", dir.to_str()?);
        fs::create_dir_all(dir).ok()?;


        info!("Writing File ({}).", file_name);
        let mut file = File::create(file_name).ok()?;
        file.write_all(content.as_bytes()).ok()?;

        None
    }
}
