use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;
use std::path::Path;

// #[warn(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    name: String,
    kind: String,
    path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Post {
    title: String,
    created: String,
    link: String,
    description: String,
    content: String,
    author: String,
}

pub fn read_directory(path: String) -> String {
    let new_path = Path::new(&path as &str);
    let paths = fs::read_dir(new_path).unwrap();

    let mut files: Vec<FileInfo> = Vec::new();

    for path in paths {
        let path_unwrap = path.unwrap();
        let meta = path_unwrap.metadata();
        let meta_unwrap = meta.unwrap();

        let mut kind = String::from("file");

        if meta_unwrap.is_dir() {
            kind = String::from("directory");
        }

        let filename = match path_unwrap.file_name().into_string() {
            Ok(str) => str,
            Err(error) => String::from("ERROR"),
        };

        let new_file_info = FileInfo {
            name: filename,
            kind,
            path: String::from("test-directoryu"),
        };

        files.push(new_file_info);
    }

    let files_str = match serde_json::to_string(&files) {
        Ok(str) => str,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // println!("file {:?}", files_str);

    files_str
}

pub fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("ERROR");
    contents
}

// update file and create new file
pub fn write_file(path: &str, content: &str) -> Result<()> {
    let file_path = Path::new(path);
    fs::write(file_path, content);
    Ok(())
}

pub fn create_directory(path: &str) {
    let dir_path = Path::new(path);
    fs::create_dir(dir_path);
}

pub fn remove_file(path: &str) {
    let file_path = Path::new(path);
    fs::remove_file(file_path);
}

pub fn remove_folder(path: &str) {
    let folder_path = Path::new(path);
    fs::remove_dir_all(folder_path);
}
