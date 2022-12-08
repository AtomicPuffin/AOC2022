use std::fs;

fn main() {

    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> i32 {
    let mut counter = 1;
    let mut sop = (' ',' ',' ',' ');
    for c in input.chars() {
        sop.0 = sop.1;
        sop.1 = sop.2;
        sop.2 = sop.3;
        sop.3 = c;
        if sop.0 != sop.1
            && sop.0 != sop.2
            && sop.0 != sop.3
            && sop.1 != sop.2
            && sop.1 != sop.3
            && sop.2 != sop.3 {
                if counter > 4 {
                    return counter
                }
            }
        counter += 1;
    }
    panic!("Reached eof without match");
}

fn part_2 (input: &str) -> i32 {
    let mut counter = 1;
    let mut sop = Vec::new();
    sop.push('\n');
    for c in input.chars() {
        if counter < 14 {
            sop.push(c);
        } else {
            sop.remove(0);
            sop.push(c);
            let mut trial = true;
            let mut sop_temp = Vec::new();
            for s in &sop {
                let _ = &sop_temp.push(s.clone());
            }
            'break_here: for _ in 0..sop_temp.len() {
                let x = &sop_temp.pop().unwrap();
                for i in &sop_temp {
                    if x == i {
                        trial = false;
                        break 'break_here;
                    }
                }
            }
            if trial {
                return counter
            }
        }
        counter += 1;
    }
    panic!("Reached eof without match");
}

fn read_file(file: &str) -> String {
    fs::read_to_string(file).unwrap().trim().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_p1_ex() {
        assert_eq!(part_1(&read_file("input copy.txt")), 6);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1723);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 23);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 3708);
    }
}