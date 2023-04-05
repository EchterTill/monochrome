use std::{env, fs, io};
use std::fs::DirEntry;
use std::io::{stdout, Write};
use std::path::Path;

use termion::color;

pub(crate) fn get_file() -> String {
    let path = Path::new(".");

    let mut paths = fs::read_dir("./").unwrap();
    let mut files = Vec::new();
    let mut file: String = String::new();


    for (i, path) in paths.by_ref().enumerate() {
        if path.as_ref().unwrap().metadata().expect("").is_file() {
            println!("{}{}) {}", color::Fg(color::LightCyan), i, path.as_ref().unwrap().path().display().to_string().trim_start_matches("./"));
        } else {
            println!("{}{}) {}", color::Fg(color::LightGreen), i, path.as_ref().unwrap().path().display().to_string().trim_start_matches("./"));
        }
        files.push(path.unwrap());
    }
    print!("{}> ", color::Fg(color::LightWhite));
    stdout().flush().unwrap();
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if choice.trim() == ".." {
        env::set_current_dir("..").expect("");
        file = get_file();
        env::set_current_dir(&path).expect("");
    }

    else if choice.trim().parse::<usize>().unwrap() < files.len() {
        for i in 0..files.len() {
            if choice.trim().parse::<usize>().unwrap() == i {
                if files[i].metadata().expect("").is_file() {
                    file = files[i].path().display().to_string();
                } else {
                    env::set_current_dir(files[i].path().display().to_string()).expect("");
                    file = get_file();
                    env::set_current_dir(&path).expect("Bitte nicht das Verzeichnis löschen, \
                während du das Programm nutzt danke ;)");
                }
                break;
            }
        }
    }
    println!("test");
    return file;
}