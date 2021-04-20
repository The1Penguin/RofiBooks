extern crate rofi;
extern crate regex;
use regex::Regex;
use std::path::Path;
use std::process::Command;
extern crate glob;
use glob::glob;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let entries = generatelist(path);
    display(path, entries);
    }

fn display(path: &str, dir_entries:std::vec::Vec<std::string::String>){
    match rofi::Rofi::new(&dir_entries).prompt("Books").run() {
        Ok(_choice) => displayOption(path, &_choice),
        Err(rofi::Error::Interrupted) => (),
        Err(e) => println!("Error, {}", e)
    }
}

fn displayOption(path: &str, file: &str){
    match isDir(&(path.to_string() + "/" + file)) {
        false => {
            Command::new("xdg-open")
                .args(&[&(path.to_string() + "/" + file)])
                .spawn()
                .expect("Failed to start");
            },
        true => {
            let newpath = path.to_string() + "/" + file;
            display(&newpath, generatelist(&newpath));
        },
        }
}

fn generatelist(path: &str) -> std::vec::Vec<std::string::String>{
    let mut temp: Vec<std::string::String> = Vec::new();
    let re = Regex::new(r"^[^_]*/").unwrap();
    for file in glob(&(path.to_string() + "/*")).expect("Failed to find") {
        temp.push((re.replace_all(&(file.unwrap().display().to_string()), "")).to_string());
    }
    return temp;
}

fn isDir(filename: &str) -> bool {
    Path::new(filename)
        .is_dir()
}
