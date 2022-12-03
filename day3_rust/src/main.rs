#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]

mod a;
mod b;

use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;

fn main() {
    a::run();
    b::run();
    let file = "input.txt";
    println!("Answer to Part 1 functional using sets: {}", part_1f(fs::read_to_string(file).unwrap().trim()));
    println!("Answer to Part 1 imperative using sets: {}", part_1i(fs::read_to_string(file).unwrap().trim()));
}

fn part_1f (input: &str) -> i32 {
    input
        .lines()
        .map(|backpack| backpack.split_at(backpack.len() / 2))
        .map(|(first,second)| (first.chars().collect::<HashSet<char>>(), second.chars().collect::<HashSet<char>>()))
        .map(|(fh,sh)| *fh.intersection(&sh).last().unwrap()) 
        .map(|c| if c.is_lowercase() {c as i32 - 96} else {c as i32 - 38})
        .sum()
}

fn part_1i (input: &str) -> i32 {
    let mut score = 0;
    for l in input.lines() {
        let (first, second) = l.split_at(l.len() / 2);
        let f: HashSet<char> = first.chars().collect();
        let s: HashSet<char> = second.chars().collect();
        let ci = f.intersection(&s);
        let c = ci.last().unwrap();
        if c.is_lowercase() {score += *c as i32 - 96} else {score += *c as i32 - 38}
    }
    score
}
