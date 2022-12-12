use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> usize {
    let monkeys = parse_monkey(input);
    execute_rounds(monkeys, 20, &(|x| x / 3))
}

fn part_2 (input: &str) -> usize {
    let monkeys = parse_monkey(input);
    let mut product = 1;
    for i in &monkeys {
        product = product * i.test;
    }
    execute_rounds(monkeys, 10_000, &(|x| x % product))
}

fn execute_rounds(mut monkeys: Vec<Monkey>, rounds: usize, func: &dyn Fn(usize) -> usize) -> usize {
    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            let items = &monkeys[m].items.clone();
            for i in items {
                monkeys[m].inspections += 1;
                let y: usize;
                if monkeys[m].operand == "old" {
                    y = i.clone();
                } else {
                    y = monkeys[m].operand.parse::<usize>().unwrap();
                }
                let mut ni = 0;
                if monkeys[m].operator == '+' {
                    ni = func(i + y);
                } else if monkeys[m].operator == '*' {
                    ni = func(i * y);
                }
                if &ni % monkeys[m].test == 0 {
                    let tt = monkeys[m].target_true;
                    monkeys[tt].items.push(ni);
                } else {
                    let tf = monkeys[m].target_false;
                    monkeys[tf].items.push(ni);
                }
            }
            monkeys[m].items.clear();
        }
    }
    let mut insp = Vec::new();
    for i in monkeys {
        insp.push(i.inspections);
    }
    insp.sort();
    insp.pop().unwrap() * insp.pop().unwrap()
}

struct Monkey {
    items: Vec<usize>,
    operator: char,
    operand: String,
    test: usize,
    target_true: usize,
    target_false: usize,
    inspections: usize,
}

fn parse_monkey (input: &str) -> Vec<Monkey> {
    let mut position = 0;
    let mut monkeys = Vec::new();
    let mut i = Vec::new();
    let mut or = ' ';
    let mut o = "";
    let mut t = 0;
    let mut tt = 0;
    let mut tf: usize;
    for l in input.lines() {
        let mut inst: Vec<&str> = l.split([' ', ',']).collect();
        match position % 7 {
            0 => position +=1,
            1 => {position +=1; 
                let mut even = 0; 
                for n in &mut inst[4..] {
                    if even % 2 == 0 {
                        i.push(n.parse::<usize>().unwrap());
                    }
                    even += 1;
                }},
            2 => {position +=1;
                or = inst[6].chars().next().unwrap();
                o = inst[7];
            },
            3 => {position +=1; t = inst[5].parse::<usize>().unwrap();},
            4 => {position +=1; tt = inst[9].parse::<usize>().unwrap();},
            5 => {position +=1; 
                tf = inst[9].parse::<usize>().unwrap();
                let m = Monkey {items: i.clone(), 
                    operator: or, 
                    operand: o.to_string(), 
                    test: t, 
                    target_true: tt, 
                    target_false: tf,
                    inspections: 0,
                    };
                i.clear();
                monkeys.push(m);},
            6 => position +=1,
            _ => println!("Modulo operator wrong"),
        }
    }
    monkeys
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("input copy.txt")), 10605);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 55944);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 2713310158);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 15117269860);
    }
}