use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    println!(
        "Answer to Part 1 test: {}",
        part_1::<4>(&read_file("input copy.txt"))
    );

    println!(
        "Answer to Part 1: {}",
        part_1::<50>(&read_file("input.txt"))
    );
    //panic!("lets stop");
    println!(
        "Answer to Part 2 test: {}",
        part_2::<4>(&read_file("input copy.txt"))
    );
    //panic!("lets stop");
    println!(
        "Answer to Part 2: {}",
        part_2::<50>(&read_file("input.txt"))
    );
}

// Exceptionally stupid solution, but impressive that I got it to work

fn part_1<const N: usize>(input: &str) -> usize {
    let (inst, _blocks, mut start) = do_the_lines::<N>(input);
    let blocks = neighbours1(_blocks);
    move_walker(inst, &blocks, &mut start)
}

fn part_2<const N: usize>(input: &str) -> usize {
    let (inst, _blocks, mut start) = do_the_lines::<N>(input);
    let blocks = neighbours2(_blocks);
    move_walker(inst, &blocks, &mut start)
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().to_string()
}

#[derive(Clone, Eq, PartialEq)]
struct Block<const N: usize> {
    y: usize,
    x: usize,
    content: [[char; N]; N],
    size: usize,
    neighbours: [((usize, usize), usize); 4],
}

#[derive(Clone, Eq, PartialEq)]
struct Walker {
    block: (usize, usize),
    position: (usize, usize),
    facing: usize,
}

impl Walker {
    fn sum(&self, size: usize) -> usize {
        (self.block.0 * size + self.position.0 + 1) * 1000
            + (self.block.1 * size + self.position.1 + 1) * 4
            + self.facing
    }
}

impl Walker {
    fn turn(&mut self, turns: i32) {
        self.facing = modulo(self.facing as i32 + turns, 4)
    }
}

fn modulo(a: i32, b: i32) -> usize {
    //conversion to i32 must occur at source since -1 will overflow usize
    (((a % b) + b) % b) as usize
}

fn do_the_lines<const N: usize>(
    input: &str,
) -> (&str, HashMap<(usize, usize), [[char; N]; N]>, Walker) {
    let (map, instructions) = input.split_once("\n\n").unwrap();
    let mut row: usize = 0;
    let mut blocks: HashMap<(usize, usize), [[char; N]; N]> = HashMap::new();
    let mut start: Option<Walker> = None;

    //parse map into blocks, separates out instrutions, finds start position
    for line in map.lines() {
        let mut col: usize = 0;
        //int division - rounds down
        let y = row / N as usize;
        for c in line.chars() {
            if c == ' ' || c == '.' {
                if start == None && c == '.' {
                    let x = col / N as usize;
                    start = Some(Walker {
                        block: (y, x),
                        position: (modulo(row as i32, N as i32), modulo(col as i32, N as i32)),
                        facing: 0,
                    });
                }
                col += 1;
                continue;
            }
            let x = col / N as usize;
            if !blocks.contains_key(&(y, x)) {
                blocks.insert((y, x), [['.'; N]; N]);
            }
            blocks.get_mut(&(y, x)).unwrap()[row - y * N][col - x * N] = c;
            col += 1;
            // already know c == '#'
        }
        row += 1;
    }
    if true {
        for row in 0..(N * 4) {
            for col in 0..(N * 4) {
                if blocks.contains_key(&(row / N, col / N)) {
                    print!("{}", blocks[&(row / N, col / N)][row % N][col % N]);
                } else {
                    print!("{}", ' ');
                }
            }
            println!();
        }
    }

    (instructions.trim(), blocks, start.unwrap())
}

fn move_walker<const N: usize>(
    inst: &str,
    blocks: &HashMap<(usize, usize), Block<N>>,
    walker: &mut Walker,
) -> usize {
    //iterates instructions and moves the walker
    // 0 = E, 1 = S, 2 = W, 3 = N
    let mut distance: Vec<char> = Vec::new();
    let mut instructions = inst.chars().peekable();
    while let Some(c) = instructions.next() {
        if c.is_numeric() {
            //make sure to collect whole number, then walk
            distance.push(c);
            if instructions.peek() == None || !instructions.peek().unwrap().is_numeric() {
                let num_dist = distance
                    .iter()
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap();
                //send off to walk
                walk(blocks, walker, num_dist);
                distance.clear();
            }
            //or turn
        } else if c == 'L' {
            walker.turn(-1);
        } else if c == 'R' {
            walker.turn(1);
        } else {
            panic!("Unknown instruction");
        }
    }
    //once all instructions complete, calculate answer
    walker.sum(N)
}

fn walk<const N: usize>(
    blocks: &HashMap<(usize, usize), Block<N>>,
    walker: &mut Walker,
    mut distance: usize,
) {
    //makes one step at a time, if it hits a wall stops, if it hits an edge, moves to next block
    let directions = HashMap::from([(0, (0, 1)), (1, (1, 0)), (2, (0, -1)), (3, (-1, 0))]);
    let mut new_position: (i32, i32) = (walker.position.0 as i32, walker.position.1 as i32);
    let mut new_block = walker.block;
    let mut new_turn = walker.facing;
    while distance > 0 {
        new_position.0 += directions[&walker.facing].0 as i32;
        new_position.1 += directions[&walker.facing].1 as i32;
        //moving blocks east
        if new_position.1 >= N as i32 {
            new_block = blocks[&walker.block].neighbours[0].0;
            new_turn = blocks[&walker.block].neighbours[0].1;
            // coordinates on arrival change depending on orientation of new block in part 2
            if angular_dist(new_turn, walker.facing) == 1 {
                new_position = rotate::<N>(walker.facing as i32, new_position, new_turn as i32)
            } else if (new_turn as i32 - walker.facing as i32).abs() == 2 {
                new_position.0 = modulo(N as i32 - new_position.0 as i32 - 1, N as i32) as i32;
                new_position.1 = N as i32 - 1;
            } else {
                new_position.1 = 0;
            }
        }
        //moving blocks south
        if new_position.0 >= N as i32 {
            new_block = blocks[&walker.block].neighbours[1].0;
            new_turn = blocks[&walker.block].neighbours[1].1;
            if angular_dist(new_turn, walker.facing) == 1 {
                new_position = rotate::<N>(walker.facing as i32, new_position, new_turn as i32);
            } else if (new_turn as i32 - walker.facing as i32).abs() == 2 {
                new_position.1 = modulo(N as i32 - new_position.1 as i32 - 1, N as i32) as i32;
                new_position.0 = N as i32 - 1;
            } else {
                new_position.0 = 0;
            }
        }
        //moving blocks west
        if new_position.1 < 0 {
            new_block = blocks[&walker.block].neighbours[2].0;
            new_turn = blocks[&walker.block].neighbours[2].1;
            if angular_dist(new_turn, walker.facing) == 1 {
                new_position = rotate::<N>(walker.facing as i32, new_position, new_turn as i32);
            } else if (new_turn as i32 - walker.facing as i32).abs() == 2 {
                new_position.0 = modulo(N as i32 - new_position.0 as i32 - 1, N as i32) as i32;
                new_position.1 = 0;
            } else {
                new_position.1 = (N as i32) - 1;
            }
        }
        //moving blocks north
        if new_position.0 < 0 {
            new_block = blocks[&walker.block].neighbours[3].0;
            new_turn = blocks[&walker.block].neighbours[3].1;
            if angular_dist(new_turn, walker.facing) == 1 {
                new_position = rotate::<N>(walker.facing as i32, new_position, new_turn as i32);
                println!("new_position: {:?}", new_position);
            } else if (new_turn as i32 - walker.facing as i32).abs() == 2 {
                new_position.1 = modulo(N as i32 - new_position.1 as i32 - 1, N as i32) as i32;
                new_position.0 = 0;
            } else {
                new_position.0 = (N as i32) - 1;
            }
        }
        if blocks[&new_block].content[new_position.0 as usize][new_position.1 as usize] == '#' {
            println!("found #");
            break;
        } else {
            walker.block = new_block;
            walker.position = (new_position.0 as usize, new_position.1 as usize);
            walker.facing = new_turn;
        }
        println! {"block {:?} positionl {:?} face {}", walker.block , walker.position ,walker.facing};
        distance -= 1;
    }
}
fn angular_dist(a: usize, b: usize) -> i32 {
    match a {
        0 => match b {
            0 => return 0,
            1 => return 1,
            2 => return 2,
            3 => return 1,
            _ => panic!("Unknown rotation"),
        },
        1 => match b {
            0 => return 1,
            1 => return 0,
            2 => return 1,
            3 => return 2,
            _ => panic!("Unknown rotation"),
        },
        2 => match b {
            0 => return 2,
            1 => return 1,
            2 => return 0,
            3 => return 1,
            _ => panic!("Unknown rotation"),
        },
        3 => match b {
            0 => return 1,
            1 => return 2,
            2 => return 1,
            3 => return 0,
            _ => panic!("Unknown rotation"),
        },
        _ => panic!("Unknown rotation"),
    }
}

fn rotate<const N: usize>(face: i32, mut position: (i32, i32), rotation: i32) -> (i32, i32) {
    let n = N as i32;
    match face {
        0 => match rotation {
            1 => {
                position = (0, modulo(n - position.0 - 1, n) as i32);
            }
            3 => {
                position = (n - 1, position.0);
            }
            _ => panic!("Unknown rotation"),
        },
        1 => match rotation {
            2 => {
                position = (position.1, n - 1);
            }
            0 => {
                position = (modulo(n - position.1 - 1, n) as i32, 0);
            }
            _ => panic!("Unknown rotation"),
        },
        2 => match rotation {
            3 => {
                position = (n - 1, modulo(n - position.0 - 1, n) as i32);
            }
            1 => {
                position = (0, position.0);
            }
            _ => panic!("Unknown rotation"),
        },
        3 => match rotation {
            0 => {
                position = (position.1, 0);
            }
            2 => {
                position = (modulo(n - position.1, n - 1) as i32, n - 1);
            }
            _ => panic!("Unknown rotation"),
        },
        _ => panic!("Unknown rotation"),
    }
    println!("face {} position: {:?}", face, position);
    position
}

fn neighbours1<const N: usize>(
    blocks: HashMap<(usize, usize), [[char; N]; N]>,
) -> HashMap<(usize, usize), Block<N>> {
    //finds the neighbour blocks in all directions for all blocks
    let mut new_blocks = HashMap::new();
    for block in blocks.keys() {
        let mut neighbs = Vec::new();
        let steps = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for step in steps.iter() {
            let mut next = (block.0, block.1);
            loop {
                //loop over 4 even if it is 3x4 or 4x3 for simplicity
                next = (
                    modulo(next.0 as i32 + step.0, 4),
                    modulo(next.1 as i32 + step.1, 4),
                );
                if blocks.contains_key(&&next) {
                    let facing;
                    match step {
                        (0, 1) => facing = 0,
                        (1, 0) => facing = 1,
                        (0, -1) => facing = 2,
                        (-1, 0) => facing = 3,
                        _ => facing = 6,
                    }
                    neighbs.push(((next.0, next.1), facing));
                    break;
                }
            }
        }
        let block = Block {
            y: block.0,
            x: block.1,
            content: blocks[&block],
            size: N,
            neighbours: [neighbs[0], neighbs[1], neighbs[2], neighbs[3]],
        };
        new_blocks.insert((block.y, block.x), block);
    }
    new_blocks
}

fn neighbours2<const N: usize>(
    _blocks: HashMap<(usize, usize), [[char; N]; N]>,
) -> HashMap<(usize, usize), Block<N>> {
    //Same but for the cube, and oh boy...
    let blocks = _blocks
        .keys()
        .map(|x| x.clone())
        .collect::<HashSet<(usize, usize)>>();

    let mut new_blocks = HashMap::new();
    for block in _blocks.keys() {
        new_blocks.insert(
            (block.0, block.1),
            Block {
                y: block.0,
                x: block.1,
                content: _blocks[&block],
                size: N,
                neighbours: [((5, 5), 5); 4],
            },
        );
    }
    let mut not_found = Vec::new();

    println!("start_blocks: {:?}", blocks);
    let mut stage = 0;
    while stage < 3 {
        let mut neighbours = Vec::new();
        for current_block in blocks.iter() {
            //check all direct contacts and set them, blocking them from next stages
            //check att inner corners, diagonal away from edge
            //check all 2 to a side one up/down
            //find all missing, split on long side, sort by short side, assign

            //Straight
            let directions_1 = [(0, [(0, 1)]), (1, [(1, 0)]), (2, [(0, -1)]), (3, [(-1, 0)])]; //E, S, W, N

            //Diagonal
            let directions_2 = [
                (0, [(1, 1), (-1, 1)]),
                (1, [(1, 1), (1, -1)]),
                (2, [(1, -1), (-1, -1)]),
                (3, [(-1, 1), (-1, -1)]),
            ];

            //2:1 with variations
            let directions_3 = [
                (0, [(2, 1), (-2, 1), (2, -1), (-2, -1)]),
                (1, [(1, 2), (1, -2), (-1, 2), (-1, -2)]),
                (2, [(2, -1), (-2, -1), (2, 1), (-2, 1)]),
                (3, [(-1, 2), (-1, -2), (1, 2), (1, -2)]),
            ];

            if stage == 0 {
                for (face, checks) in directions_1.iter() {
                    for checks in checks.iter() {
                        if current_block.0 as i32 + checks.0 >= 0
                            && current_block.1 as i32 + checks.1 >= 0
                        {
                            let destination = (
                                (current_block.0 as i32 + checks.0) as usize,
                                (current_block.1 as i32 + checks.1) as usize,
                            );
                            let face_now = *face;
                            let face_arrive = *face;
                            if blocks.contains(&destination) {
                                neighbours.push((
                                    *current_block,
                                    destination,
                                    face_now,
                                    face_arrive,
                                ));
                            }
                        }
                    }
                }
            } else if stage == 1 {
                for (face, checks) in directions_2.iter() {
                    for checks in checks.iter() {
                        if current_block.0 as i32 + checks.0 >= 0
                            && current_block.1 as i32 + checks.1 >= 0
                        {
                            let destination = (
                                (current_block.0 as i32 + checks.0) as usize,
                                (current_block.1 as i32 + checks.1) as usize,
                            );
                            let face_now = *face;
                            let mut face_arrive = *face;
                            match face_now {
                                0 => {
                                    face_arrive = modulo(face_arrive + checks.0, 4) as i32;
                                }
                                1 => {
                                    face_arrive = modulo(face_arrive - checks.1, 4) as i32;
                                }
                                2 => {
                                    face_arrive = modulo(face_arrive - checks.0, 4) as i32;
                                }
                                3 => {
                                    face_arrive = modulo(face_arrive + checks.1, 4) as i32;
                                }
                                _ => panic!("Unknown rotation"),
                            }
                            if blocks.contains(&destination)
                                && new_blocks[&current_block].neighbours[face_now as usize].0
                                    == (5, 5)
                                && new_blocks[&destination].neighbours
                                    [modulo(face_arrive + 2, 4) as usize]
                                    .0
                                    == (5, 5)
                            {
                                neighbours.push((
                                    *current_block,
                                    destination,
                                    face_now,
                                    face_arrive,
                                ));
                            }
                        }
                    }
                }
            } else if stage == 2 {
                for (face, checks) in directions_3.iter() {
                    for checks in checks.iter() {
                        if current_block.0 as i32 + checks.0 >= 0
                            && current_block.1 as i32 + checks.1 >= 0
                        {
                            let destination = (
                                (current_block.0 as i32 + checks.0) as usize,
                                (current_block.1 as i32 + checks.1) as usize,
                            );
                            let face_now = *face;
                            let face_arrive = modulo(*face + 2, 4) as i32;
                            if blocks.contains(&destination)
                                && new_blocks[&current_block].neighbours[face_now as usize].0
                                    == (5, 5)
                                && new_blocks[&destination].neighbours
                                    [modulo(face_arrive + 2, 4) as usize]
                                    .0
                                    == (5, 5)
                            {
                                neighbours.push((
                                    *current_block,
                                    destination,
                                    face_now,
                                    face_arrive,
                                ));
                            }
                        }
                    }
                }
            }
        }
        for n in neighbours.iter() {
            //add
            add_neighbour(&mut new_blocks, n.0, n.1, n.2, n.3);
        }

        stage += 1;
    }
    for (k, block) in new_blocks.iter() {
        for face in 0..4 {
            if block.neighbours[face].0 == (5, 5) {
                not_found.push((*k, face));
            }
        }
    }
    let mut max = (0, 0);
    for v in not_found.iter() {
        if v.0 .0 > max.0 {
            max.0 = v.0 .0;
        }
        if v.0 .1 > max.1 {
            max.1 = v.0 .1;
        }
    }
    let mut al = Vec::new();
    let mut bl = Vec::new();
    if max.0 > max.1 {
        for v in not_found.iter() {
            let sorter;
            // first we sort on block, then rotation. We do this by giving bloch weight 10
            match v.1 {
                0 => sorter = 3 + v.0 .1 * 10,
                1 => sorter = 2 + v.0 .1 * 10,
                2 => sorter = 1 + v.0 .1 * 10,
                3 => sorter = 2 + v.0 .1 * 10,
                _ => panic!("Unknown rotation"),
            }
            if v.0 .0 <= 1 {
                al.push((*v, sorter));
            } else {
                bl.push((*v, sorter));
            }
        }
    } else {
        for v in not_found.iter() {
            let sorter;
            match v.1 {
                1 => sorter = 3 + v.0 .0 * 10,
                2 => sorter = 2 + v.0 .0 * 10,
                3 => sorter = 1 + v.0 .0 * 10,
                0 => sorter = 2 + v.0 .0 * 10,
                _ => panic!("Unknown rotation"),
            }
            if v.0 .1 <= 1 {
                al.push((*v, sorter));
            } else {
                bl.push((*v, sorter));
            }
        }
    }
    al.sort_by_key(|a| a.1);
    bl.sort_by_key(|a| a.1);
    let mut i = al.iter();
    let mut j = bl.iter();

    while let Some(a) = i.next() {
        // block, rotation(direction of face not found)
        let b = j.next().unwrap();
        let from = a.0 .0;
        let to = b.0 .0;
        let face_from = a.0 .1 as i32;
        let face_to = b.0 .1 as i32;
        add_neighbour(
            &mut new_blocks,
            from,
            to,
            face_from,
            modulo(face_to + 2, 4) as i32,
        );
        add_neighbour(
            &mut new_blocks,
            to,
            from,
            face_to,
            modulo(face_from + 2, 4) as i32,
        );
    }
    new_blocks
}

fn add_neighbour<const N: usize>(
    new_blocks: &mut HashMap<(usize, usize), Block<N>>,
    current_block: (usize, usize),
    target_block: (usize, usize),
    current_face: i32,
    target_face: i32,
) {
    if new_blocks[&current_block].neighbours[current_face as usize].0 == (5, 5) {
        println!(
            "{:?} face {} links {:?} facing {}",
            current_block, current_face, target_block, target_face
        );
        new_blocks.get_mut(&current_block).unwrap().neighbours[current_face as usize] =
            (target_block, target_face as usize);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1::<4>(&read_file("input copy.txt")), 6032);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1::<50>(&read_file("input.txt")), 58248);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2::<4>(&read_file("input copy.txt")), 5031);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2::<50>(&read_file("input.txt")), 179091);
    }
}
