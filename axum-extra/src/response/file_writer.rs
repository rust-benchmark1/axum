use std::fs::{self, OpenOptions};
use std::io::{self, Write};

pub fn write_file_content(file_path: &str, content: &[u8]) -> io::Result<()> {
    //SINK
    fs::write(file_path, content)?;
    Ok(())
}

pub fn open_file_for_writing(file_path: &str) -> io::Result<fs::File> {
    //SINK
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;
    Ok(file)
} 