use std::collections::HashSet;
use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy 2.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut h = (0,0); // x, y
    let mut t = (0,0);
    for i in input.lines() {
        let mut row = i.chars();
        let d = row.next().unwrap();
        row.next();
        let s = row.as_str().parse::<i32>().unwrap();
        for _ in 0..s {
            match d {
                'U' => h.0 += 1,
                'D' => h.0 -= 1,
                'R' => h.1 += 1,
                'L' => h.1 -= 1,
                _ => panic!("invalid instruction"),
            }
            t = follow(h,t);
            visited.insert(t);
        }
    }
    visited.len()
}

fn part_2 (input: &str) -> usize {
    let mut visited = HashSet::new();
    let mut h = (0,0); // x, y
    let mut t = [(0,0); 9];
    for i in input.lines() {
        let mut row = i.chars();
        let d = row.next().unwrap();
        row.next();
        let s = row.as_str().parse::<i32>().unwrap();
        for _ in 0..s {
            match d {
                'U' => h.0 += 1,
                'D' => h.0 -= 1,
                'R' => h.1 += 1,
                'L' => h.1 -= 1,
                _ => panic!("invalid instruction"),
            }
            t[0] = follow(h,t[0]);
            for i in 1..9 {
                t[i] = follow(t[i-1],t[i]);
            }
            visited.insert(t[8]);
        }
    }
    visited.len()
}

fn follow(h: (i32, i32), mut t: (i32,i32)) -> (i32,i32) {
    if (h.0-t.0).abs() > 1 || (h.1-t.1).abs() > 1 {
        if h.0 != t.0 {
            t.0 += (h.0-t.0) / (h.0-t.0).abs();
        }
        if h.1 != t.1 {
            t.1 += (h.1-t.1) / (h.1-t.1).abs();
        }
    }
    t
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

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 5695);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 36);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 2434);
    }
}