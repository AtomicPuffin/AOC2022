#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]
use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> i32 {
    let (cave, start) = parse_rooms(input);
    let mut open_cut = 0;
    for (k,v) in &cave {
        open_cut = open_cut.max(v.0);
    }
    iter_tunnels(start, &cave, (0,0,0, open_cut/2, 30))
}

fn part_2 (input: &str) -> i32 {
    todo!()
}
fn iterate_room(current: &str, cave: &HashMap<String, (i32, HashMap<String, i32>)>, params: (i32,i32,i32,i32,i32)) -> i32 {
    let ( mut cycle, mut preassure, mut flow_t, open_cut, time) = params;
    if cycle >= time {
        return flow_t
    }

    let target = cave[current].1.clone();
    
    if target.len() > 0 { //more rooms left
        // send of recursion without opening
        let mut c = 0;
        if cave[current].0 < open_cut { //but only if rate is small
            c = iter_tunnels(target.clone(), &cave, (cycle, preassure, flow_t, open_cut, time));
        }
        // open vault, remove room, recurse
        let cave_c = prune(cave.clone(), current.to_string(), cave[current].1.clone());
        flow_t += preassure;
        preassure += cave[current].0;
        let o = iter_tunnels(target, &cave_c, (cycle+1, preassure, flow_t, open_cut, time));        
        return o.max(c)
    } else { //last room, open calculate
        flow_t += preassure;
        cycle += 1; //current cycle we open
        preassure += cave[current].0;
        return flow_t + preassure * (time-cycle)
    }
}

fn iter_tunnels(next: HashMap<String, i32>, cave: &HashMap<String, (i32, HashMap<String, i32>)>, params: (i32, i32,i32,i32,i32)) -> i32 {
    let (cycle, preassure, flow_t, open_cut, time) = params;
    let mut paths: Vec<i32> = Vec::new();
    for (r, c) in next {
        let mut c_temp = cycle;
        let mut f_temp = flow_t;
        for _ in 0..c {
            c_temp += 1;
            if c_temp > time {// so we dont count steps over 30
                break;
            }
            f_temp += preassure; 
        }
        paths.push(iterate_room(&r, &cave, (c_temp, preassure, f_temp, open_cut, time)));
    }
    paths.sort();
    return paths.pop().unwrap()
}

fn parse_rooms(input: &str) -> (HashMap<String, (i32, HashMap<String, i32>)>, HashMap<String, i32>) {
    // to_optimize (room (rate (neighbour, distance)))
    let mut to_optimize: HashMap<String, (i32, HashMap<String, i32>)> = HashMap::new();
    for s in input.lines() {
        let raw: Vec<&str> = s.split(" ").collect();
        let name = raw[1].to_string();
        let mut next: HashMap<String, i32> = HashMap::new();
        for i in &raw[9..] {
            next.insert(i[0..2].to_string(), 1);
        }
        let rate = raw[4][5..raw[4].len()-1].parse::<i32>().unwrap();
        to_optimize.insert(name, (rate, next));
        
    }

    let mut not_done = true;
    
    while not_done { // replace zero rate rooms with travel costs
        not_done = false;
        let mut swap: String = String::new();
        let mut target: HashMap<String, i32> = HashMap::new();
        for (r, v) in &to_optimize { //get first item with rate 0
            if v.0 == 0 && r != "AA" {
                swap = r.to_string();
                target = v.1.clone();
                not_done = true; //if we found one we keep looking
                break;
            }
        }
        if not_done {
            to_optimize = prune(to_optimize, swap, target);
        }
    }
    let start = to_optimize["AA"].1.clone();
    to_optimize = prune(to_optimize, "AA".to_string(), start.clone());
    /*for (k,v) in &to_optimize {
        println!("key {} #############rate {}", k, v.0);
        for (k,v) in &v.1 {
            println!("key {} cost {}", k, v);
        }
    }
    println!("{}", to_optimize.len());
    for i in &start {
        println!("start {} {}", i.0, i.1);
    }*/
    (to_optimize, start)
}

fn prune(mut cave: HashMap<String, (i32, HashMap<String, i32>)>, swap: String,  target: HashMap<String, i32>, ) -> HashMap<String, (i32, HashMap<String, i32>)> {
    
    cave.remove(&swap);
    for (r,dr) in &target {
        cave.get_mut(r).unwrap().1.remove(&swap);
        for (t, dt) in &target {
            if t != r {
                let mut dn = dr+dt;
                if cave[r].1.contains_key(t) { //if already connected take smaller path
                    dn = cave[r].1[t].min(dn);
                }
                cave.get_mut(r).unwrap().1.insert(t.to_string(), dn);
            }
        }
    }
    cave
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
        assert_eq!(part_1(&read_file("input copy.txt")), 1651);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1559);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 1707);
    }

    #[ignore]
    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), todo!());
    }
}