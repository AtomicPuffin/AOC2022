
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
    let mut max = 0;
    let mut current = 0;
    input.push("".to_string());
    for e in input {
        if e == "" {
            if current > max {
                max = current;
            };
            current = 0;
        } else {
            current += e.parse::<i32>().unwrap();
        }
    }
    println!("Svaret på del 1 är: {}", max);
}