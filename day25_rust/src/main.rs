use std::fs;

fn main() {

    println!("Answer to Part 1 test: {}", part_1(&read_file("input copy.txt")));
    println!("Answer to Part 1: {}", part_1(&read_file("input.txt")));
}

fn part_1 (input: &str) -> String {
    let mut sum = "0".to_string();
    let mut sum2 = 0;
    for l in input.lines() {
        sum = add_snafu(&sum, l);
        sum2 += from_snafu(l);
    }
    println!("{}", sum2);
    sum
}


fn add_snafu(snafu_a: &str, snafu_b: &str) -> String {
    let a: Vec<i128> = snafu_a.chars().map(|x| translate_todigit(x)).collect();
    let b: Vec<i128> = snafu_b.chars().map(|x| translate_todigit(x)).collect();
    let mut carry = 0;
    let mut sum: Vec<i128> = Vec::new();
    let (mut long, mut short) = if a.len() > b.len() {(a, b)} else {(b,a)};
    let mut ans;
    while let Some(n1) = long.pop() {
        if short.len() > 0 {
            let n2 = short.pop().unwrap();
            (ans, carry) = add_dig(n1, n2, carry);
            sum.insert(0, ans);
        } else if carry != 0 {
            (ans, carry) = add_dig(n1, 0, carry);
            sum.insert(0, ans);
        } else {
            sum.insert(0, n1);
        }
    }
    if carry != 0 {
        (ans, _) = add_dig(0, 0, carry);
        sum.insert(0, ans);
    }
    let mut out = "".to_string();
    for i in sum {
        out.push(translate_fromdigit(i));
    }
    out
}

fn add_dig(a: i128, b:i128, carry:i128) -> (i128,i128) {
    let ans = a + b + carry;
    if ans > 2 {
        return (ans - 5, 1)
    } else if ans < -2 {
        return (ans + 5, -1)
    } else {
        return (ans, 0)
    }
}

fn translate_todigit(a: char) -> i128 {
    match a {
        '2' => return 2,
        '1' => return 1,
        '0' => return 0,
        '-' => return -1,
        '=' => return -2,
        _ => panic!("Not good snafu"),
    }
}
fn translate_fromdigit(a: i128) -> char {
    match a {
        2 => return '2',
        1 => return '1',
        0 => return '0',
        -1 => return '-',
        -2 => return '=',
        _ => panic!("Not good digit"),
    }
}

fn from_snafu(input: &str) -> i128 {
    let mut position = 0;
    let mut ans = 0;
    let base: i128 = 5;
    let mut num: Vec<char> = input.chars().collect();
    while let Some(n) = num.pop() {
        match n {
            '2' => ans += 2 * base.pow(position),
            '1' => ans += 1 * base.pow(position),
            '0' => ans += 0 * base.pow(position),
            '-' => ans += -1 * base.pow(position),
            '=' => ans += -2 * base.pow(position),
            _ => panic!("Not good digit"),
        }
        position += 1;
    }
    ans
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
        assert_eq!(part_1(&read_file("input copy.txt")), 2=-1=0);
    }

    #[test]
    fn test_p1() {
        assert_eq!(part_1(&read_file("input.txt")), 20-1-11==0-=0112-222);
    }
}