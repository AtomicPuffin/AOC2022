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
    println!("Ej påbörjad");
}