
use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

#[derive(Copy, Clone)]
struct Blueprint {
    id: i32,
    ore: i32,
    clay: i32,
    obsidian: (i32,i32),
    geode: (i32,i32),
}

#[derive(Copy, Clone)]
struct Factory {
    next: i32,
    bp: Blueprint,
    clock: i32,
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_rob: i32,
    clay_rob: i32,
    obsidian_rob: i32,
    geode_rob: i32,
}

impl Factory {
    fn collect(&mut self) {
        self.ore += self.ore_rob;
        self.clay += self.clay_rob;
        self.obsidian += self.obsidian_rob;
        self.geode += self.geode_rob;
    }
}

fn part_1 (input: &str) -> i32 {
    let blueprints = parse_bp(input);
    let mut answer = 0;
    for bp in blueprints {
        let fct = Factory {
            next: -1,
            bp: bp,
            clock: 25,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_rob: 1,
            clay_rob: 0,
            obsidian_rob: 0,
            geode_rob: 0,
        };
        let quality = branch(fct);
        answer += bp.id * quality;
    }
    answer
}

fn part_2 (input: &str) -> i32 {
    let blueprints = parse_bp(input);
    let mut answer = 1;
    let mut counter = 0;
    for bp in blueprints {
        counter += 1;
        let fct = Factory {
            next: -1,
            bp: bp,
            clock: 33,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_rob: 1,
            clay_rob: 0,
            obsidian_rob: 0,
            geode_rob: 0,
        };
        let quality = branch(fct);
        answer = answer * quality;
        if counter == 3 {
            break;
        }
    }
    answer
}

fn branch(mut fct: Factory) -> i32 {
    fct.clock -= 1; //countdown clock
    if fct.clock == 0 {
        return fct.geode
    }

    // produce the designated robot if resources available
    match fct.next {
        3 => { if fct.bp.geode.0 <= fct.ore && fct.bp.geode.1 <= fct.obsidian {
            fct.obsidian -= fct.bp.geode.1;
            fct.ore -= fct.bp.geode.0;
            fct.geode_rob += 1;
            fct.geode -= 1; //compensate for instruction out of order
            fct.next = -1;
        }},
        2 => {if fct.bp.obsidian.0 <= fct.ore && fct.bp.obsidian.1 <= fct.clay {
            fct.clay -= fct.bp.obsidian.1;
            fct.ore -= fct.bp.obsidian.0;
            fct.obsidian_rob += 1;
            fct.obsidian -= 1;
            fct.next = -1;
        }},
        1 => {if fct.ore >= fct.bp.clay {
            fct.ore -= fct.bp.clay;
            fct.clay_rob += 1;
            fct.clay -= 1;
            fct.next = -1;
        }},
        0 => {if fct.ore >= fct.bp.ore {
            fct.ore -= fct.bp.ore;
            fct.ore_rob += 1;
            fct.ore -= 1;
            fct.next = -1;
        }},
        _ => {},
    }

    fct.collect();

    let mut branches = Vec::new();

    if fct.next == -1 {
        let mut next = Vec::new();
        if fct.ore <= 5  { //prune by not producing robots with ample stock
            next.push(0);
        }        
        if fct.clay <= fct.bp.obsidian.1 +1  {
            next.push(1);
        }
        if fct.clay_rob > 0 && fct.obsidian <= fct.bp.geode.1 +1  {
            next.push(2);
        }
        if fct.obsidian_rob > 0 {
            next.push(3);
        }
        for i in next {
            let mut fct_temp = fct.clone();
            fct_temp.next = i;
            branches.push(branch(fct_temp));
        }
       
        branches.sort();
        return branches.pop().unwrap()
    } else {
        return branch(fct)
    }
}

fn parse_bp(input: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();
    let lines = input.lines();
    let mut id = 1;
    for l in lines {
        let raw: Vec<&str> = l.split(" ").collect();
        let bp = Blueprint {
            id: id,
            ore: raw[6].parse::<i32>().unwrap(),
            clay: raw[12].parse::<i32>().unwrap(),
            obsidian: (raw[18].parse::<i32>().unwrap(), raw[21].parse::<i32>().unwrap()),
            geode: (raw[27].parse::<i32>().unwrap(), raw[30].parse::<i32>().unwrap()),
        };
        blueprints.push(bp);
        id += 1;
    }
    blueprints
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
        assert_eq!(part_1(&read_file("input copy.txt")), 33);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1306);
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