use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Filename error");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn run() {
    let input = read_input("input3.txt");
    let mut score = 0;
    let teams: Vec<&[String]> = input.chunks(3).collect();
    for t in teams {        
        for c in t[0].chars() {
            if t[1].contains(c) && t[2].contains(c) {
                if c.is_lowercase() {
                    score += c as i32 - 96;
                    break;
                } else {
                    score += c as i32 - 38;
                    break;
                }
            }
        }
    }
    println!("Svaret på del 2 är {}", score)
}