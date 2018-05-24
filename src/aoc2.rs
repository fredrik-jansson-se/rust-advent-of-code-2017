use std::fs;

pub fn run() {
    let input = fs::read_to_string("day2.txt").unwrap();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn parse_line(input: &str) -> Vec<u32> {
    let mut v : Vec<u32> = input.split_whitespace().
        map(|s| s.parse().unwrap()).collect();
    v.sort();
    v
}   

pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input.split('\n').filter(|s| *s!="").map(parse_line).collect()
}

fn min_max(input: &[u32]) -> (u32, u32) {
    let mut min = input[0];
    let mut max = input[0];
    for i in &input[1..] {
        if *i < min {
            min = *i;
        }
        else if *i > max {
            max = *i;
        }
    }
    (min, max)
}

fn part1(input: &str) -> u32 {
    let v = parse(input);
    let mm :Vec<(u32, u32)> = v.iter().map(|v| min_max(v.as_slice())).collect();
    mm.iter().map(|(min, max)| max - min).sum()
}

fn divisable(vals: &[u32]) -> u32 {
    let mut sum = 0;
    for (i,v) in vals.iter().enumerate() {
        for v2 in &vals[i+1..] {
            if v2 % v == 0 {
                sum += v2/v;
            }
        }
    }
    sum
}

fn part2(input: &str) -> u32 {
    let v = parse(input);
    v.iter().map(|v| divisable(v.as_slice())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input = r"5 1 9 5
7 5 3
2 4 6 8";
        assert_eq!(18, part1(input));
    }

    #[test]
    fn p2() {
        let input = r"5 9 2 8
9 4 7 3
3 8 6 5";
        assert_eq!(9, part2(input));
    }
}
