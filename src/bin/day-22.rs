use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{read_to_string, remove_file, write};
use std::fs::File;
use core::str;
pub struct TempFile<'a> {
    file_path: PathBuf,
    file: File,
    content: &'a str,
}
impl<'a> TempFile<'a> {
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "duration error"))?
            .as_nanos();
        let process_id = std::process::id();
        let file_name = format!("/tmp/example_{}_{}", now, process_id);
        let file = File::create(&file_name)?;
        Ok(TempFile {
            file_path: PathBuf::from(file_name),
            file,
            content: "",
        })
    }
    pub fn write(&mut self, data: &'a [u8]) -> Result<(), std::io::Error> {
        // Your code here...
        write(&self.file_path, data)?;
        self.content = str::from_utf8(data)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "invalid"))?;
        Ok(())
    }
    pub fn read_from_cache(&self) -> &str {
        // 3. Update this method to return the content as a string slice
        self.content
    }
    pub fn read_to_string(&self) -> Result<String, std::io::Error> {
        // Your code here...
        read_to_string(&self.file_path)
    }
    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }
    pub fn file(&self) -> &File {
        &self.file
    }
}
impl<'a> Drop for TempFile<'a> {
    fn drop(&mut self) {
        let _ = remove_file(&self.file_path);
    }
}

fn main() -> Result<(), std::io::Error> {
    // Create a new temporary file
    let mut temp_file = TempFile::new()?;

    // Data to write to the temporary file
    let data = b"Hello, world!";

    // Write data to the temporary file
    temp_file.write(data)?;

    // Read the content from the cache
    let cached_content = temp_file.read_from_cache();
    println!("Cached content: {}", cached_content);

    // Read the content from the file
    let file_content = temp_file.read_to_string()?;
    println!("File content: {}", file_content);

    // Print the path of the temporary file
    println!("Temporary file path: {:?}", temp_file.path());

    // The temporary file will be automatically deleted when `temp_file` goes out of scope
    Ok(())
}
