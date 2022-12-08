use std::fs;

fn main() {
    let file = "input.txt";
    println!("Answer to Part 1: {}", part_1(&fs::read_to_string(file).unwrap()));
    println!("Answer to Part 2: {}", part_2(&fs::read_to_string(file).unwrap()));
}

fn part_1 (input: &str) -> String {
    let (stack_data, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stack_data);
    let inst = parse_inst(instructions);

    for i in inst {
        for _ in 0..i.0 {
            let x = stacks[i.1].pop().unwrap();
            stacks[i.2].push(x);
        }
    }

    let ans = parse_out(stacks);
    ans
}

fn part_2 (input: &str) -> String {
    let (stack_data, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_stacks(stack_data);
    let inst = parse_inst(instructions);

    for i in inst {
        let mut x = Vec::new();
        for _ in 0..i.0 {
            x.push(stacks[i.1].pop().unwrap());
        }
        for _ in 0..i.0 {
            stacks[i.2].push(x.pop().unwrap());
        }
    }

    let ans = parse_out_functional(stacks);
    ans
}

fn parse_stacks(stack_data: &str) -> Vec<Vec<&str>> {
    let mut stacks = Vec::new();
    let rs = stack_data[..stack_data.len()-2].lines();

    for _ in 0..stack_data.lines().last().unwrap().split("   ").count() {
        stacks.push(Vec::new());
    }
    
    for l in rs {
        let mut pile_no = 0;
        let crates = l.as_bytes()
            .chunks(4)
            .map(std::str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap();
        for c in &crates {
            if c.chars().nth(0).unwrap() != ' ' {
                stacks[pile_no].insert(0, *c);
            }
            pile_no += 1;
        }
    }
    stacks
}

fn parse_inst(instructions: &str) -> Vec<(usize,usize,usize)> {
    let mut inst = Vec::new();
    for i in instructions.lines() {
        let t: Vec<&str> = i.split(' ').collect();
        let c = t[1].parse::<usize>().unwrap();
        let f = t[3].parse::<usize>().unwrap()-1;
        let t = t[5].parse::<usize>().unwrap()-1;
        inst.push((c,f,t));
    }
    inst
}

fn parse_out(stacks: Vec<Vec<&str>>) -> String {
    let mut ans = String::from("");
    for mut i in stacks {
        ans.push(i.pop()
            .unwrap()
            .chars()
            .nth(1)
            .unwrap());
    }
    ans
}

fn parse_out_functional(stacks: Vec<Vec<&str>>) -> String {
    stacks
        .into_iter()
        .map(|mut i| i.pop()
        .unwrap()
        .chars()
        .nth(1)
        .unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&fs::read_to_string("input copy.txt").unwrap()), "CMZ");
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&fs::read_to_string("input.txt").unwrap()), "SHMSDGZVC");
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&fs::read_to_string("input copy.txt").unwrap()), "MCD");
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&fs::read_to_string("input.txt").unwrap()), "VRZGHDFBQ");
    }
}