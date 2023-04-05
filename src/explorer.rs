use std::{env, fs, io};
use std::io::{stdout, Write};

use termion::color;

pub(crate) fn get_file() -> String {
    let paths = fs::read_dir("./").unwrap();
    let mut files = Vec::new();
    let mut selection: String = String::new();


    for path in paths {
        files.push(path.unwrap());
    }

    for i in 0..files.len() {
        let path = files[i].path();
        let filename = path.display().to_string().trim_start_matches("./").to_string();
        if files[i].metadata().expect("").is_file() {
            println!("{}{}) {}", color::Fg(color::LightCyan), i, &filename);
            continue;
        }
        println!("{}{}) {}", color::Fg(color::LightGreen), i, &filename);
    }

    print!("{}> ", color::Fg(color::LightWhite));
    stdout().flush().unwrap();
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if choice.trim() == ".." {
        env::set_current_dir("..").expect("");
        selection = get_file();
    } else if choice.trim().chars().all(|c| c.is_numeric())
        && choice.trim().parse::<usize>().map_or(false, |index| index < files.len()) {
        for i in 0..files.len() {
            if choice.trim() != i.to_string() {
                continue;
            }
            // Check if selection is file
            if files[i].metadata().expect("").is_file() {
                selection = files[i].path().display().to_string();
                break;
            }
            env::set_current_dir(files[i].path().display().to_string()).expect("");
            selection = get_file();
            break;
        }
    } else {
        println!("Bist du doof oder so? Entweder die Zahlen oder .. nutzen!");
        selection = get_file();
    }

    return selection;
}


pub(crate) fn get_dir() -> String {

    let paths = fs::read_dir("./").unwrap();
    let mut directories = Vec::new();
    let mut selection: String = String::new();


    for path in paths {
        directories.push(path.unwrap());
    }

    for i in 0..directories.len() {
        let path = directories[i].path();
        let filename = path.display().to_string().trim_start_matches("./").to_string();
        if directories[i].metadata().expect("").is_file() {
            println!("{}{}) {}", color::Fg(color::LightCyan), i, &filename);
            continue;
        }
        println!("{}{}) {}", color::Fg(color::LightGreen), i, &filename);
    }

    print!("{}> ", color::Fg(color::LightWhite));
    stdout().flush().unwrap();
    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    if choice.trim() == ".." {
        env::set_current_dir("..").expect("");
        selection = get_file();
    } else if choice.trim().chars().all(|c| c.is_numeric())
        && choice.trim().parse::<usize>().map_or(false, |index| index < directories.len()) {
        for i in 0..directories.len() {
            if choice.trim() != i.to_string() {
                continue;
            }
            // Check if selection is file
            if directories[i].metadata().expect("").is_file() {
                selection = directories[i].path().display().to_string();
                break;
            }
            env::set_current_dir(directories[i].path().display().to_string()).expect("");
            selection = get_file();
            break;
        }
    } else {
        println!("Bist du doof oder so? Entweder die Zahlen oder .. nutzen!");
        selection = get_file();
    }
    return selection;
}