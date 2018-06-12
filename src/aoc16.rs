use std::fs;

// #[macro_use]
// use combine::{sep_by, Parser, ParseError, Stream, many1};
// use combine::parser::char::{letter, space, digit};

type Prog = Vec<u8>;

type Move = Box<Fn(Prog) -> Prog>;

pub fn run() {
    let input = fs::read_to_string("day16.txt").unwrap();
}

fn spin(n: usize) -> Move {
    Box::new(move |mut v| {
        v.rotate_left(n);
        v
    }
    )
}

fn exchange(a: usize, b: usize) -> Move {
    Box::new(move |mut v| {
        v.swap(a,b);
        v
    }
    )
}

fn swap(a: char, b: char) -> Move {
    let ac = 0u8;
    let bc = 0u8;
    Box::new(move |mut v| {
        let mut ap: usize = 0;
        let mut bp: usize = 0;
        for (idx, c) in v.iter().enumerate() {
            if ac == *c {
                ap = idx;
            }
            else if bc == *c {
                bp = idx;
            }
        }
        v.swap(ap, bp);
        v
    }
    )
}

fn parse(input: &str) -> () {
    let mut v = input.split(',');
    
    println!("v = {:?}", v)
}

fn part1(input: &str) -> &str {
    let res = parse(input);    
    ""
}

fn to_bytes(str: &str) -> Prog {
    String::from(str).into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]
    // fn spin_test() {
    //     assert_eq!(to_bytes("cdeab"), spin(2)(to_bytes("abcde")));
    // }

    #[test]
    fn exchange_test() {
        assert_eq!(to_bytes("eabdc"), exchange(3,4)(to_bytes("eabcd")));
    }

    #[test]
    fn swap_test() {
        assert_eq!(to_bytes("baedc"), swap('e', 'b')(to_bytes("eabdc")));
    }

    // #[test]
    // fn p1() {
    //     let input="s1,x3/4,pe/b";
    //     assert_eq!("baedc", part1(input));

    // }
}
