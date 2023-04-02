/*
Monochrome - Color to grayscale
By Kruwy3A (Till Vogelsang)
*/
use std::fs;

fn main() {

    credits();

    let image = load_image("boxes.ppm");

    let pixels = read_pixels(image.0.clone(), image.4);

    let gray_pixels = convert_grayscale(pixels.clone());

    let new_image = generate_image(image.0.clone(), image.4, gray_pixels.clone());


    fs::write("./output.ppm", new_image.clone()).unwrap();

    if image.0 == new_image {
        println!("Fehler")
    }
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

fn load_image(filename: &str) -> (Vec<u8>, usize, usize, usize, usize) {
    let contents = fs::read(filename)
        .expect("Should have been able to read the file");

// 80 -> P; 54 -> 6; Check, if header is valid (P6)
    if !(contents[0] == 80 && contents[1] == 54) {
        eprintln!("Invalid image format");
        std::process::exit(65);
    }

    let metadata = get_metadata(contents.clone());

    //Debug
    println!("Breite: {}", metadata.0);
    println!("Höhe: {}", metadata.1);
    println!("Farben: {}", metadata.2);
    println!("Start: {}", metadata.3);
    println!("Erster: {}", contents[metadata.3]);
    println!("Bildlänge: {}", contents.len() - metadata.3);

    return (contents, metadata.0, metadata.1, metadata.2, metadata.3);
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
    return (size.split_ascii_whitespace().next_back().unwrap().parse().unwrap(), size.split_ascii_whitespace().next().unwrap().parse().unwrap(), depth.parse().unwrap(), wspos + 1);
}

fn read_pixels(contents: Vec<u8>, start: usize) -> Vec<(usize, usize, usize)> {
    let mut image: Vec<(usize, usize, usize)> = Vec::new();
    for i in start..contents.len() {
        if (i - start) % 3 == 0 {
            image.push((contents[i] as usize, contents[i + 1] as usize, contents[i + 2] as usize));
        }
    }
    return image;
}

fn convert_grayscale(mut pixels: Vec<(usize, usize, usize)>) -> Vec<(usize, usize, usize)> {
    for i in 0..pixels.len() {
        let average = (pixels[i].0 + pixels[i].1 + pixels[i].2) / 3;
        pixels[i] = (average, average, average)
    }
    return pixels;
}

fn generate_image(mut image: Vec<u8>, start: usize, pixels: Vec<(usize, usize, usize)>) -> Vec<u8> {
    for i in start..image.len() {
        if (i - start) % 3 == 0 {
            image[i] = pixels[(i - start) / 3].0 as u8;
            image[i + 1] = pixels[(i - start) / 3].1 as u8;
            image[i + 2] = pixels[(i - start) / 3].2 as u8;
        }
    }
    return image;
}

