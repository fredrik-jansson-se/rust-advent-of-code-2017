use regex::Regex;
use std::fs;
use std::str;
use std::collections::HashMap;

type Prog = Vec<char>;

#[derive(Debug, PartialEq)]
enum Cmd {
   Spin(usize),
   Exchange(usize, usize),
   Partner(char, char),
}

pub fn run() {
    let input = fs::read_to_string("day16.txt").unwrap();
    let mut prg = to_bytes("abcdefghijklmnop");
    run_1(&input, &mut prg, 1);
    let sprg : String = prg.iter().collect();
    println!("aoc16 - 1: {:?}", sprg);
    let mut prg = to_bytes("abcdefghijklmnop");
    run_1(&input, &mut prg, 1000000000);
    let sprg : String = prg.iter().collect();
    println!("aoc16 - 2: {:?}", sprg);
}

fn execute(cmd: &Cmd, prg: &mut Prog) {
    match cmd {
        Cmd::Spin(n) => {
            prg.rotate_right(*n);
        },
        Cmd::Exchange(p1, p2) => {
            prg.swap(*p1, *p2)
        },
        Cmd::Partner(c1, c2) => {
            let mut p1 = 0;
            let mut p2 = 0;
            let mut p1f = false;
            let mut p2f = false;
            for (i,c) in prg.iter().enumerate() {
                if *c1 == *c {
                    p1 = i;
                    p1f = true;
                }
                else if *c2 == *c {
                    p2 = i;
                    p2f = true;
                }
                if p1f && p2f {
                    break;
                }
            }
            prg.swap(p1, p2);
        }
    }
}

type ParseRow = fn(&str) -> Option<Cmd>;

fn s2i(s: &str) -> usize {
    usize::from_str_radix(s, 10).unwrap()
}

fn parse_spin(row: &str) -> Option<Cmd> {
    let r = Regex::new(r"s(\d+)").unwrap();
    match r.captures(row) {
        None => None,
        Some(c) => Some(Cmd::Spin(s2i(&c[1])))
    }
}

fn parse_exchange(row: &str) -> Option<Cmd> {
    let r = Regex::new(r#"x(\d+)/(\d+)"#).unwrap();
    match r.captures(row) {
        None => None,
        Some(c) => Some(Cmd::Exchange(s2i(&c[1]), s2i(&c[2])))
    }
}

fn parse_partner(row: &str) -> Option<Cmd> {
    let r = Regex::new(r#"p([a-z])/([a-z])"#).unwrap();
    match r.captures(row) {
        None => None,
        Some(c) => Some(Cmd::Partner(
                c[1].chars().next().unwrap(),
                c[2].chars().next().unwrap()))
    }
}

fn parse_row(row: &str) -> Cmd {
    let parsers : Vec<ParseRow> = vec!{
        parse_spin,
        parse_exchange,
        parse_partner,
    };

    for p in parsers {
        match p(row) {
            Some(c) => return c,
            None => continue
        }
    }

    panic!("No parser for {}", row);
}

fn to_bytes(str: &str) -> Prog {
    str.chars().collect()
}

fn run_1(scode: &str, prg: &mut Prog, iter: usize) {
    let code: Vec<Cmd> = scode.split(",").map(parse_row).collect();

    let mut lookup: HashMap<Prog, Prog> = HashMap::new();
    for i in 0..iter {
        if i % 100000 == 0 {
            println!("left: {}", iter - i);
        }
        let old_prg = prg.clone();
        match lookup.get(prg).cloned() {
            Some(prg2) => { 
                prg.clone_from_slice(&prg2); 
            }
            None => {
                for cmd in code.iter() {
                    execute(cmd, prg);
                }
                lookup.insert(old_prg, prg.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc16_spin() {
        let cmd = parse_row("s1");
        assert_eq!(Cmd::Spin(1), cmd);
        let mut prg = to_bytes("abcde");
        execute(&cmd, &mut prg);
        assert_eq!(to_bytes("eabcd"), prg);
    }

    #[test]
    fn aoc16_exchange() {
        let cmd = parse_row("x3/4");
        assert_eq!(Cmd::Exchange(3, 4), cmd);
        let mut prg = to_bytes("eabcd");
        execute(&cmd, &mut prg);
        assert_eq!(to_bytes("eabdc"), prg);
    }

    #[test]
    fn aoc16_partner() {
        let cmd = parse_row("pe/b");
        assert_eq!(Cmd::Partner('e', 'b'), cmd);
        let mut prg = to_bytes("eabdc");
        execute(&cmd, &mut prg);
        assert_eq!(to_bytes("baedc"), prg);
    }

    #[test]
    fn aoc16_1() {
        let code = "s1,x3/4,pe/b";
        let mut prg = to_bytes("abcde");
        run_1(code, &mut prg, 1);
        assert_eq!(to_bytes("baedc"), prg);
    }

    #[test]
    fn aoc16_2() {
        let code = "s1,x3/4,pe/b";
        let mut prg = to_bytes("abcde");
        run_1(code, &mut prg, 2);
        assert_eq!(to_bytes("ceadb"), prg);
    }
}
