use std::path::Path;
use std::fs::{self, File};
use std::io;

pub const PALIT_PROJECTS: &str = "/usr/local/palit/";
pub const PALIT_MODULES: &str = "/usr/local/palit/modules/";

pub fn get_files(path: &str, mut collection: Vec<String>) -> io::Result<Vec<String>> {
    let dir = Path::new(path);
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                collection.push(entry.file_name().into_string().unwrap());
            }
        }
    }
    Ok(collection)
}