use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1(input: &str) -> i64 {
    let (dirs, _) = build_tree(input);
    let mut answer = 0;
    for k in dirs.keys() {
        let x = dirs[k];
        if x <= 100_000 {
            answer += x;
        }
    }
    answer
}

fn part_2(input: &str) -> i64 {
    let (dirs, total) = build_tree(input);
    let need = (70_000_000 - total - 30_000_000).abs();
    let mut current = i64::MAX;
    for i in dirs.keys() {
        if dirs[i] > need && dirs[i] < current {
            current = dirs[i];
        }
    }
    current
}

fn build_tree(input: &str) -> (HashMap<String, i64>, i64) {
    let mut root = HashMap::new();
    let mut current = Vec::new();
    let mut total = 0;
    current.push("/");
    for l in input.lines().skip(1) {
        let inst: Vec<&str> = l.split(" ").collect();
        if inst[0] == "$" {
            if inst[1] == "cd" {
                if inst[2] == ".." {
                    // goto parent
                    current.pop();
                } else {
                    // goto child
                    current.push(inst[2]);
                }
            } else if inst[1] == "ls" {
                //do nothing, just a header
            } else {
                panic!("Unknown command following $");
            }
        } else if inst[0] == "dir" {
            //ignore for now, not used in part 1
        } else {
            //assume file
            let x = inst[0].parse::<i64>().unwrap();
            total += x;
            let mut dir = "".to_string();
            for i in &current {
                let next = format!("{}{}", &dir, i).to_string();
                dir = next.clone();
                if root.contains_key(&next) {
                    let y = root[&next] + x;
                    root.insert(next, y);
                } else {
                    root.insert(next, x);
                }
            }
        }
    }
    (root, total)
}


fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("input copy.txt")), 95437);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1077191);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 24933642);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 5649896);
    }
}