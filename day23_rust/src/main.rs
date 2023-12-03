#![allow(unused)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("input copy.txt"))
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("input copy.txt"))
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let mut elves = parse_input(input);
    let mut directions = vec![
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
        [(-1, -1), (-1, 0), (-1, 1)],
        [(1, -1), (1, 0), (1, 1)],
    ];

    for _round in 0..10 {
        let mut proposals: HashMap<(i64, i64), ((i64, i64), bool)> = HashMap::new();
        for elf in &elves {
            if !check_looner(&elves, &elf) {
                if let Some(prop) = propose(elf, &directions, &elves) {
                    if proposals.contains_key(&prop) {
                        proposals.insert(prop.clone(), (elf.clone(), true));
                    } else {
                        proposals.insert(prop.clone(), (elf.clone(), false));
                    }
                }
            }
        }
        for (to, from) in proposals {
            if from.1 {
                continue;
            } else {
                elves.remove(&from.0);
                elves.insert(to);
            }
        }
        directions.rotate_left(1);
    }
    calculate_answer(&elves)
}

fn part_2(input: &str) -> i64 {
    let mut elves = parse_input(input);
    let mut directions = vec![
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
        [(-1, -1), (-1, 0), (-1, 1)],
        [(1, -1), (1, 0), (1, 1)],
    ];
    let mut round = 0;
    loop {
        let mut change = false;
        let mut proposals: HashMap<(i64, i64), ((i64, i64), bool)> = HashMap::new();
        for elf in &elves {
            if !check_looner(&elves, &elf) {
                if let Some(prop) = propose(elf, &directions, &elves) {
                    if proposals.contains_key(&prop) {
                        proposals.insert(prop.clone(), (elf.clone(), true));
                    } else {
                        proposals.insert(prop.clone(), (elf.clone(), false));
                    }
                }
            }
        }
        for (to, from) in proposals {
            if from.1 {
                continue;
            } else {
                change = true;
                elves.remove(&from.0);
                elves.insert(to);
            }
        }
        directions.rotate_left(1);
        round += 1;
        if !change {
            return round;
        }
    }
}

fn print_elves(elves: &HashSet<(i64, i64)>) {
    let mut max = (0, 0);
    let mut min = (i64::MAX, i64::MAX);

    for e in elves {
        max.0 = max.0.max(e.0);
        min.0 = min.0.min(e.0);
        max.1 = max.1.max(e.1);
        min.1 = min.1.min(e.1);
    }
    for y in min.1..max.1 + 1 {
        let mut line = "".to_string();
        for x in min.0..max.0 + 1 {
            if elves.contains(&(x, y)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
    println!("");
}

fn calculate_answer(elves: &HashSet<(i64, i64)>) -> i64 {
    let mut max = (0, 0);
    let mut min = (i64::MAX, i64::MAX);
    for e in elves {
        max.0 = max.0.max(e.0);
        min.0 = min.0.min(e.0);
        max.1 = max.1.max(e.1);
        min.1 = min.1.min(e.1);
    }
    (max.0 - min.0 + 1) * (max.1 - min.1 + 1) - elves.len() as i64
}

fn propose(
    elf: &(i64, i64),
    directions: &Vec<[(i64, i64); 3]>,
    elves: &HashSet<(i64, i64)>,
) -> Option<(i64, i64)> {
    for d in directions {
        let a = (elf.0 + d[0].0, elf.1 + d[0].1);
        let b = (elf.0 + d[1].0, elf.1 + d[1].1);
        let c = (elf.0 + d[2].0, elf.1 + d[2].1);
        if elves.contains(&a) || elves.contains(&b) || elves.contains(&c) {
            continue;
        } else {
            return Some(b);
        }
    }
    None
}

fn check_looner(elves: &HashSet<(i64, i64)>, elf: &(i64, i64)) -> bool {
    let nghb = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    for n in nghb {
        if elves.contains(&(elf.0 + n.0, elf.1 + n.1)) {
            return false;
        }
    }
    return true;
}

fn parse_input(input: &str) -> HashSet<(i64, i64)> {
    let mut elves = HashSet::new();
    let mut y = 0;
    for l in input.lines() {
        let mut x = 0;
        for c in l.chars() {
            if c == '#' {
                elves.insert((x, y));
            }
            x += 1;
        }
        y += 1;
    }
    elves
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
        assert_eq!(part_1(&read_file("input copy.txt")), 110);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 3877);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 20);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 982);
    }
}
