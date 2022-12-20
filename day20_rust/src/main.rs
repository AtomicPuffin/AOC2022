use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> i64 {
    let lines: Vec<i64> = input.lines()
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
    let mut start: Vec<(i64, i64)> = Vec::new();
    for l in 00..lines.len() {
        start.push((lines[l], l as i64));
    }
    let mixed = mix(start);
    find_ans(mixed)
}

fn part_2 (input: &str) -> i64 {
    let lines: Vec<i64> = input.lines()
    .map(|x| x.parse::<i64>().unwrap())
    .collect();
let mut start: Vec<(i64, i64)> = Vec::new();
for l in 00..lines.len() {
    start.push((lines[l] * 811589153, l as i64));
}
let mut mixed = start;
for _ in 0..10 {
    mixed = mix(mixed);
}
find_ans(mixed)
}

fn mix(mut mixed: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let len = mixed.len() as i64;
    for i in 0..len as i64 {
        let pos = mixed.iter().position(|&x| x.1 == i).unwrap(); //find pos of old index i
        let num = mixed[pos].0; // save number, remove
        mixed.remove(pos);
        let new_pos = (pos as i64 + num).rem_euclid(len-1) as usize; //Calculate new position
        mixed.insert(new_pos, (num, i)); //technically inserts at front and not back if pos = 0, makes no difference
    }
    mixed
}

fn find_ans(mixed: Vec<(i64, i64)>) -> i64 {
    let mut ans = 0;
    let pos = mixed.iter().position(|&x| x.0 == 0).unwrap(); // find 0
    let len = mixed.len() as i64;
    for i in [1000, 2000, 3000] { //sum items
        let new_pos = (pos as i64 + i).rem_euclid(len) as usize;
        ans += mixed[new_pos].0;
    }
    ans
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
        assert_eq!(part_1(&read_file("input copy.txt")), 3);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 7228);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 1623178306);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 4526232706281);
    }
}