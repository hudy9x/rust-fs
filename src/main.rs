mod file_controller; 

fn main() {
    
    let paths = file_controller::read_directory("E:\\test\\rust-gui");

    println!("paths: {:?}", paths);


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
