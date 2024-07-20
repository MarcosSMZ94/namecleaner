use namecleaner::*;
use std::{fs, process};

fn main() {
    // Directory path
    let path = r"C:\Users\marco\DownloadsTeste";

    // Reading the directory path
    let dir_paths = fs::read_dir(&path).unwrap();

    // Storing the file names
    let find_files = match Files::find_files(dir_paths) {
        Ok(t) => t,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    // Creating a struct with file names and the path
    let struct_file = match Files::new(path, &find_files) {
        Ok(t) => t,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    // Renaming
    let _renamer = match Files::renamer(struct_file) {
        Ok(t) => t,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };
}
