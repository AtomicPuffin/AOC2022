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
    let input = read_input("input2.txt");
    let mut score = 0;
    for i in input {
        let (elf, you) = i.split_once(' ').unwrap();
        match you {
            "X" => {
                score += 1 + if elf == "A" {
                    3
                } else if elf == "B" {
                    0
                } else {
                    6
                }
            }
            "Y" => {
                score += 2 + if elf == "B" {
                    3
                } else if elf == "C" {
                    0
                } else {
                    6
                }
            }
            "Z" => {
                score += 3 + if elf == "C" {
                    3
                } else if elf == "A" {
                    0
                } else {
                    6
                }
            }
            _ => println!("Fel i match"),
        }
    }
    println!("Svaren på del 1 är {}", score)
}
