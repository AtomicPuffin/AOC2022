use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> i32 {
    let cycles = [20,60,100,140,180,220];
    let mut x = 1;
    let mut cycle = 0;
    let mut answer = 0;
    for l in input.lines() {
        let inst: Vec<&str> = l.split(' ').collect();
        if inst[0] == "addx" {
            for _ in 0..2 {
                cycle += 1;
                if cycles.contains(&cycle) {
                    answer += x * cycle;
                }
            }
            let y = inst[1].parse::<i32>().unwrap();
            x += y;
        } else if inst[0] == "noop" {
            cycle += 1;
            if cycles.contains(&cycle) {
                answer += x * cycle;
            }
        } else {
            println!("Invalid instruction");
        }
    }
    answer
}

fn part_2 (input: &str) -> i32 {
    let mut screen = [['.'; 40]; 6];
    let mut x:i32 = 1;
    let mut cycle: usize = 0;
    for l in input.lines() {
        let inst: Vec<&str> = l.split(' ').collect();
        if inst[0] == "addx" {
            for _ in 0..2 {
                if (cycle % 40) as i32 >= x-1 && (cycle % 40) as i32 <= x+1 {
                    screen[cycle/40][cycle-(cycle / 40 * 40)] = '#';
                }
                cycle += 1;
            }
            let y = inst[1].parse::<i32>().unwrap();
            x += y;
        } else if inst[0] == "noop" {
            if (cycle % 40) as i32 >= x-1 && (cycle % 40) as i32 <= x+1 {
                screen[cycle/40][cycle-(cycle / 40 * 40)] = '#';
            }
            cycle += 1;
        } else {
            println!("Invalid instruction");
        }
    }
    for l in screen {
        let mut line = "".to_string();
        for c in l {
            line.push(c);
        }
        println!("{}", line);
    }
    0
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
        assert_eq!(part_1(&read_file("input copy.txt")), 13140);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 11780);
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