use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

use std::fs;

fn main() {
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
    let start_time = Instant::now();
    println!("Start time: {:?}", start_time);
    let cave = parse_rooms(input);
    let djik_cave = all_the_djikstras(cave.clone());
    let (bit_cave, _) = bitize_keys(djik_cave.clone());
    let res = breadth_first_search(&bit_cave, 1, max_time, 0);
    println!("End time: {:?}", (start_time.elapsed().as_secs() as f32));
    res
}

fn part_2(input: &str, max_time: i32) -> i32 {
    let start_time = Instant::now();
    println!("Start time: {:?}", start_time);
    let cave = parse_rooms(input);
    let djik_cave = all_the_djikstras(cave.clone());
    let (bit_cave, max_bit) = bitize_keys(djik_cave.clone());
    let states = seed_states(max_bit);
    let res = plant_seeds(bit_cave, max_time, states, max_bit);
    println!("End time: {:?}", (start_time.elapsed().as_secs() as f32));
    res
}

fn plant_seeds(
    bit_cave: HashMap<i16, (i32, HashMap<i16, i32>)>,
    max_time: i32,
    states: HashSet<i16>,
    max_bit: i16,
) -> i32 {
    let mut max = 0;
    let mut max_seed = 0;
    for seed in states.clone() {
        if !max_bit & seed == 0 {
            let el_seed = !seed & max_bit;
            let me = breadth_first_search(&bit_cave, 1, max_time, seed);
            let elephant = breadth_first_search(&bit_cave, 1, max_time, el_seed);
            if max < me + elephant {
                max = me + elephant;
                max_seed = seed.clone();
            }
        }
    }
    println!(
        "Max seed: {:#018b} Inverse {:#018b}",
        max_seed,
        !max_seed & max_bit
    );
    max
}

fn seed_states(max_bit: i16) -> HashSet<i16> {
    let mut states = HashSet::new();
    let mut current_state: i16 = 0;
    let end_state: i16 = i16::MAX; //sign bit is AA, we can ignore it and use it as u15

    // create all bitpatterns
    while end_state > current_state {
        current_state += 1;
        // set a limit on uneven distributions, ie smaller set must have x rooms
        if !states.contains(&!current_state)
            && current_state.count_ones() > max_bit.count_ones() / 2
        {
            // WARNING hard coded cutof
            states.insert(current_state);
        }
    }

    // return patterns, starting pattern describes EXCLUDED rooms
    // answer is the best solution for both the pattern and its inverse
    states
}

//need cashing to avoid recalculating on single tracks
// create function that calculates values for a solo-state and saves it to a hashmap as cashe

fn breadth_first_search(
    cave: &HashMap<i16, (i32, HashMap<i16, i32>)>,
    start: i16,
    max_time: i32,
    visited: i16,
) -> i32 {
    //room names have been translated to bit patterns, where every bit corresponds to one room
    #[derive(Clone, Eq)]
    struct State {
        room: i16,
        flow: i32,
        pressure: i32,
        time: i32,
        visited: i16,
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

    let root: State = State {
        room: start,
        flow: 0,
        pressure: 0,
        time: 0,
        visited: visited | start,
    };
    let mut max_rooms = 0;
    for (room, _) in cave {
        max_rooms = max_rooms | room;
    }
    //println!("Max rooms: {:#018b}", max_rooms);
    let mut queue: Vec<State> = Vec::new();
    queue.push(root);
    while queue.len() > 0 {
        let current = queue.pop().unwrap();
        for new_room in cave.keys() {
            // if visited is 0 on room bit
            if (current.visited & new_room) == 0 {
                // +1 is cost to open valve
                let distance = cave[&current.room].1[new_room] + 1;
                let new_time = current.time + distance;
                // Check for max time
                if new_time >= max_time {
                    //Times up, count out remaining pressure
                    let new_pressure = current.pressure + current.flow * (max_time - current.time);
                    //See if we have a new max and update if so
                    if new_pressure > max_pressure {
                        max_pressure = new_pressure;
                    }
                    continue;
                }
                let new_visited = new_room | current.visited; //add to pattern
                let new_flow = current.flow + cave[new_room].0;
                //check for visited all rooms
                if new_visited == max_rooms {
                    //We have visited all rooms, count out remaining pressure
                    // first travel and open, then when open
                    let new_pressure = current.pressure
                        + current.flow * distance
                        + new_flow * (max_time - new_time);
                    //See if we have a new max and update if so
                    if new_pressure > max_pressure {
                        max_pressure = new_pressure;
                    }
                    continue;
                }

                // Gain pressure from flow for travel + 1 for opening
                let new_pressure = current.pressure + current.flow * distance;

                let new_state = State {
                    room: *new_room,
                    flow: new_flow,
                    pressure: new_pressure,
                    time: new_time,
                    visited: new_visited,
                };
                queue.push(new_state);
            }
        }
    }
    max_pressure
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
fn bitize_keys(
    cave: HashMap<String, (i32, HashMap<String, i32>)>,
) -> (HashMap<i16, (i32, HashMap<i16, i32>)>, i16) {
    let mut new_cave: HashMap<i16, (i32, HashMap<i16, i32>)> = HashMap::new();
    let mut rooms = cave.keys().collect::<Vec<&String>>();
    let mut room = 1;
    let mut translations = HashMap::new();
    rooms.sort();
    let mut max_bit = 0;
    //rooms.reverse();
    for r in rooms {
        translations.insert(r.to_string(), room);
        max_bit = room | max_bit;
        room = room << 1;
    }

    for (room, cont) in &cave {
        new_cave.insert(translations[room], (cont.0, HashMap::new()));

        for (k, v) in cave[room].1.clone() {
            new_cave
                .get_mut(&translations[room])
                .unwrap()
                .1
                .insert(translations[&k], v);
        }
    }
    (new_cave, max_bit)
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

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt"), 26), 2191);
    }
}
