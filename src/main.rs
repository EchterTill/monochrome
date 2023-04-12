/*
Monochrome - Color to grayscale
By Kruwy3A (Till Vogelsang)
*/

extern crate termion;

use std::{fs, io};
use std::collections::HashMap;

use termion::color;

mod explorer;

fn main() {
    println!("{}", color::Fg(color::LightWhite));

    let mut mode_settings = HashMap::from([
        ("grayscale", false),
        ("r", true),
        ("g", true),
        ("b", true)
    ]);

    let mut has_input = false;

    credits();

    use std::io;

    let mut image: (Vec<u8>, usize, usize, usize, usize) = load_image("west_1.ppm");

    loop {
        println!("Select an option:");
        println!("1) Set Input");
        println!("2) Set Output");
        println!("3) Color Settings");
        println!("4) Run");
        eprint!("> ");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let filepath = &explorer::get_file();
                println!("{}", filepath);
                image = load_image(filepath);
                has_input = true;
            }
            "2" => {
                println!("Coming soon");
                println!("output.ppm");
            }
            "3" => {
                mode_settings = set_mode(mode_settings);
            }
            "4" => {
                if has_input {
                    println!("Running program");
                    break;
                } else { println!("Please set input first"); }
            }
            _ => println!("Invalid choice"),
        }
    }


    let mut pixels = read_pixels(&image.0, image.4);

    if mode_settings.get("grayscale").unwrap() == &true {
        pixels = convert_grayscale(&pixels);
    }

    if mode_settings.get("r").unwrap() == &false {
        pixels = remove_color(&pixels, "r");
    }

    if mode_settings.get("g").unwrap() == &false {
        pixels = remove_color(&pixels, "g");
    }

    if mode_settings.get("b").unwrap() == &false {
        pixels = remove_color(&pixels, "b");
    }

    let new_image = generate_image(&mut image.0, image.4, (&pixels).to_vec());


    fs::write("output.ppm", &new_image).unwrap();
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
└────────────┘");
}

fn set_mode(mut settings: HashMap<&str, bool>) -> HashMap<&str, bool> {
    println!("Toggle Color settings:");
    print!("1) Grayscale (");
    if settings.get("grayscale").unwrap() == &false {
        println!("{}OFF{})", color::Fg(color::Red), color::Fg(color::LightWhite))
    } else { println!("{}ON{})", color::Fg(color::Green), color::Fg(color::LightWhite)) }

    print!("2) RED (");
    if settings.get("r").unwrap() == &false {
        println!("{}OFF{})", color::Fg(color::Red), color::Fg(color::LightWhite))
    } else { println!("{}ON{})", color::Fg(color::Green), color::Fg(color::LightWhite)) }

    print!("3) GREEN (");
    if settings.get("g").unwrap() == &false {
        println!("{}OFF{})", color::Fg(color::Red), color::Fg(color::LightWhite))
    } else { println!("{}ON{})", color::Fg(color::Green), color::Fg(color::LightWhite)) }

    print!("4) BLUE (");
    if settings.get("b").unwrap() == &false {
        println!("{}OFF{})", color::Fg(color::Red), color::Fg(color::LightWhite))
    } else { println!("{}ON{})", color::Fg(color::Green), color::Fg(color::LightWhite)) }
    eprint!("> ");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    match choice.trim() {
        "1" => { *settings.get_mut("grayscale").unwrap() = !*settings.get("grayscale").unwrap() }
        "2" => {
            *settings.get_mut("r").unwrap() = !*settings.get("r").unwrap()
        }
        "3" => {
            *settings.get_mut("g").unwrap() = !*settings.get("g").unwrap()
        }
        "4" => {
            *settings.get_mut("b").unwrap() = !*settings.get("b").unwrap()

        }

        _ => {},
    }

    return settings;
}

fn load_image(filename: &str) -> (Vec<u8>, usize, usize, usize, usize) {
    let contents = fs::read(filename)
        .expect("Should have been able to read the file");

// 80 -> P; 54 -> 6; Check, if header is valid (P6)
    if !(contents[0] == 80 && contents[1] == 54) {
        eprintln!("{}Invalid image format{}", color::Fg(color::Red), color::Fg(color::LightWhite));
        return load_image(&explorer::get_file());
    }

    let metadata = get_metadata(&contents);

    /* Debug
    println!("Breite: {}", metadata.0);
    println!("Höhe: {}", metadata.1);
    println!("Farben: {}", metadata.2);
    println!("Start: {}", metadata.3);
    println!("Erster: {}", contents[metadata.3]);
    println!("Bildlänge: {}", contents.len() - metadata.3);*/

    return (contents, metadata.0, metadata.1, metadata.2, metadata.3);
}

fn get_metadata(contents: &Vec<u8>) -> (usize, usize, usize, usize) {
    fn get_next_whitespace(contents: &Vec<u8>, &start: &usize) -> usize {
        for i in start + 1..contents.len() {
            if contents[i] == 10 {
                return i;
            }
        }
        return start;
    }

    let mut wspos: usize = 0;
    let mut size = String::new();
    let mut depth = String::new();
    let mut metapos = [0, 0, 0];

    let mut i = 0;
    while i <= 2 {
        wspos = get_next_whitespace(&contents, &wspos);
        if contents[wspos + 1] == 35 {
            wspos = get_next_whitespace(&contents, &wspos);
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
    return (size.split_ascii_whitespace().next_back().unwrap().parse().unwrap(), size.split_ascii_whitespace().next().unwrap().parse().unwrap(), depth.parse().unwrap(), wspos + 1);
}

fn read_pixels(contents: &Vec<u8>, start: usize) -> Vec<(usize, usize, usize)> {
    let mut image: Vec<(usize, usize, usize)> = Vec::new();
    for i in start..contents.len() {
        if (i - start) % 3 == 0 {
            image.push((contents[i] as usize, contents[i + 1] as usize, contents[i + 2] as usize));
        }
    }
    return image;
}

fn convert_grayscale(pixels: &Vec<(usize, usize, usize)>) -> Vec<(usize, usize, usize)> {
    let mut new_pixels = Vec::with_capacity(pixels.len());
    for i in 0..pixels.len() {
        let average = (pixels[i].0 + pixels[i].1 + pixels[i].2) / 3;
        new_pixels.push((average, average, average));
    }
    return new_pixels;
}

fn remove_color(pixels: &Vec<(usize, usize, usize)>, color: &str) -> Vec<(usize, usize, usize)> {
    let mut new_pixels = Vec::with_capacity(pixels.len());
    for i in 0..pixels.len() {
        match color {
            "r" => {
                new_pixels.push((0, pixels[i].1, pixels[i].2));
            }
            "g" => {
                new_pixels.push((pixels[i].0, 0, pixels[i].2));
            }
            "b" => {
                new_pixels.push((pixels[i].0, pixels[i].1, 0));
            }
            _ => {}
        }
    }
    return new_pixels;
}

fn generate_image(image: &mut Vec<u8>, start: usize, pixels: Vec<(usize, usize, usize)>) -> &Vec<u8> {
    for i in start..image.len() {
        if (i - start) % 3 == 0 {
            image[i] = pixels[(i - start) / 3].0 as u8;
            image[i + 1] = pixels[(i - start) / 3].1 as u8;
            image[i + 2] = pixels[(i - start) / 3].2 as u8;
        }
    }
    return image;
}