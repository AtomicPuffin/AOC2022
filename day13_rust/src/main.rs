use std::fs;
use std::iter::Peekable;
use std::cmp::Ordering;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> i32 {
    let mut lines = input.lines();
    let mut answer = 0;
    let mut pindex = 0;
    while let Some(line) = lines.next() {
        pindex += 1;
        let p1 = parse_int(&mut line.chars().peekable());
        let p2 = parse_int(&mut lines.next().unwrap().chars().peekable());
        if p1 < p2 {
            answer += pindex;
        }
        lines.next(); // skip divider
    }
    answer
}

fn part_2 (input: &str) -> i32 {
    let dp1 = parse_int(&mut "[[2]]".chars().peekable());
    let dp2 = parse_int(&mut "[[6]]".chars().peekable());
    let mut i1 = 1;
    let mut i2 = 2; 
    for l in input.lines() {
        if l != "" {
            let packet = parse_int(&mut l.chars().peekable());
            if dp1 > packet {
                i1 += 1;
            }
            if dp2 > packet {
                i2 += 1;
            }
        }
    }
    i1 * i2
}

#[derive(PartialEq, Eq, Clone)]
enum Tree {
    Leaf(i32),
    Branches(Vec<Tree>),
}

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tree {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use Tree::*;
        match self {
            Leaf(ls) => match other {
                Leaf(lo) => ls.cmp(lo),
                Branches(_) => Branches(vec![Leaf(*ls)]).cmp(other),
            },
            Branches(bs) => match other {
                Leaf(lo) => self.cmp(&Branches(vec![Leaf(*lo)])),
                Branches(bo) => {
                    let len = bs.len().min(bo.len());
                    let mut result = None;
                    for i in 0..len {
                        result = Some(bs[i].cmp(&bo[i]));
                        if result == Some(Ordering::Equal) {
                            result = None; // equal is not a result yet
                        } else if result == Some(Ordering::Greater) {
                            result = Some(Ordering::Greater);
                            break;
                        } else if result == Some(Ordering::Less) {
                            result = Some(Ordering::Less);
                            break;
                        }
                    } 
                    result.unwrap_or_else(|| bs.len().cmp(&bo.len()))
                },
            },
        }
    }
}

fn parse_list<T: Iterator<Item = char>>(input: &mut Peekable<T>) -> Tree  {
    let mut list: Vec<Tree> = Vec::new();
    loop {
        if let Some(_c) = input.next_if(|&c| c == ',') {
            //throw it away
        } else if let Some(_c) = input.next_if(|&c| c == ']') {
            break; //end of list
        }
        list.push(parse_int(input)); //keep building
    }
    Tree::Branches(list)
}

fn parse_int<T: Iterator<Item = char>>(input: &mut Peekable<T>) -> Tree {
    let c = input.next().unwrap();
    if c == '[' {
        parse_list(input)
    } else {
        let mut temp: String = c.into();
        while let Some(temp_next) = input.next_if(|&c| c.is_ascii_digit()) {
            temp.push(temp_next);
        }
        Tree::Leaf(temp.parse::<i32>().unwrap())
    }
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("input copy.txt")), 13);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 5760);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 140);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 26670);
    }
}