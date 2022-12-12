use std::collections::HashMap;
use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> i32 {
    let (unvisited, start, end) = parse_input(input);
    shortest_path(unvisited, start, end, false)
}

fn part_2 (input: &str) -> i32 {
    let (unvisited, start, end) = parse_input(input);
    shortest_path(unvisited, end, start, true)
}

fn shortest_path(mut unvisited: HashMap<(i32,i32), (i32, i32)> , start: (i32, i32),end: (i32,i32), part_2: bool) -> i32 {
    let mut pos = start;
    unvisited.get_mut(&pos).unwrap().1 = 0;
    loop {
        for n in find_next(&unvisited, pos, part_2) {
            if part_2 {
                if unvisited[&n].0 == 'a' as i32 {
                    return unvisited[&pos].1 +1;
                }
            } else {
                if n == end {
                    return unvisited[&pos].1 +1;
                }
            }
            if unvisited[&(n)].1 > unvisited[&(pos)].1 + 1 {
                unvisited.get_mut(&n).unwrap().1 = unvisited[&(pos)].1 + 1;
            }
        }
        unvisited.remove(&pos);
        let mut next = ((-1,-1), i32::MAX);
        for (k,v) in &unvisited {
            if v.1 < next.1 {
                next = (*k, v.1);
            }
        }
        pos = next.0;
        if pos.0 == -1 {
            return i32::MAX;
        }
    }

}

fn find_next(unvisited: &HashMap<(i32,i32), (i32, i32)>, pos: (i32, i32), part_2: bool) -> Vec<(i32, i32)> {
    let mut next = Vec::new();
    for i in [(-1,0),(1,0),(0,-1),(0,1)] {
        let mut temp = (-10,-10);
        temp.0 = pos.0 + i.0;
        temp.1 = pos.1 + i.1;
        if temp.0 >= 0 && temp.1 >= 0  && unvisited.contains_key(&temp) { // avoid leaving grid
            // check if ok step, all distances = 1
            if part_2 {
                if unvisited[&temp].0 >= unvisited[&pos].0 -1 {
                    next.push(temp);
                }               
            } else {
                if unvisited[&temp].0 <= unvisited[&pos].0 +1 {
                    next.push(temp);
                }
            }  
        }
    }
    next
}

fn parse_input(input: &str) -> (HashMap<(i32,i32), (i32, i32)> , (i32, i32), (i32,i32)) {
    let mut unvisited: HashMap<(i32,i32), (i32, i32)> = HashMap::new();
    let mut y = 0;
    let mut x = 0;
    let mut start = (0,0);
    let mut end = (0,0);
    for l in input.lines() {
        for c in l.chars() {
            if c == 'S' {
                unvisited.insert((y,x),('a' as i32, i32::MAX));
                start = (y, x);
            } else if c == 'E' {
                unvisited.insert((y,x),('z' as i32, i32::MAX));
                end = (y, x);
            } else {
                unvisited.insert((y,x),(c as i32, i32::MAX));
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
    (unvisited, start, end)
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[ignore]
    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("input copy.txt")), 31);
    }

    #[ignore]
    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 447);
    }

    #[ignore]
    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 29);
    }

    #[ignore]
    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 446);
    }
}