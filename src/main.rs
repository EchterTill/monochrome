/*
Monochrome - Color to grayscale
By Kruwy3A (Till Vogelsang)
*/
use std::fs;

fn main() {
    credits();
    load_image("synth_1.ppm");
}

fn credits() {
    println!("
 _____                 _
|     |___ ___ ___ ___| |_ ___ ___ _____ ___
| | | | . |   | . |  _|   |  _| . |     | -_|
|_|_|_|___|_|_|___|___|_|_|_| |___|_|_|_|___|

┌────────────┐
│ Monochrome │
│ v0.0.2     │
│ by Kruwy3A │
└────────────┘
");
}

fn load_image(filename: &str) {
    let contents = fs::read(filename)
        .expect("Should have been able to read the file");

// 80 -> P; 54 -> 6; Check, if header is valid (P6)
    if !(contents[0] == 80 && contents[1] == 54) {
        eprintln!("Invalid image format");
        std::process::exit(65);
    }

    let metadata = get_metadata(contents.clone());

    println!("Breite: {}", metadata.0);
    println!("Höhe: {}", metadata.1);
    println!("Farben: {}", metadata.2);
    println!("Start: {}", metadata.3);
    println!("Erster: {}", contents[metadata.3]);
}


fn get_next_whitespace(contents: Vec<u8>, start: usize) -> usize {
    for i in start + 1..contents.len() {
        if contents[i] == 10 {
            return i;
        }
    }
    return start;
}

fn get_metadata(contents: Vec<u8>) -> (usize, usize, usize, usize) {
    let mut wspos: usize = 0;
    let mut size = String::new();
    let mut depth = String::new();
    let mut metapos = [0, 0, 0];

    let mut i = 0;
    while i <= 2 {
        wspos = get_next_whitespace(contents.clone(), wspos);
        if contents[wspos + 1] == 35 {
            wspos = get_next_whitespace(contents.clone(), wspos);
        }
        metapos[i] = wspos;

        i = i + 1;
    }


    for i in metapos[0] + 1..metapos[1] {
        size.push(char::from_u32(contents[i] as u32).unwrap());
    }

    for i in metapos[1] + 1..metapos[2] {
        depth.push(char::from_u32(contents[i] as u32).unwrap());
    }

    println!("{}", metapos[1]);
    //return (width, height, depth, startpos)
    return (size.split_ascii_whitespace().next_back().unwrap().parse().unwrap(), size.split_ascii_whitespace().next().unwrap().parse().unwrap(), depth.parse().unwrap(), wspos+1);
}