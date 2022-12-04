use std::fs;
use itertools::Itertools;

fn main() {
    let file = "input.txt";
    println!("Answer to Part 1: {}", part_1(fs::read_to_string(file).unwrap().trim()));
    println!("Answer to Part 2: {}", part_2(fs::read_to_string(file).unwrap().trim()));
}

fn part_1 (input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            l.split(['-', ','])
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .map(|(e1s,e1e,e2s,e2e)| 
            if (e1s >= e2s && e1e <= e2e) 
                || (e1s <= e2s && e1e >= e2e) 
            {1} else {0})
        .sum()
}

fn part_2 (input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            l.split(['-', ','])
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap() 
        })
        .map(|(e1s,e1e,e2s,e2e)| 
            if (e1s >= e2s && e1s <= e2e) 
                || (e1e >= e2s && e1e <= e2e) 
                || (e1s <= e2s && e1e >= e2e) 
            {1} else {0})
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(fs::read_to_string("input copy.txt").unwrap().trim()), 2);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(fs::read_to_string("input.txt").unwrap().trim()), 584);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(fs::read_to_string("input copy.txt").unwrap().trim()), 4);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(fs::read_to_string("input.txt").unwrap().trim()), 933);
    }
}