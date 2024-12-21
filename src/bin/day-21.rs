use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

pub struct TempFile {
    pub file_path: PathBuf,
    pub file: File,
}

impl TempFile {

    pub fn new() -> Result<Self, std::io::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Back to the future!")
            .as_nanos();
        let file_path = PathBuf::from(format!("./tmp{now}"));
        let file = File::create_new(&file_path)?;
        Ok(Self { file_path, file })
    }

    pub fn write(&self, data: &[u8]) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new().write(true).open(&self.file_path)?;
        file.write(data).map(|_| ())
    }

    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buf: String = String::new();
        file.read_to_string(&mut buf).map(|_| buf)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }

}

impl Drop for TempFile {

    fn drop(&mut self) {
        _ = std::fs::remove_file(&self.file_path);
    }

}

fn main() {
    // Example usage
    let temp_file = TempFile::new().expect("Failed to create temp file");
    temp_file.write(b"Hello, world!").expect("Failed to write to temp file");
    let contents = temp_file.read_to_string().expect("Failed to read temp file");
    println!("Contents: {}", contents);
}