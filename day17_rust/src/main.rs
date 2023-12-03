#![allow(unused)]
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

fn part_1(input: &str) -> usize {
    let instructions: Vec<char> = input.chars().collect();
    let mut dir_index = 0;
    let mut room = Vec::new();
    room.push(0b11111111); // , push floor, index = position, len() = height
    let mut rock = 0;

    while rock < 2022 {
        let mut shape = create_next(room.len(), rock);
        loop {
            let (out, _possible) = move_shape(&shape, &room, instructions[dir_index]);
            dir_index = (dir_index + 1) % instructions.len();
            shape = out; // motion dealt with inside, ignore possible since hitting walls is irrelevant
            let (out, possible) = move_shape(&shape, &room, 'd');
            if !possible {
                //could not drop, add to room
                for (h, s) in out {
                    if h < room.len() {
                        //same level as current room
                        room[h] = room[h] | s; // bitwise or combines row
                    } else {
                        //new level
                        room.push(s);
                    }
                }
                break; //done with this shape
            } else {
                shape = out;
            }
        }
        rock += 1;
    }
    //print_room(&room);
    room.len() - 1 //-1 since we use one space for the floor
}

fn part_2(input: &str) -> usize {
    let instructions: Vec<char> = input.chars().collect();
    let mut dir_index = 0;
    let mut room = Vec::new();
    room.push(0b11111111); // index = position, le = height
    let mut rock = 0;
    let mut pattern = Vec::new();
    let mut units = 0;
    let mut rocks_to = 1000000000000;
    let mut found_pattern = false;
    while rock < rocks_to {
        let mut cycles_new = 0;
        let shape_number = rock % 5;
        let mut shape = create_next(room.len(), rock);
        loop {
            let (out, _possible) = move_shape(&shape, &room, instructions[dir_index]);
            // detect repeating pattern when instructions loop, not every loop, but neventually
            if dir_index + 1 == instructions.len() && !found_pattern {
                // save loop snapshot
                pattern.push((cycles_new, shape_number, room.len(), rock));
                if pattern.len() >= 2 {
                    let mut pt = pattern.clone();
                    let (c, s, l, r) = pt.pop().unwrap();
                    for p in pt {
                        //se if we find repetition
                        if c == p.0 && s == p.1 {
                            //Cyclic repetitions will be added arithmetically, head and tail will be room length
                            let cycle_rocks = r - p.3;
                            let cycle_units = l - p.2;
                            let cycles = (1000000000000 - p.3) / cycle_rocks;
                            // -1 cycle so we can just use room.len for head and tail and the first cycle
                            units = (cycles - 1) * cycle_units;
                            //remove head and cycles, execute tail
                            rocks_to = 1000000000000 - (cycles - 1) * cycle_rocks;
                            found_pattern = true;
                        }
                    }
                }
            }
            cycles_new += 1;
            dir_index = (dir_index + 1) % instructions.len();
            shape = out; // motion dealt with inside, ignore possible since hitting walls is irrelevant
            let (out, possible) = move_shape(&shape, &room, 'd');
            if !possible {
                //could not drop, add to room
                for (h, s) in out {
                    if h < room.len() {
                        room[h] = room[h] | s;
                    } else {
                        room.push(s);
                    }
                }
                break;
            } else {
                shape = out;
            }
        }
        rock += 1;
    }
    //print_room(&room);
    units + room.len() - 1
}

fn print_room(room: &Vec<u8>) {
    let mut rcopy = room.clone();
    rcopy.reverse();
    for r in rcopy {
        println!("{:08b}", r);
    }
    println!("#############");
}

fn create_next(mut height: usize, rock: usize) -> Vec<(usize, u8)> {
    let mut out: Vec<(usize, u8)> = Vec::new();
    let shapes = {
        vec![
            vec![0b00011110],
            vec![0b00001000, 0b00011100, 0b00001000],
            vec![0b00000100, 0b00000100, 0b00011100],
            vec![0b00010000, 0b00010000, 0b00010000, 0b0010000],
            vec![0b00011000, 0b00011000],
        ]
    };
    let mut shape = shapes[(rock % 5) as usize].clone();
    height += 3;
    shape.reverse();
    for s in shape {
        out.push((height, s));
        height += 1;
    }
    out
}

fn move_shape(
    shape: &Vec<(usize, u8)>,
    room: &Vec<u8>,
    direction: char,
) -> (Vec<(usize, u8)>, bool) {
    let mut out: Vec<(usize, u8)> = Vec::new();
    let mut possible = true;
    for (h, s) in shape {
        if direction == '>' {
            possible = possible && (s & 0b0000_0001 == 0); //test wall (see if edge is 1)
            if room.len() > *h {
                possible = possible && ((s >> 1) & room[*h] == 0); //see if we run into old piece
            }
        } else if direction == '<' {
            possible = possible && (s & 0b0100_0000 == 0); //test wall
            if room.len() > *h {
                possible = possible && ((s << 1) & room[*h] == 0); //see if we run into old piece
            }
        } else {
            if room.len() > (*h - 1) {
                possible = possible && s & room[*h - 1] == 0; //test collission down
            }
        }
    }
    if possible {
        for (h, s) in shape {
            if direction == '>' {
                out.push((*h, *s >> 1));
            } else if direction == '<' {
                out.push((*h, *s << 1));
            } else {
                out.push((*h - 1, *s));
            }
        }
    } else {
        out = shape.to_vec();
    }
    (out, possible)
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
        assert_eq!(part_1(&read_file("input copy.txt")), 3068);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 3186);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 1514285714288);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 1566376811584);
    }
}
