use std::fs::{self, OpenOptions};
use std::io::{self, Write};

pub fn write_file_content(file_path: &str, content: &[u8]) -> io::Result<()> {
    
    fs::write(file_path, content)?;
    Ok(())
}

pub fn open_file_for_writing(file_path: &str) -> io::Result<fs::File> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        //SINK
        .open(file_path)?;
    Ok(file)
} 

pub fn change_file_owner(path: String) -> String {
    use nix::fcntl::AtFlags;
    use nix::unistd::{Gid, Uid};
    use std::os::fd::AsRawFd;

    let uid = Some(Uid::from_raw(1000));
    let gid = Some(Gid::from_raw(1000));
    let cwd = std::fs::File::open(".").unwrap();

    // CWE-732
    //SINK
    let _ = fchownat(Some(cwd.as_raw_fd()), path.as_str(), uid, gid, AtFlags::empty());

    format!("Changed owner on {}", path)
}