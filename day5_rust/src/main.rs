#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]
use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::fs;

fn main() {
    let file = "input.txt";
    println!("Answer to Part 1 functional using sets: {}", part_1(fs::read_to_string(file).unwrap().trim()));
    println!("Answer to Part 2 imperative using sets: {}", part_2(fs::read_to_string(file).unwrap().trim()));
}

fn part_1 (input: &str) -> i32 {
    todo!()
}

fn part_2 (input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use pretty_assertions::assert_eq;

    #[ignore]
    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(fs::read_to_string("input copy.txt").unwrap().trim()), todo!());
    }

    #[ignore]
    #[test]
    fn test_p1() {
        assert_eq!(part_1(fs::read_to_string("input.txt").unwrap().trim()), todo!());
    }

    #[ignore]
    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(fs::read_to_string("input copy.txt").unwrap().trim()), todo!());
    }

    #[ignore]
    #[test]
    fn test_p2() {
        assert_eq!(part_2(fs::read_to_string("input.txt").unwrap().trim()), todo!());
    }
}