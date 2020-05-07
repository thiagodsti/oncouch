use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

pub fn find_files(dir: &Path, extensions: Vec<&str>) -> io::Result<Vec<DirEntry>> {
    let mut list = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                if !extensions.is_empty() && entry.path().extension().is_some() && extensions.contains(&entry.path().extension().unwrap().to_str().unwrap()) {
                    list.push(entry);
                }
            }
        }
    }
    return Ok(list);
}