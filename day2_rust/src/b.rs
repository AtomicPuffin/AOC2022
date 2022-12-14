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
    let mut input = read_input("input2.txt");
    let mut score = 0;
    for i in input {
        let (elf, you) = i.split_once(' ').unwrap();
        match you {
            "X" => score += if elf == "A" {3} else if elf == "B" {1} else {2},
            "Y" => score += if elf == "A" {4} else if elf == "B" {5} else {6},
            "Z" => score += if elf == "A" {8} else if elf == "B" {9} else {7},
            _ => println!("Fel i match"),
        }
    }
    println!("Svaren på del 1 är {}", score)
}