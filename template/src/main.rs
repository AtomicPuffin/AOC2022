#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> i32 {
    let mut visited = HashSet::new();
    let mut h = (0,0);
    let mut t = (0,0);
    
}

fn part_2 (input: &str) -> i32 {
    todo!()
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("input copy.txt")), 13);
    }

    #[ignore]
    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), todo!());
    }

    #[ignore]
    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), todo!());
    }

    #[ignore]
    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), todo!());
    }
}