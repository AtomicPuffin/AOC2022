use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
    println!("Answer to Part 2 test: {}", part_2(&read_file("input copy.txt")));
    println!("Answer to Part 2: {}", part_2(&read_file("input.txt")));
}

fn part_1 (input: &str) -> usize {
    let trees: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars()
            .map(|c| c.to_string()
                .parse::<i32>()
                .unwrap())
            .collect())
        .collect();

    let mut answer = trees[0].len() *2; //add first last row
    let max_row = trees.len();
    let max_col = trees[0].len();
    for y in 1..max_row-1 {
        answer += 2; //add first last element of row
        for x in 1..max_col-1 { // for each coordinate
            let tree = trees[y][x];
            let tallest_u = (0..y).all(|u| tree > trees[u][x]);
            let tallest_d = (y+1..max_row).all(|d| tree > trees[d][x]);
            let tallest_l = (0..x).all(|l| tree > trees[y][l]);
            let tallest_r = (x+1..max_col).all(|r| tree > trees[y][r]);
            if [tallest_u, tallest_d, tallest_l, tallest_r].iter().any(|&b| b) {
                answer +=1;
            }
        }
    }
    answer
}


fn part_2 (input: &str) -> usize {
    let mut trees = Vec::new();
    for r in input.lines() {
        let mut row = Vec::new();
        for t in r.chars() {
            let x = t.to_string().parse::<i32>().unwrap();
            row.push(x)
        }
        trees.push(row);
    }
    let mut answer = 0;
    let max_row = trees.len();
    let max_col = trees[0].len();

    for y in 0..max_row {
        for x in 0..max_col {
            let mut view_u = 0;
            let mut view_d = 0;
            let mut view_l = 0;
            let mut view_r = 0;
    
            for u in (0..y).rev() {
                view_u += 1; //inclusive last element
                if trees[y][x] <= trees[u][x] {
                    break;
                }
            }
            for d in y+1..max_row {
                view_d += 1;
                if trees[y][x] <= trees[d][x] {
                    break;
                }
            }
            for l in (0..x).rev() {
                view_l += 1;
                if trees[y][x] <= trees[y][l] {
                    break;
                }
            }
            for r in x+1..max_col {
                view_r += 1;
                if trees[y][x] <= trees[y][r] {
                    break;
                }
            }
            let new = view_u * view_d * view_r * view_l;
            if answer < new {
                answer = new;
            }
        }
    }
    answer
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
        assert_eq!(part_1(&read_file("input copy.txt")), 21);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 1854);
    }

    #[test]
    fn test_p2_ex() {
        assert_eq!(part_2(&read_file("input copy.txt")), 8);
    }

    #[test]
    fn test_p2() {
        assert_eq!(part_2(&read_file("input.txt")), 527340);
    }
}