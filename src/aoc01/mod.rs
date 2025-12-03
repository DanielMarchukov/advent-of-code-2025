const INPUT: &str = include_str!("input.txt");

pub fn solve() {
    let mut res: i16 = 0;
    let mut curr: i16 = 50;
    for line in INPUT.lines() {
        let sign = if line.starts_with('L') { -1 } else { 1 };
        let value: i16 = line[1..].parse().unwrap();
        let old_curr = curr;
        curr += sign * value;

        let count = if curr >= old_curr {
            curr.div_euclid(100)
        } else {
            (old_curr - 1).div_euclid(100) - (curr - 1).div_euclid(100)
        };
        res += count;
        curr = curr.rem_euclid(100);
    }
    println!("aoc01 = {}", res);
}