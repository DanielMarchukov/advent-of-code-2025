const INPUT: &str = include_str!("input.txt");

pub fn solve() {
    let mut res: u64 = 0;
    INPUT
        .split(",")
        .map(|s| s.split('-'))
        .map(|mut parts| {
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .for_each(|(start, end)| {
            for n in start..=end {
                let digits = if n == 0 { 1 } else { n.ilog10() + 1};
                for d in 1..=digits / 2 {
                    if digits % d != 0 {
                        continue;
                    }
                    let divisor = 10_u64.pow(d);
                    let prev = n % divisor;
                    let mut num = n;
                    while num != 0 {
                        let curr = num % divisor;
                        if curr != prev {
                            break;
                        }
                        num /= divisor;
                    }
                    if num == 0 {
                        res += n;
                        break;
                    }
                }
            }
        });
    println!("aoc02 = {}", res);
}
