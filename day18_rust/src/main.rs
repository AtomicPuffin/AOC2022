use std::collections::HashSet;
use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

// chunk all into hashset
// iterate and look for neighbours

fn part_1 (input: &str) -> i32 {
    let (pond, _) = parse_cubes(input);
    count_sides_1(pond)
}

fn part_2 (input: &str) -> i32 {
    let (pond, edg) = parse_cubes(input);
    count_sides_2(pond, edg)
}

fn count_sides_1(pond: HashSet<[i32; 3]>) -> i32 {
    let test = vec![[1,0,0],[-1,0,0],[0,1,0],[0,-1,0],[0,0,1],[0,0,-1]];
    let mut counter = 0;
    for c in &pond {
        for p in &test {
            let t = [c[0]+ p[0], c[1]+ p[1], c[2]+ p[2]];
            if !pond.contains(&t) {
                counter += 1;
            }
        }
    }
    counter
}

fn count_sides_2(pond: HashSet<[i32; 3]>, edg: ((i32,i32), (i32,i32), (i32,i32))) -> i32 {
    let test = vec![[1,0,0],[-1,0,0],[0,1,0],[0,-1,0],[0,0,1],[0,0,-1]];
    let mut steam: HashSet<[i32; 3]> = HashSet::new();
    steam.insert([edg.0.0, edg.1.0, edg.2.0]);
    let mut counter = 0;
    let mut done = false;
    while !done {
        done = true;
        let mut new = HashSet::new();
        for s in &steam {
            for p in &test {
                let t = [s[0]+ p[0], s[1]+ p[1], s[2]+ p[2]];
                if !{t[0] < edg.0.0 || t[0] > edg.0.1 ||
                    t[1] < edg.1.0 || t[1] > edg.1.1 ||
                    t[2] < edg.2.0 || t[2] > edg.2.1} {
                        if !steam.contains(&t) && !pond.contains(&t) {//empty neighbour
                            new.insert(t);
                            done = false;
                        }
                }
            }
        }
        for s in new {
            for p in &test {
                let t = [s[0]+ p[0], s[1]+ p[1], s[2]+ p[2]];
                if pond.contains(&t) {
                    counter +=1;
                }
            steam.insert(s);
            }
        }
    }
    counter
}

fn parse_cubes(input: &str) -> (HashSet<[i32; 3]>, ((i32,i32), (i32,i32), (i32,i32))) {
    let mut max_x = (0,0);
    let mut max_y = (0,0);
    let mut max_z = (0,0);
    let mut out: HashSet<[i32; 3]> = HashSet::new();
    let lines = { input 
                    .lines()
                    .map(|l| l.split(","))
                };
    for mut i in lines {
        let x = i.next().unwrap().parse::<i32>().unwrap();
        let y = i.next().unwrap().parse::<i32>().unwrap();
        let z = i.next().unwrap().parse::<i32>().unwrap();
        max_x = (max_x.0.min(x), max_x.1.max(x));
        max_y = (max_y.0.min(y), max_y.1.max(y));
        max_z = (max_z.0.min(x), max_z.1.max(z));
        out.insert([x,y,z]);
    }
    let edg = ((max_x.0-1, max_x.1+1), (max_y.0-1, max_y.1+1), (max_z.0-1, max_z.1+1));
    (out, edg)
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
        assert_eq!(part_1(&read_file("input copy.txt")), 64);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 4300);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 58);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 2490);
    }
}