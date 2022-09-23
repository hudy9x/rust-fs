use std::io::{self, Error, Result};
use std::fs::{self, ReadDir, metadata};
use std::path::Path;

fn read_directory(path: &str) -> ReadDir {
    let new_path = Path::new(path);
    let paths = fs::read_dir(new_path).unwrap();

    paths
}

fn read_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("ERROR");
    contents
}

// update file and create new file
fn write_file(path: &str, content: &str) -> Result<()> {
    let file_path = Path::new(path);
    fs::write(file_path, content);
    Ok(())
}

fn create_directory(path: &str) {
    let dir_path = Path::new(path);
    fs::create_dir(dir_path);
}

fn remove_file(path: &str) {
    let file_path = Path::new(path);
    fs::remove_file(file_path);
}

fn remove_folder(path: &str) {
    let folder_path = Path::new(path);
    fs::remove_dir_all(folder_path);
}

fn main() {

    // let paths = read_directory("E:\\dotfiles\\.config\\nvim");

    // for path in paths {
        // println!("Name: {}", path.unwrap().path().display())
    // }

    // let content = read_file("E:\\dotfiles\\.config\\nvim\\init.lua");

    // println!("content is: {content}");

    // write_file("E:\\test\\guessing_game\\src\\main.rs", "aloooo");
    // write_file("E:\\test\\guessing_game\\src\\new.rs", "aloooo2");

    // create_directory("E:\\test\\guessing_game\\src\\some");
    // remove_file("E:\\test\\guessing_game\\src\\new11.rs")

    remove_folder("E:\\test\\guessing_game\\src\\foo")

}
