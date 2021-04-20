extern crate rofi;
extern crate regex;
extern crate clap;
use regex::Regex;
use std::path::Path;
use std::process::Command;
extern crate glob;
use glob::glob;
use std::env;
use clap::{Arg,App};

fn main() {
    let args = App::new("rofi_book")
	.version("0.1.0")
	.about("shows all the files in folder and opens using xdg-open")
	.author("ThePenguin")
    .args_from_usage("
        -d, --dir=[TARGET_PATH] 'Sets your target PATH'
       ")
	.get_matches();

    let path = args.value_of("dir").unwrap_or(".");
    let entries = generate_list(path);
    display(path, entries);
    }

fn display(path: &str, dir_entries:std::vec::Vec<std::string::String>){
    match rofi::Rofi::new(&dir_entries).prompt("Books").run() {
        Ok(_choice) => display_option(path, &_choice),
        Err(rofi::Error::Interrupted) => (),
        Err(e) => println!("Error, {}", e)
    }
}

fn display_option(path: &str, file: &str){
    match is_dir(&format!("{}/{}", &path, file)) {
        false => {
            Command::new("xdg-open")
                .args(&[&format!("{}/{}", &path, file)])
                .spawn()
                .expect("Failed to start");
            },
        true => {
            let newpath = path.to_string() + "/" + file;
            display(&newpath, generate_list(&newpath));
        },
        }
}

fn generate_list(path: &str) -> std::vec::Vec<std::string::String>{
    let mut temp: Vec<std::string::String> = Vec::new();
    let re = Regex::new(r"^[^_]*/").unwrap();
    for file in glob(&format!("{}/*", path)).expect("Failed to find") {
        temp.push((re.replace_all(&(file.unwrap().display().to_string()), "")).to_string());
    }
    return temp;
}

fn is_dir(filename: &str) -> bool {
    Path::new(filename)
        .is_dir()
}
