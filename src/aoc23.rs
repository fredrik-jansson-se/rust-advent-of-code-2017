use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
enum Cmd {
    SetV(char, isize),
    SetR(char, char),
    MulV(char, isize),
    MulR(char, char),
    JnzVV(isize, isize),
    JnzV(char, isize),
    JnzR(char, char),
    AddV(char, isize),
    AddR(char, char),
    SubV(char, isize),
    SubR(char, char),
    ModV(char, isize),
    ModR(char, char),
    Npr(char, char),
}

#[derive(Debug)]
struct CPU {
    regs: HashMap<char, isize>,
    pc: usize,
    mul_cnt: usize,
}

impl CPU {
    fn new(a_val: isize ) -> Self {
        let mut regs = HashMap::new();
        regs.insert('a', a_val);
        CPU {
            regs: regs,
            pc: 0,
            mul_cnt: 0,
        }
    }

    fn get(&self, r: char) -> isize {
        match self.regs.get(&r) {
            Some(v) => *v,
            None => 0
        }
    }

    fn step(&mut self, c: &Cmd) {
        match *c {
            Cmd::SetV(r, v) => { 
                self.regs.insert(r, v); 
                self.pc += 1;
            },
            Cmd::SetR(r, o) => {
                let v = self.get(o);
                self.regs.insert(r, v);
                self.pc += 1;
            },
            Cmd::MulV(r, o) => {
                let v = self.get(r);
                self.regs.insert(r, v*o);
                self.pc += 1;
                self.mul_cnt += 1;
            },
            Cmd::MulR(r, o) => {
                let v = self.get(r);
                let vo = self.get(o);
                self.regs.insert(r, v*vo);
                self.pc += 1;
                self.mul_cnt += 1;
            },
            Cmd::AddR(r, o) => {
                let v = self.get(r);
                let vo = self.get(o);
                self.regs.insert(r, v+vo);
                self.pc += 1;
            },
            Cmd::AddV(r, o) => {
                let v = self.get(r);
                self.regs.insert(r, v+o);
                self.pc += 1;
            },
            Cmd::SubR(r, o) => {
                let v = self.get(r);
                let vo = self.get(o);
                self.regs.insert(r, v-vo);
                self.pc += 1;
            },
            Cmd::SubV(r, o) => {
                let v = self.get(r);
                self.regs.insert(r, v-o);
                self.pc += 1;
            },
            Cmd::ModR(r, o) => {
                let v = self.get(r);
                let vo = self.get(o);
                self.regs.insert(r, v%vo);
                self.pc += 1;
            },
            Cmd::ModV(r, o) => {
                let v = self.get(r);
                self.regs.insert(r, v%o);
                self.pc += 1;
            },
            Cmd::JnzR(r, o) => {
                let v = self.get(o);
                self.step(&Cmd::JnzV(r, v));
            },
            Cmd::JnzV(r, o) => {
                if self.get(r) != 0 {
                    let pc = self.pc as isize + o;
                    self.pc = pc as usize;
                }
                else {
                    self.pc += 1;
                }
            },
            Cmd::JnzVV(v, o) => {
                if v != 0 {
                    let pc = self.pc as isize + o;
                    self.pc = pc as usize;
                }
                else {
                    self.pc += 1;
                }
            },
            Cmd::Npr(r1, r2) => {
                let v2 = self.get(r2);
                if is_composite(v2) {
                    self.regs.insert(r1, 0);
                }
                else {
                    self.regs.insert(r1, 1);
                }
                self.pc += 1;
            }
        }
    }
}

fn is_composite(n: isize) -> bool {
    let sqn = (n as f64).sqrt() as isize;
    for i in 2..(sqn+1) {
        if n % i == 0 {
            return true;
        }
    }
    false
}

fn s2i(s: &str) -> isize {
    isize::from_str_radix(s, 10).unwrap()
}

fn parse_set(row: &str) -> Option<Cmd> {
    let r_v = Regex::new(r"set\s+(\w)\s+(-?\d+)").unwrap();
    if let Some(c) = r_v.captures(row) {
        return Some(Cmd::SetV(c[1].chars().next().unwrap(), s2i(&c[2])))
    }

    let r_r = Regex::new(r"set\s+(\w)\s+(\w)").unwrap();
    match r_r.captures(row) {
        None => None,
        Some(c) => Some(Cmd::SetR(c[1].chars().next().unwrap(), 
                                  c[2].chars().next().unwrap()))
    }
}

fn parse_npr(row: &str) -> Option<Cmd> {
    let r_r = Regex::new(r"npr\s+(\w)\s+(\w)").unwrap();
    match r_r.captures(row) {
        None => None,
        Some(c) => Some(Cmd::Npr(c[1].chars().next().unwrap(), 
                                 c[2].chars().next().unwrap()))
    }
}

fn parse_mul(row: &str) -> Option<Cmd> {
    let r_v = Regex::new(r"mul\s+(\w)\s+(-?\d+)").unwrap();

    if let Some(c) = r_v.captures(row) {
        return Some(Cmd::MulV(c[1].chars().next().unwrap(), s2i(&c[2])))
    }

    let r_r = Regex::new(r"mul\s+(\w)\s+(\w)").unwrap();
    match r_r.captures(row) {
        None => None,
        Some(c) => Some(Cmd::MulR(c[1].chars().next().unwrap(),
                                  c[2].chars().next().unwrap()))
    }
}

fn parse_jnz(row: &str) -> Option<Cmd> {
    let r_vv = Regex::new(r"jnz\s+(\d)\s+(-?\d+)").unwrap();
    if let Some(c) = r_vv.captures(row) {
        return Some(Cmd::JnzVV(s2i(&c[1]), s2i(&c[2])))
    }

    let r_v = Regex::new(r"jnz\s+(\w)\s+(-?\d+)").unwrap();
    let c1 = r_v.captures(row);
    if let Some(c) = c1 {
        return Some(Cmd::JnzV(c[1].chars().next().unwrap(), s2i(&c[2])))
    }

    let r_v = Regex::new(r"jnz\s+(\w)\s+(\w+)").unwrap();
    match r_v.captures(row) {
        None => None,
        Some(c) => Some(Cmd::JnzR(c[1].chars().next().unwrap(), c[2].chars().next().unwrap()))
    }
}

fn parse_add(row: &str) -> Option<Cmd> {
    let r_v = Regex::new(r"add\s+(\w)\s+(-?\d+)").unwrap();
    let c1 = r_v.captures(row);
    if let Some(c) = c1 {
        return Some(Cmd::AddV(c[1].chars().next().unwrap(), s2i(&c[2])))
    }

    let r_v = Regex::new(r"add\s+(\w)\s+(\w+)").unwrap();
    match r_v.captures(row) {
        None => None,
        Some(c) => Some(Cmd::AddR(c[1].chars().next().unwrap(), c[2].chars().next().unwrap()))
    }
}

fn parse_sub(row: &str) -> Option<Cmd> {
    let r_v = Regex::new(r"sub\s+(\w)\s+(-?\d+)").unwrap();
    let c1 = r_v.captures(row);
    if let Some(c) = c1 {
        return Some(Cmd::SubV(c[1].chars().next().unwrap(), s2i(&c[2])))
    }

    let r_v = Regex::new(r"sub\s+(\w)\s+(\w+)").unwrap();
    match r_v.captures(row) {
        None => None,
        Some(c) => Some(Cmd::SubR(c[1].chars().next().unwrap(), c[2].chars().next().unwrap()))
    }
}

fn parse_mod(row: &str) -> Option<Cmd> {
    let r_v = Regex::new(r"mod\s+(\w)\s+(-?\d+)").unwrap();
    let c1 = r_v.captures(row);
    if let Some(c) = c1 {
        return Some(Cmd::ModV(c[1].chars().next().unwrap(), s2i(&c[2])))
    }

    let r_v = Regex::new(r"mod\s+(\w)\s+(\w+)").unwrap();
    match r_v.captures(row) {
        None => None,
        Some(c) => Some(Cmd::ModR(c[1].chars().next().unwrap(), c[2].chars().next().unwrap()))
    }
}

type ParseRow = fn(&str) -> Option<Cmd>;

fn parse_row(row: &str) -> Cmd {
    let parsers : Vec<ParseRow> = vec!{
        parse_mul,
        parse_mod,
        parse_jnz,
        parse_set,
        parse_add,
        parse_sub,
        parse_npr,
    };

    for p in parsers {
        match p(row) {
            Some(c) => return c,
            None => continue
        }
    }

    panic!("No parser for {}", row);
}

fn parse(code: &str) -> Vec<Cmd> {
    code.lines().map(parse_row).collect()
}

pub fn run_1(code: &str) -> usize {
    let cmds = parse(&code);
    // println!("Cmds: {:?}", cmds);
    let mut cpu = CPU::new(0);
    loop {
        let pc = cpu.pc;
        // println!("pc: {}", pc);
        cpu.step(&cmds[pc]);
        // println!("cpu: {:?}", cpu);
        if cpu.pc >= cmds.len() {
            break;
        }
    }
    cpu.mul_cnt
}

pub fn run_2(code: &str) -> isize {
     let cmds = parse(&code);
     println!("Cmds: {:?}", cmds);
     let mut cpu_1 = CPU::new(1);

     loop {
         let pc = cpu_1.pc;
         // println!("pc: {}", pc);
         println!("cpu {:?}", cpu_1);
         cpu_1.step(&cmds[pc]);
         if cpu_1.pc >= cmds.len() {
             break;
         }
     }
     return *cpu_1.regs.get(&'h').unwrap();
    

    // let mut h = 0;

    // let mut b = 81 * 100 + 100000;
    // let b_end = b + 17000;

    // while b <= b_end {
    //     if is_composite(b) {
    //         h+=1;
    //     }
    //     b += 17;
    // }


    // h
}

pub fn run() {
    let mut file = File::open("day23.txt").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    println!("day21-1: {}", run_1(&code));
    let mut file = File::open("day23-opt.txt").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    println!("day21-2: {:?}", run_2(&code));
    // 947 too high
    // 908 too low
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc23_parse_set() {
        assert_eq!(Cmd::SetV('i', 31), parse_row("set i 31"));
        assert_eq!(Cmd::SetR('b', 'p'), parse_row("set b p"));
    }

    #[test]
    fn aoc23_parse_mul() {
        assert_eq!(Cmd::MulV('i', 112), parse_row("mul i 112"));
        assert_eq!(Cmd::MulV('p', -1), parse_row("mul p -1"));
        assert_eq!(Cmd::MulR('p', 'p'), parse_row("mul p p"));
    }

    #[test]
    fn aoc23_parse_jnz() {
        assert_eq!(Cmd::JnzVV(1, 3), parse_row("jnz 1 3"));
        assert_eq!(Cmd::JnzV('i', -2), parse_row("jnz i -2"));
        assert_eq!(Cmd::JnzR('p', 'a'), parse_row("jnz p a"));
    }

    //add
    #[test]
    fn aoc23_parse_add() {
        assert_eq!(Some(Cmd::AddV('i', -2)), parse_add("add i -2"));
        assert_eq!(Some(Cmd::AddR('b', 'p')), parse_add("add b p"));
    }
    //sub
    #[test]
    fn aoc23_parse_sub() {
        assert_eq!(Cmd::SubV('i', -2), parse_row("sub i -2"));
        assert_eq!(Cmd::SubR('b', 'p'), parse_row("sub b p"));
    }
    //
    // mod
    #[test]
    fn aoc23_parse_mod() {
        assert_eq!(Some(Cmd::ModV('i', -2)), parse_mod("mod i -2"));
        assert_eq!(Some(Cmd::ModR('p', 'a')), parse_mod("mod p a"));
    }
}
