#![allow(dead_code, unreachable_code, unused_imports, unused_variables)]
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
//use std::env;
use std::fs;

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    println!(
        "Answer to Part 1 test: {}",
        part_1(&read_file("input copy.txt"), 30)
    );
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt"), 30));
    println!(
        "Answer to Part 2 test: {}",
        part_2(&read_file("input copy.txt"), 26)
    );
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt"), 26));
}

fn part_1(input: &str, max_time: i32) -> i32 {
    let cave = parse_rooms(input);
    let djik_cave = all_the_djikstras(cave.clone());
    //breadth_first_search(djik_cave, "AA".to_string(), max_time)
    breadth_first_search_x2(djik_cave, "AA".to_string(), 50, max_time)
}

fn part_2(input: &str, max_time: i32) -> i32 {
    let cave = parse_rooms(input);
    let djik_cave = all_the_djikstras(cave.clone());
    breadth_first_search_x2(djik_cave, "AA".to_string(), 0, max_time)
}

/*fn breadth_first_search(
    cave: HashMap<String, (i32, HashMap<String, i32>)>,
    start: String,
    max_time: i32,
) -> i32 {
    #[derive(Clone, Eq)]
    struct State {
        room: String,
        flow: i32,
        pressure: i32,
        time: i32,
        visited: HashSet<String>,
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.pressure.cmp(&other.pressure)
        }
    }
    impl PartialEq for State {
        fn eq(&self, other: &Self) -> bool {
            (&self.visited, &self.room).eq(&(&other.visited, &other.room))
        }
    }
    let mut max_pressure = 0;
    let mut winning_state: State = State {
        room: "".to_string(),
        flow: 0,
        pressure: 0,
        time: 0,
        visited: HashSet::new(),
    };
    let numer_of_rooms = cave.len();

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.clone());
    let root: State = State {
        room: start,
        flow: 0,
        pressure: 0,
        time: 0,
        visited: visited,
    };
    let mut queue: Vec<State> = Vec::new();
    queue.push(root);
    while queue.len() > 0 {
        let current = queue.pop().unwrap();
        for new_room in cave.keys() {
            if !current.visited.contains(new_room) {
                let distance = cave[&current.room].1[new_room] + 1;
                let new_time = current.time + distance;
                // Check for max time
                if new_time >= max_time {
                    //Times up, count out remaining pressure
                    let new_pressure = current.pressure + current.flow * (max_time - current.time);
                    //See if we have a new max and update if so
                    if new_pressure > max_pressure {
                        winning_state = current.clone();
                        max_pressure = new_pressure;
                    }
                    continue;
                }
                let mut new_visited = current.visited.clone();
                new_visited.insert(new_room.to_string());
                let new_flow = current.flow + cave[new_room].0;
                //check for visited all rooms
                if new_visited.len() == numer_of_rooms {
                    //We have visited all rooms, count out remaining pressure
                    // first travel and open, then when open
                    let new_pressure = current.pressure
                        + current.flow * distance
                        + new_flow * (max_time - new_time);
                    //See if we have a new max and update if so
                    if new_pressure > max_pressure {
                        winning_state = current.clone();
                        max_pressure = new_pressure;
                    }
                    continue;
                }

                // Gain pressure from flow for travel + 1 for opening
                let new_pressure = current.pressure + current.flow * distance;
                let new_state = State {
                    room: new_room.to_string(),
                    flow: new_flow,
                    pressure: new_pressure,
                    time: new_time,
                    visited: new_visited,
                };
                queue.push(new_state);
            }
        }
    }
    println!(
        "Winning state: room {} flow {} pressure {} time {} ",
        winning_state.room, winning_state.flow, winning_state.pressure, winning_state.time
    );
    max_pressure
}*/

#[derive(Clone, Eq)]
struct State {
    player1: String,
    arrive1: i32,
    player2: String,
    arrive2: i32,
    flow: i32,
    pressure: i32,
    time: i32,
    visited: HashSet<String>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pressure.cmp(&other.pressure)
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        (&self.visited, &self.player1, &self.player2).eq(&(
            &other.visited,
            &other.player1,
            &other.player2,
        )) || (&self.visited, &self.player1, &self.player2).eq(&(
            &other.visited,
            &other.player2,
            &other.player1,
        ))
    }
}

fn breadth_first_search_x2(
    cave: HashMap<String, (i32, HashMap<String, i32>)>,
    start: String,
    arrive2: i32,
    max_time: i32,
) -> i32 {
    let mut visited_states: HashSet<State>;
    let mut max_pressure = 0;
    let mut winning_state: State = State {
        player1: start.clone(),
        arrive1: 0,
        player2: start.clone(),
        arrive2: 0,
        flow: 0,
        pressure: 0,
        time: 0,
        visited: HashSet::new(),
    };
    let number_of_rooms = cave.len();

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.clone());
    let root: State = State {
        player1: start.to_string(),
        arrive1: 0,
        player2: start.to_string(),
        arrive2: arrive2,
        flow: 0,
        pressure: 0,
        time: 0,
        visited: visited,
    };

    let mut queue: Vec<State> = Vec::new();
    queue.push(root);
    while queue.len() > 0 {
        let mut current = queue.pop().unwrap();
        //let the State arrive at it's destination(s)
        let mut now = current.arrive1.min(current.arrive2);
        let mut times_up = false;
        if now > max_time {
            times_up = true;
            now = max_time;
        }
        let time_to_here = now - current.time;
        //update with either or both rooms if they arrived at the same time
        let mut new_flow = current.flow;
        //if both if miss, we walk beyond max time

        if now == current.arrive1 && now != current.time {
            new_flow += cave[&current.player1].0;
        }
        if now == current.arrive2 && now != current.time {
            new_flow += cave[&current.player2].0;
        }
        let new_pressure = current.pressure + current.flow * time_to_here;
        current.time = now;
        current.pressure = new_pressure;
        current.flow = new_flow;

        if current.visited.len() == number_of_rooms {
            if current.arrive1.max(current.arrive2) < max_time {
                //Other still has time to arrive
                if current.arrive1 > current.arrive2 {
                    current.arrive2 = max_time + 1;
                } else {
                    current.arrive1 = max_time + 1;
                }
                queue.push(current.clone());
                continue;
            }
            current.pressure += current.flow * (max_time - current.time);
            times_up = true;
        }

        if times_up {
            if current.pressure > max_pressure {
                winning_state = current.clone();
                max_pressure = current.pressure;
            }
            continue;
        }
        let mut candidates: Vec<(i32, State, i32, String)> = Vec::new();
        for new_room in cave.keys() {
            if !current.visited.contains(new_room) {
                if current.arrive1 > current.arrive2 {
                    let weight = cave[new_room].0
                        * 0.max(max_time - cave[&current.player2].1[new_room] - 1 - current.time);
                    candidates.push((weight, current.clone(), 2, new_room.to_string()));
                } else if current.arrive1 < current.arrive2 {
                    let weight = cave[new_room].0
                        * 0.max(max_time - cave[&current.player1].1[new_room] - 1 - current.time);
                    candidates.push((weight, current.clone(), 1, new_room.to_string()));
                } else {
                    let weight1 = cave[new_room].0
                        * 0.max(max_time - cave[&current.player1].1[new_room] - 1 - current.time);
                    let weight2 = cave[new_room].0
                        * 0.max(max_time - cave[&current.player2].1[new_room] - 1 - current.time);
                    if weight1 < weight2 {
                        candidates.push((weight2, current.clone(), 2, new_room.to_string()));
                    } else if weight1 > weight2 {
                        candidates.push((weight1, current.clone(), 1, new_room.to_string()));
                    } else {
                        candidates.push((weight1, current.clone(), 1, new_room.to_string()));
                        candidates.push((weight2, current.clone(), 2, new_room.to_string()));
                    }
                }
            }
        }
        let mut counter = 14.min(14.max(candidates.len() / 2));
        let total_flow = 205;
        candidates.sort_by(|a, b| a.0.cmp(&b.0));
        while candidates.len() > 0 {
            //println!("candidates {}", candidates.len());
            let candidate = candidates.pop().unwrap();
            if current.time >= max_time / 2 && current.flow < total_flow / 4 {
                //println!("Pruned");
                break;
            }
            if counter == 0 {
                break;
            }

            let mut new_visited = current.visited.clone();
            new_visited.insert(candidate.3.to_string());
            if candidate.2 == 1 {
                let distance = cave[&current.player1].1[&candidate.3] + 1;
                let new_arrive = current.arrive1 + distance;
                let new_state = State {
                    player1: candidate.3.to_string(),
                    player2: current.player2.clone(),
                    arrive1: new_arrive,
                    arrive2: current.arrive2,
                    flow: current.flow,
                    pressure: current.pressure,
                    time: current.time,
                    visited: new_visited.clone(),
                };
                queue.push(new_state);
            } else {
                let distance = cave[&current.player2].1[&candidate.3] + 1;
                let new_arrive = current.arrive2 + distance;
                let new_state = State {
                    player1: current.player1.clone(),
                    player2: candidate.3.to_string(),
                    arrive1: current.arrive1,
                    arrive2: new_arrive,
                    flow: current.flow,
                    pressure: current.pressure,
                    time: current.time,
                    visited: new_visited.clone(),
                };
                queue.push(new_state);
            }
            counter -= 1;
        }
    }
    println!(
        "Winning state: room1 {} room2 {} flow {} pressure {} time1 {} time2 {}",
        winning_state.player1,
        winning_state.player2,
        winning_state.flow,
        winning_state.pressure,
        winning_state.arrive1,
        winning_state.arrive2
    );

    max_pressure
}

fn filter_dfs(distance: i32, flow: i32) -> bool {
    return if (flow as f32 / distance as f32) < 0.5 {
        false
    } else {
        true
    };
}

fn parse_rooms(input: &str) -> HashMap<String, (i32, HashMap<String, i32>)> {
    // to_optimize (room (rate (neighbour, distance)))
    let mut to_optimize: HashMap<String, (i32, HashMap<String, i32>)> = HashMap::new();
    for s in input.lines() {
        let raw: Vec<&str> = s.split(" ").collect();
        let name = raw[1].to_string();
        let mut next: HashMap<String, i32> = HashMap::new();
        for i in &raw[9..] {
            next.insert(i[0..2].to_string(), 1);
        }
        let rate = raw[4][5..raw[4].len() - 1].parse::<i32>().unwrap();
        to_optimize.insert(name, (rate, next));
    }

    let mut not_done = true;

    while not_done {
        // replace zero rate rooms with travel costs
        not_done = false;
        let mut swap: String = String::new();
        let mut target: HashMap<String, i32> = HashMap::new();
        for (r, v) in &to_optimize {
            //get first item with rate 0
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
    //let start = to_optimize["AA"].1.clone();
    //to_optimize = prune(to_optimize, "AA".to_string(), start.clone());
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
    to_optimize
}

fn prune(
    mut cave: HashMap<String, (i32, HashMap<String, i32>)>,
    swap: String,
    target: HashMap<String, i32>,
) -> HashMap<String, (i32, HashMap<String, i32>)> {
    cave.remove(&swap);
    for (r, dr) in &target {
        cave.get_mut(r).unwrap().1.remove(&swap);
        for (t, dt) in &target {
            if t != r {
                let mut dn = dr + dt;
                if cave[r].1.contains_key(t) {
                    //if already connected take smaller path
                    dn = cave[r].1[t].min(dn);
                }
                cave.get_mut(r).unwrap().1.insert(t.to_string(), dn);
            }
        }
    }
    cave
}

fn djikstra(
    cave: HashMap<String, (i32, HashMap<String, i32>)>,
    start: String,
) -> HashMap<String, i32> {
    let mut dist: HashMap<String, i32> = HashMap::new();
    for room in cave.keys() {
        dist.insert(room.to_string(), i32::MAX);
    }
    let mut heap = BinaryHeap::new();

    #[derive(Clone, Eq, PartialEq)]
    struct State {
        cost: i32,
        room: String,
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.cost.cmp(&other.cost)
        }
    }

    heap.push(State {
        cost: 0,
        room: start,
    });

    while let Some(State { cost, room }) = heap.pop() {
        if cost > dist[&room] {
            continue;
        }
        for (room, weight) in &cave[&room].1 {
            let next = State {
                cost: cost + weight,
                room: room.to_string(),
            };

            if next.cost < dist[&next.room] {
                heap.push(next.clone());
                dist.insert(next.clone().room.to_string(), next.clone().cost);
            }
        }
    }
    dist
}

fn all_the_djikstras(
    cave: HashMap<String, (i32, HashMap<String, i32>)>,
) -> HashMap<String, (i32, HashMap<String, i32>)> {
    let mut new_cave: HashMap<String, (i32, HashMap<String, i32>)> = HashMap::new();
    for room in cave.keys() {
        let distances = djikstra(cave.clone(), room.to_string());
        new_cave.insert(room.to_string(), (cave[room].0, distances.clone()));
        /*println!("Room {}", room);
        for (k, v) in distances {
            println!("R {} D {} F {}", k, v, cave[&k].0);
        }*/
    }
    new_cave
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
        assert_eq!(part_1(&read_file("input copy.txt"), 30), 1651);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt"), 30), 1559);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt"), 26), 1707);
    }

    #[ignore]
    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt"), 30), 2191);
    }
}
