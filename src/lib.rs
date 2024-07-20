use regex::Regex;
use std::{error::Error, fs};
//Struct containing file path and file name
pub struct Files<'a> {
    pub file_path: &'a str,
    pub file_name: &'a Vec<String>,
}

impl<'a> Files<'a> {
    // Create a struct with path and all file names
    pub fn new(path: &'a str, vec: &'a Vec<String>) -> Result<Files<'a>, Box<dyn Error>> {
        let file = Files {
            file_path: path,
            file_name: vec,
        };

        Ok(file)
    }
    // Find the file names and store on a vector
    pub fn find_files(path: fs::ReadDir) -> Result<Vec<String>, Box<dyn Error>> {
        let files = path
            .filter_map(|entry| {
                entry
                    .ok()
                    .and_then(|dir_entry| match dir_entry.path().is_file() {
                        true => dir_entry
                            .path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .map(|s| String::from(s)),
                        false => None,
                    })
            })
            .collect::<Vec<String>>();

        Ok(files)
    }
    // Renaming process
    pub fn renamer(files: Files) -> Result<(), Box<dyn Error>> {
        let path = files.file_path;

        let names = files.file_name;

        let re = Regex::new(r"\s(\(|\[)[^\)\]]*(\)|\])").unwrap();

        for name in names.into_iter() {
            let file_name = path.to_string() + "\\" + name;

            let replaced = re.replace_all(&name, "");

            let renamed = path.to_string() + "\\" + replaced.as_ref();

            println!("De {file_name} para {renamed}");

            fs::rename(&file_name, &renamed).unwrap();
        }
        Ok(())
    }
}
