extern crate rofi;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::ffi::OsStr;
extern crate glob;
use glob::glob;


fn main() {
    let entries = generatelist("/home/thepenguin/Documents/Books");
    display(entries);
    }

fn display(dir_entries:std::vec::Vec<std::string::String>){
    match rofi::Rofi::new(&dir_entries).prompt("Schedule").run() {
        Ok(_choice) => displayOption(&_choice),
        Err(rofi::Error::Interrupted) => (),
        Err(e) => println!("Error, {}", e)
    }
}

fn displayOption(path: &str){
    match get_extension_from_filename(&path) {
        Some("pdf") => {
            Command::new("zathura")
                .args(&[&path])
                .spawn()
                .expect("Failed to start");
            },
        _ => {
            println!("{}", path.to_string());
            display(generatelist(path));
        },
        }
}

fn generatelist(path: &str) -> std::vec::Vec<std::string::String>{
    let mut temp: Vec<std::string::String> = Vec::new();
    for file in glob(&(path.to_string() + "/*")).expect("Failed to find") {
        temp.push(file.unwrap().display().to_string())
    }
    return temp;
    
}

fn get_extension_from_filename(filename: &str) -> Option<&str> {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
}
