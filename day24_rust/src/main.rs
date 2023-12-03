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
    let pos = (0, -1);
    let mut queue = HashSet::new();
    let mut round = 0;
    queue.insert(pos);
    let mut finish;
    let (mut winds, max) = parse_input(input);
    //print_map(&winds, max);
    let goal = (max.0 - 1, max.1);
    loop {
        winds = blow(winds, max);
        //print_map(&winds, max);
        (queue, finish) = find_options(&winds, max, &queue, goal);
        round += 1;
        if finish {
            break;
        }
    }
    round
}

fn part_2(input: &str) -> i64 {
    let pos = (0, -1);
    let mut queue = HashSet::new();
    let mut round = 0;
    queue.insert(pos);
    let mut finish;
    let (mut winds, max) = parse_input(input);
    //print_map(&winds, max);
    let mut iteration = 0;
    let mut goal = (max.0 - 1, max.1);
    loop {
        winds = blow(winds, max);
        //print_map(&winds, max);
        (queue, finish) = find_options(&winds, max, &queue, goal);
        round += 1;
        if finish {
            // assumes first arrival on each is optimal
            println!("GOAL ##### {}", round);
            if iteration == 0 {
                goal = (0, -1);
                iteration += 1;
                queue.clear();
                queue.insert((max.0 - 1, max.1));
            } else if iteration == 1 {
                goal = (max.0 - 1, max.1);
                iteration += 1;
                queue.clear();
                queue.insert((0, -1));
            } else if iteration == 2 {
                break;
            }
        }
    }
    round
}

fn blow(winds: HashMap<(i64, i64), Vec<char>>, max: (i64, i64)) -> HashMap<(i64, i64), Vec<char>> {
    //move all winds one step
    let mut winds_next: HashMap<(i64, i64), Vec<char>> = HashMap::new();
    for (p, wnds) in winds {
        for w in wnds {
            let pos;
            match w {
                '<' => pos = ((p.0 - 1).rem_euclid(max.0), p.1),
                '>' => pos = ((p.0 + 1).rem_euclid(max.0), p.1),
                '^' => pos = (p.0, (p.1 - 1).rem_euclid(max.1)),
                'v' => pos = (p.0, (p.1 + 1).rem_euclid(max.1)),
                _ => panic!("Wind error"),
            }
            if winds_next.contains_key(&pos) {
                winds_next.get_mut(&pos).unwrap().push(w);
            } else {
                winds_next.insert(pos, vec![w]);
            }
        }
    }
    winds_next
}

fn find_options(
    winds: &HashMap<(i64, i64), Vec<char>>,
    max: (i64, i64),
    queue: &HashSet<(i64, i64)>,
    goal: (i64, i64),
) -> (HashSet<(i64, i64)>, bool) {
    //find all possible next steps
    let mut finish = false;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0), (0, 0)];
    let mut options = HashSet::new();
    for o in queue {
        for d in directions {
            let x = o.0 + d.0;
            let y = o.1 + d.1;
            if x == goal.0 && y == goal.1 {
                finish = true;
            }
            if x == 0 && y == -1 {
                //we can stay in start/end positions and wait for an opening
                options.insert((x, y));
            }
            if x == max.0 - 1 && y == max.1 {
                options.insert((x, y));
            }
            if x >= 0 && x < max.0 && y >= 0 && y < max.1 {
                if !winds.contains_key(&(x, y)) {
                    options.insert((x, y));
                }
            }
        }
    }
    (options, finish)
}

fn parse_input(input: &str) -> (HashMap<(i64, i64), Vec<char>>, (i64, i64)) {
    let mut winds = HashMap::new();
    let mut y = -1;
    let mut max = (0, 0);
    for l in input.lines() {
        let mut x = -1;
        for c in l.chars() {
            if ['<', '>', '^', 'v'].contains(&c) {
                winds.insert((x, y), vec![c]);
            }
            max.0 = max.0.max(x);
            x += 1;
        }
        max.1 = max.1.max(y);
        y += 1;
    }
    (winds, max)
}

fn print_map(winds: &HashMap<(i64, i64), Vec<char>>, max: (i64, i64)) {
    for y in 0..max.1 {
        let mut line = "".to_string();
        for x in 0..max.0 {
            if winds.contains_key(&(x, y)) {
                if winds[&(x, y)].len() > 1 {
                    line.push(char::from_digit(winds[&(x, y)].len() as u32, 10).unwrap());
                } else {
                    line.push(winds[&(x, y)][0]);
                }
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
    println!("");
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
        assert_eq!(part_1(&read_file("input copy.txt")), 18);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 274);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 54);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 839);
    }
}
