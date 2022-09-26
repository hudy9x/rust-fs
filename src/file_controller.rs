
use std::fs::{self, metadata, DirEntry, ReadDir};
use std::io::{self, Error, Result};
use std::path::Path;
use std::ffi::OsString;

#[warn(dead_code)]
#[derive(Debug)]
pub struct FileInfo {
    name: OsString,
    kind: String,
    path: String,
}

pub fn read_directory(path: String) -> Vec<FileInfo> {
    let new_path = Path::new(&path as &str);
    let paths = fs::read_dir(new_path).unwrap();

    let mut files: Vec<FileInfo> = vec![];

    for path in paths {
        let path_unwrap = path.unwrap();
        let meta = path_unwrap.metadata();
        let meta_unwrap = meta.unwrap();

        let mut kind = String::from("file");

        if meta_unwrap.is_dir() {
            kind = String::from("directory");
        }

        let new_file_info = FileInfo {
            name: path_unwrap.file_name(),
            kind,
            path: String::from("test-directoryu") 
        };

        files.push(new_file_info);
    }

    files
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

fn main() {
    // let paths = read_directory("E:\\test");


    // for path in paths {
    // println!("Name: {}", path.unwrap().path().display())
    // }

    // let content = read_file("E:\\dotfiles\\.config\\nvim\\init.lua");

    // println!("content is: {content}");

    // write_file("E:\\test\\guessing_game\\src\\main.rs", "aloooo");
    // write_file("E:\\test\\guessing_game\\src\\new.rs", "aloooo2");

    // create_directory("E:\\test\\guessing_game\\src\\some");
    // remove_file("E:\\test\\guessing_game\\src\\new11.rs")

    // remove_folder("E:\\test\\guessing_game\\src\\foo")
}
