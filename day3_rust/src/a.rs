//use core::num::dec2flt::parse;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
//use std::io::Result;

fn read_input(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Filename error");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn run() {
    let input = read_input("input3.txt");
    //let mut input = read_input("input3 copy.txt");
    let mut score = 0;
    for i in input {        
        let (first, second) = i.split_at(i.len()/2);
        for c in first.chars() {
            if second.contains(c) {
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

    println!("Svaret på del 1 är {}", score)
}