use std::collections::HashMap;
use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> i64 {
    let monkeys = parse_monkeys(input);
    run_monkey(&monkeys, "root".to_string())
}

fn part_2 (input: &str) -> i64 {
    let monkeys = parse_monkeys(input);
    let monkey = "root".to_string();
    let ma = monkeys[&monkey][0].clone();
    let mb = monkeys[&monkey][2].clone();
    if find_humn(&monkeys, ma.clone()) {
        let x = run_monkey(&monkeys, mb);
        return branch(&monkeys, ma, x)
    } else {
        let x = run_monkey(&monkeys, ma);
        return branch(&monkeys, mb, x)
    }
}

fn run_monkey(monkeys: &HashMap<String, Vec<String>>, monkey: String) -> i64 {
    if monkeys[&monkey].len() == 1 {
        return monkeys[&monkey][0].parse::<i64>().unwrap();
    } else {
        let ma = monkeys[&monkey][0].clone();
        let mb = monkeys[&monkey][2].clone();
        let a = run_monkey(monkeys, ma);
        let b = run_monkey(monkeys, mb);
        match monkeys[&monkey][1].as_str() {
            "+" => return a + b,
            "-" => return a - b,
            "*" => return a * b,
            "/" => return a / b,
            _ => panic!("Parse error"),
        }
    }
}

fn branch(monkeys: &HashMap<String, Vec<String>>, monkey: String, x: i64) -> i64 {
    if monkey == "humn".to_string() {
        return x
    }
    if monkeys[&monkey].len() == 1 {
        panic!("humn was not here");
    }
    let ma = monkeys[&monkey][0].clone();
    let mb = monkeys[&monkey][2].clone();
    if find_humn(&monkeys, ma.clone()) {
        let y = run_monkey(&monkeys, mb);
        let mh = ma;
        match monkeys[&monkey][1].as_str() {
            "+" => return branch(&monkeys, mh, x-y),
            "-" => return branch(&monkeys, mh, x+y),
            "*" => return branch(&monkeys, mh, x/y),
            "/" => return branch(&monkeys, mh, x*y),
            _ => panic!("Parse error"),
        }
    } else {
        let y = run_monkey(&monkeys, ma);
        let mh = mb;
        match monkeys[&monkey][1].as_str() {
            "+" => return branch(&monkeys, mh, x-y),
            "-" => return branch(&monkeys, mh, y-x),
            "*" => return branch(&monkeys, mh, x/y),
            "/" => return branch(&monkeys, mh, y/x),
            _ => panic!("Parse error"),
        }
    }
}

fn find_humn(monkeys: &HashMap<String, Vec<String>>, monkey: String) -> bool {
    if monkey == "humn".to_string() {
        return true
    } else if monkeys[&monkey].len() == 1 {
        return false
    } else {
        let ma = monkeys[&monkey][0].clone();
        let mb = monkeys[&monkey][2].clone();
        return find_humn(monkeys, ma) || find_humn(monkeys, mb)
    }
}

fn parse_monkeys(input: &str) -> HashMap<String, Vec<String>> {
    let mut monkeys = HashMap::new();
    for l in input.lines() {
        let mut raw = l.split(" ");
        let monkey = raw.next().unwrap()[..4].to_string();
        let value = raw.map(|x| x.to_string()).collect();
        monkeys.insert(monkey, value);
    }
    monkeys
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
        assert_eq!(part_1(&read_file("input copy.txt")), 152);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 31017034894002);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 301);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 3555057453229);
    }
}