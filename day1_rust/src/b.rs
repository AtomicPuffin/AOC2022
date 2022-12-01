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
    let mut input = read_input("input1.txt");
    //let mut input = read_input("input1 copy.txt");
    let mut elves: Vec<i32> = Vec::new();
    let mut current = 0;
    input.push("".to_string());
    for e in input {
        if e == "" {
            elves.push(current);
            current = 0;
        } else {
            current += e.parse::<i32>().unwrap();
        }
    }
    elves.sort();
    println!("Svaret på del 2 är: {}", elves.pop().unwrap()+elves.pop().unwrap()+elves.pop().unwrap());
}