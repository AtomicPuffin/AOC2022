#![allow(unused)]
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

fn part_1(input: &str) -> i32 {
    let (mut cave, max) = rocks(input);
    //print_cave(cave.clone());
    let mut fell_off = false;
    let mut resting = 0;
    while !fell_off {
        (cave, fell_off) = fall_1(cave, (500, 0), max);
        if !fell_off {
            resting += 1;
        }
    }
    //print_cave(cave.clone());
    resting
}

fn part_2(input: &str) -> i32 {
    let (mut cave, mut max) = rocks(input);
    max = max + 2;
    let mut full = false;
    let mut resting = 0;
    while !full {
        resting += 1;
        (cave, full) = fall_2(cave, (500, 0), max);
    }
    resting
}

fn rocks(input: &str) -> (HashSet<(i32, i32)>, i32) {
    // create grid
    let mut max_y = 0;
    let mut cave: HashSet<(i32, i32)> = HashSet::new();
    for l in input.lines() {
        let mut corners: Vec<&str> = l.split(" -> ").collect();
        let mut prev = corners.pop().unwrap().split(",");
        let mut px = prev.next().unwrap().parse::<i32>().unwrap();
        let mut py = prev.next().unwrap().parse::<i32>().unwrap();
        max_y = max_y.max(py);
        while let Some(c) = corners.pop() {
            let mut next = c.split(",");
            let nx = next.next().unwrap().parse::<i32>().unwrap();
            let ny = next.next().unwrap().parse::<i32>().unwrap();
            max_y = max_y.max(ny);

            // if x1 == x2 loop y, else loop x
            if px == nx {
                let from = py.min(ny);
                let to = py.max(ny);
                for y in from..to + 1 {
                    cave.insert((nx, y));
                }
            } else {
                //Y must be equal
                let from = px.min(nx);
                let to = px.max(nx);
                for x in from..to + 1 {
                    cave.insert((x, ny));
                }
            }
            px = nx;
            py = ny;
        }
    }
    (cave, max_y)
}

fn fall_1(
    mut cave: HashSet<(i32, i32)>,
    start: (i32, i32),
    max_y: i32,
) -> (HashSet<(i32, i32)>, bool) {
    // decides where next unit of sand falls
    // check below, check left, check right, if all blocked stop
    let mut pos = start;
    loop {
        if cave.contains(&(pos.0, pos.1 + 1)) {
            if cave.contains(&(pos.0 - 1, pos.1 + 1)) {
                if cave.contains(&(pos.0 + 1, pos.1 + 1)) {
                    cave.insert(pos); //stuck
                    return (cave, false);
                } else {
                    // right free
                    pos = (pos.0 + 1, pos.1 + 1);
                }
            } else {
                // left free
                pos = (pos.0 - 1, pos.1 + 1);
            }
        } else {
            // down free
            pos = (pos.0, pos.1 + 1);
        }
        if pos.1 > max_y {
            return (cave, true);
        }
    }
}

fn fall_2(
    mut cave: HashSet<(i32, i32)>,
    start: (i32, i32),
    max_y: i32,
) -> (HashSet<(i32, i32)>, bool) {
    // decides where next unit of sand falls
    // check below, check left, check right, if all blocked stop
    let mut pos = start;
    loop {
        if pos.1 == max_y - 1 {
            cave.insert(pos); //stuck on floor
            return (cave, false);
        }
        if cave.contains(&(pos.0, pos.1 + 1)) {
            if cave.contains(&(pos.0 - 1, pos.1 + 1)) {
                if cave.contains(&(pos.0 + 1, pos.1 + 1)) {
                    if pos == (500, 0) {
                        return (cave, true);
                    }
                    cave.insert(pos); //stuck
                    return (cave, false);
                } else {
                    // right free
                    pos = (pos.0 + 1, pos.1 + 1);
                }
            } else {
                // left free
                pos = (pos.0 - 1, pos.1 + 1);
            }
        } else {
            // down free
            pos = (pos.0, pos.1 + 1);
        }
    }
}

fn print_cave(cave: HashSet<(i32, i32)>) {
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut min_x = i32::MAX;
    let min_y = 0;

    for i in &cave {
        if i.0 < min_x {
            min_x = i.0;
        }
        if i.0 > max_x {
            max_x = i.0;
        }
        if i.1 > max_y {
            max_y = i.1;
        }
    }

    for row in min_y..max_y + 1 {
        let mut out = String::new();
        for col in min_x..max_x + 1 {
            if cave.contains(&(col, row)) {
                out.push('#');
            } else {
                out.push('.');
            }
        }
        println!("{}", out)
    }
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
        assert_eq!(part_1(&read_file("input copy.txt")), 24);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 728);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 93);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 27623);
    }
}
