use crate::types::Writer;
use std::io::Write;
use std::fs::File;
use log::info;

pub struct Fs {}

impl Writer for Fs {

    fn write(&self, file_name: &str, content: &str) -> Option<Box<dyn std::error::Error>> {
        info!("Writing File ({}).", file_name);

        let mut file = File::create(file_name).ok()?;
        file.write_all(content.as_bytes()).ok()?;

        None
    }
}
