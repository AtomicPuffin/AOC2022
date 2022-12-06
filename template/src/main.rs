#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;

fn main() {

    println!("Answer to Part 1: {}", part_1(read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(read_file("input copy.txt")));
}

fn part_1 (input: &str) -> i32 {
    todo!()
    //.inspect(|i| println!("{}", i))
}

fn part_2 (input: &str) -> i32 {
    todo!()
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().trim()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use pretty_assertions::assert_eq;

    #[ignore]
    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(read_file("input copy.txt")), todo!());
    }

    #[ignore]
    #[test]
    fn test_p1() {
        assert_eq!(part_1(read_file("input.txt")), todo!());
    }

    #[ignore]
    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(read_file("input copy.txt")), todo!());
    }

    #[ignore]
    #[test]
    fn test_p2() {
        assert_eq!(part_2(read_file("input.txt")), todo!());
    }
}