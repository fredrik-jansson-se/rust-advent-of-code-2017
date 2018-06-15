use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::sync::mpsc::{channel, Sender, Receiver};

#[derive(Debug, PartialEq)]
enum Cmd {
    Snd(char),
    SetV(char, isize),
    SetR(char, char),
    MulV(char, isize),
    MulR(char, char),
    JgzVV(isize, isize),
    JgzV(char, isize),
    JgzR(char, char),
    AddV(char, isize),
    AddR(char, char),
    ModV(char, isize),
    ModR(char, char),
    Rcv(char),
}

#[derive(Debug)]
struct CPU {
    id: String,
    regs: HashMap<char, isize>,
    pc: usize,
    s: Sender<isize>,
    r: Receiver<isize>,
    last_recv: isize,
    is_waiting: bool,
    send_cnt: usize,
}

impl CPU {
    fn new(program_id: isize ) -> Self {
        let mut regs = HashMap::new();
        regs.insert('p', program_id);
        let (tx, rx) = channel();   
        CPU {
            id: format!("{}", program_id),
            regs: regs,
            pc: 0,
            s: tx,
            r: rx,
            last_recv: 0,
            is_waiting: false,
            send_cnt: 0,
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
            Cmd::Snd(r) => {
                let v = self.get(r);
                // Drop old messages
                // while let Some(_) = self.r.try_iter().next() { }
                // Send new message
                self.s.send(v).unwrap();
                self.pc += 1;
                self.send_cnt += 1;
                // println!("Send-{} {}:{} ({})", self.id, r, v, self.send_cnt);
            },
            Cmd::Rcv(r) => {
                if true || self.get(r) != 0 {
                    match self.r.try_recv() {
                        Ok(v) => {
                            self.pc += 1;
                            self.last_recv = v;
                            self.regs.insert(r, v);
                            // println!("Recv-{} {}:{}", self.id, r, v);
                            self.is_waiting = false;
                        },
                        _ => {
                            self.is_waiting = true;
                        }
                    }
                }
                else {
                    self.pc += 1;
                }
            },
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
            },
            Cmd::MulR(r, o) => {
                let v = self.get(r);
                let vo = self.get(o);
                self.regs.insert(r, v*vo);
                self.pc += 1;
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
            Cmd::JgzR(r, o) => {
                let v = self.get(o);
                self.step(&Cmd::JgzV(r, v));
            },
            Cmd::JgzV(r, o) => {
                if self.get(r) > 0 {
                    let pc = self.pc as isize + o;
                    self.pc = pc as usize;
                }
                else {
                    self.pc += 1;
                }
            },
            Cmd::JgzVV(v, o) => {
                if v > 0 {
                    let pc = self.pc as isize + o;
                    self.pc = pc as usize;
                }
                else {
                    self.pc += 1;
                }
            }
        }
    }
}

fn parse_snd(row: &str) -> Option<Cmd> {
    let r = Regex::new(r"snd\s+(\w)").unwrap();
    match r.captures(row) {
        None => None,
        Some(c) => Some(Cmd::Snd(c[1].chars().next().unwrap()))
    }
}

fn parse_rcv(row: &str) -> Option<Cmd> {
    let r = Regex::new(r"rcv\s+(\w)").unwrap();
    match r.captures(row) {
        None => None,
        Some(c) => Some(Cmd::Rcv(c[1].chars().next().unwrap()))
    }
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

fn parse_jgz(row: &str) -> Option<Cmd> {
    let r_vv = Regex::new(r"jgz\s+(\d)\s+(-?\d+)").unwrap();
    if let Some(c) = r_vv.captures(row) {
        return Some(Cmd::JgzVV(s2i(&c[1]), s2i(&c[2])))
    }

    let r_v = Regex::new(r"jgz\s+(\w)\s+(-?\d+)").unwrap();
    let c1 = r_v.captures(row);
    if let Some(c) = c1 {
        return Some(Cmd::JgzV(c[1].chars().next().unwrap(), s2i(&c[2])))
    }

    let r_v = Regex::new(r"jgz\s+(\w)\s+(\w+)").unwrap();
    match r_v.captures(row) {
        None => None,
        Some(c) => Some(Cmd::JgzR(c[1].chars().next().unwrap(), c[2].chars().next().unwrap()))
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
        parse_rcv,
        parse_mod,
        parse_jgz,
        parse_set,
        parse_add,
        parse_snd,
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

pub fn run_1(code: &str) -> isize {
    let cmds = parse(&code);
    println!("Cmds: {:?}", cmds);
    let mut cpu = CPU::new(0);
    loop {
        let pc = cpu.pc;
        println!("pc: {}", pc);
        cpu.step(&cmds[pc]);
        println!("cpu: {:?}", cpu);
        if cpu.pc >= cmds.len() {
            break;
        }
        if cpu.last_recv > 0 {
            break;
        }
    }
    cpu.last_recv
}

pub fn run_2(code: &str) -> (usize, usize) {
    let cmds = parse(&code);
    println!("Cmds: {:?}", cmds);
    let mut cpu_1 = CPU::new(0);
    let mut cpu_2 = CPU::new(1);
    let r1 = cpu_1.r;
    cpu_1.r = cpu_2.r;
    cpu_2.r = r1;

    // println!("{:?}", cpu_1.r.try_recv());
    // return 0;
    let mut last_i = 0;
    loop {
        let pc = cpu_1.pc;
        // println!("pc: {}", pc);
        cpu_1.step(&cmds[pc]);
        if cpu_1.get('i') != last_i {
            last_i = cpu_1.get('i');
            // println!("i = {}", last_i);
        }
        // println!("cpu_1: {:?}", cpu_1);
        if cpu_1.pc >= cmds.len() {
            break;
        }
        let pc = cpu_2.pc;
        // println!("pc: {}", pc);
        cpu_2.step(&cmds[pc]);
        if cpu_2.pc >= cmds.len() {
            break;
        }

        if cpu_1.is_waiting && cpu_2.is_waiting {
            break;
        }
    }
    (cpu_1.send_cnt, cpu_2.send_cnt)
}

pub fn run() {
    let mut file = File::open("day18.txt").unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();
    // println!("1: {}", run_1(&code));
    // 6096
    println!("2: {:?}", run_2(&code));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc18_parse_snd() {
        assert_eq!(Cmd::Snd('a'), parse_row("snd a"));
    }

    #[test]
    fn aoc18_parse_set() {
        assert_eq!(Cmd::SetV('i', 31), parse_row("set i 31"));
        assert_eq!(Cmd::SetR('b', 'p'), parse_row("set b p"));
    }

    #[test]
    fn aoc18_parse_mul() {
        assert_eq!(Cmd::MulV('i', 112), parse_row("mul i 112"));
        assert_eq!(Cmd::MulV('p', -1), parse_row("mul p -1"));
        assert_eq!(Cmd::MulR('p', 'p'), parse_row("mul p p"));
    }

    #[test]
    fn aoc18_parse_jgz() {
        assert_eq!(Some(Cmd::JgzVV(1, 3)), parse_jgz("jgz 1 3"));
        assert_eq!(Some(Cmd::JgzV('i', -2)), parse_jgz("jgz i -2"));
        assert_eq!(Some(Cmd::JgzR('p', 'a')), parse_jgz("jgz p a"));
    }

    //add
    #[test]
    fn aoc18_parse_add() {
        assert_eq!(Some(Cmd::AddV('i', -2)), parse_add("add i -2"));
        assert_eq!(Some(Cmd::AddR('b', 'p')), parse_add("add b p"));
    }
    //
    // mod
    #[test]
    fn aoc18_parse_mod() {
        assert_eq!(Some(Cmd::ModV('i', -2)), parse_mod("mod i -2"));
        assert_eq!(Some(Cmd::ModR('p', 'a')), parse_mod("mod p a"));
    }
    //
    // rcv
    #[test]
    fn aoc18_parse_rcv() {
        assert_eq!(Some(Cmd::Rcv('i')), parse_rcv("rcv i"));
    }

    #[test]
    fn aoc18_run_1() {
        let code = r"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        assert_eq!(4, run_1(&code));
    }

}
