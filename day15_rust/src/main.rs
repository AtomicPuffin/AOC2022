use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("input copy.txt"), 10)
    );
    println!(
        "Answer to Part 1: {}",
        part_1(&read_file("input.txt"), 2_000_000)
    );
    println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("input copy.txt"), 20)
    );
    println!(
        "Answer to Part 2: {}",
        part_2(&read_file("input.txt"), 4_000_000)
    );
}

//parse sensor beacon pairs (hashmap[sensor] beacon)
//calc dist for each pair, no other beacons on dist <=
// plot in hashset where beacons cannot be (remember where beacons present)
//calc hashset for Y = 2_000_000

fn part_1(input: &str, row: i32) -> i32 {
    let sensors = parse_sensors(input);
    count_empty_1(sensors, row)
}

// for each row, create edges of sensor reach as -1 or 1 with coord
// iterate all edges and calculate level (number of sensors overlapping)
// find spot inside field where level == 0

fn part_2(input: &str, max: i32) -> i64 {
    let sensors = parse_sensors(input);
    for row in 0..max {
        let x = count_empty_2(&sensors, row, max);
        if x != -1 {
            return x as i64 * 4_000_000 + row as i64;
        }
    }
    -1
}

fn count_empty_2(sensors: &HashMap<(i32, i32), (i32, i32)>, row: i32, max: i32) -> i32 {
    let mut changes: Vec<(i32, i32)> = Vec::new();

    for s in sensors {
        let manh = manhattan(*s.0, *s.1);
        if manh - (row - s.0 .1).abs() >= 0 {
            let smin = s.0 .0 - (manh - (row - s.0 .1).abs());
            let smax = s.0 .0 + (manh - (row - s.0 .1).abs());
            changes.push((-1, smin));
            changes.push((1, smax));
        }
    }

    let mut level = 0;
    changes.sort_by_key(|k| k.1);
    let mut candidate = -1;
    while let Some(c) = changes.pop() {
        if level == 0 && candidate != c.1 && candidate != c.1 - 1 && candidate != -1 {
            println!("Final x {} y {}", candidate, row);
            return candidate;
        }
        level += c.0;
        if level == 0 && c.1 >= 0 && c.1 <= max {
            candidate = c.1 - 1;
        }
    }
    -1
}

fn count_empty_1(sensors: HashMap<(i32, i32), (i32, i32)>, row: i32) -> i32 {
    let mut empty: HashSet<(i32, i32)> = HashSet::new();
    for (s, b) in sensors {
        let manh = manhattan(s, b);
        if s.1 + manh >= row && s.1 - manh <= row {
            // ignore if no overlap
            if s.1 == row {
                // catch empty iterator
                for x in s.0 - manh..s.0 + manh + 1 {
                    empty.insert((x, row));
                }
            } else {
                let diff = manh - (s.1 - row).abs(); //remaining manhattan after reaching row
                for x in s.0 - diff..s.0 + diff + 1 {
                    empty.insert((x, row));
                }
            }
        }
        empty.remove(&(b.0, b.1)); //remove beacon if on row
    }
    empty.len() as i32
}

fn parse_sensors(input: &str) -> HashMap<(i32, i32), (i32, i32)> {
    let mut sensors: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    for s in input.lines() {
        let raw: Vec<&str> = s.split(" ").collect();
        let sx = raw[2][2..raw[2].len() - 1].parse::<i32>().unwrap();
        let sy = raw[3][2..raw[3].len() - 1].parse::<i32>().unwrap();
        let bx = raw[8][2..raw[8].len() - 1].parse::<i32>().unwrap();
        let by = raw[9][2..raw[9].len()].parse::<i32>().unwrap();
        sensors.insert((sx, sy), (bx, by));
    }
    sensors
}

fn manhattan(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
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
        assert_eq!(part_1(&read_file("input copy.txt"), 10), 26);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt"), 2_000_000), 4907780);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt"), 20), 56000011);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt"), 4_000_000), 13639962836448);
    }
}
