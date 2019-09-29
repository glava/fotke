use std::io;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};


pub fn image_paths(path: &str) -> Vec<PathBuf> {
    let path_string = String::from(path);
    let current_dir = Path::new(&path_string);
    let mut v = Vec::new();
    visit_dirs(&current_dir, &mut |entry: &DirEntry| { 
        let path = entry.path();
        let ext = path.extension().unwrap();
        if ext == "jpg" {
            &v.push(entry.path()); 
        }
    }).unwrap();
    return v;
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}
