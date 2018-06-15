 // #[allow(unused_must_use)]
use std::collections::{HashSet};

type StateFn = fn (&mut CPU) -> ();

#[derive(Debug)]
struct CPU {
    pos: i64,
    state: char,
    tape: HashSet<i64>,
}

impl CPU {
    fn new() -> Self {
        CPU {
            pos: 0,
            state: 'A',
            tape: HashSet::new(),
        }
    }

    fn cur_val(&self) -> bool {
        self.tape.contains(&self.pos)
    }

    fn set(&mut self) {
        // println!("set: {}", self.pos);
        self.tape.insert(self.pos);
    }

    fn reset(&mut self) {
        // println!("reset: {}", self.pos);
        self.tape.remove(&self.pos);
    }

    fn checksum(&self) -> usize {
        self.tape.len()
    }
}

fn run_1(steps: usize, sfn: StateFn) -> usize {
    let mut cpu = CPU::new();
    for i in 0..steps {
        // println!("=> {:?}", cpu);
        sfn(&mut cpu);
        // println!("<= {:?}", cpu);
    }
    cpu.checksum()
}

pub fn run() {
    let d1 = run_1(12919244, day_1);
    println!("t1: {}", d1); 
}

fn day_1(c: &mut CPU) -> () {
    match c.state {
        'A' => {
            if c.cur_val() {
                c.reset();
                c.pos -= 1;
                c.state = 'C';
            }
            else {
                c.set();
                c.pos += 1;
                c.state = 'B';
            }
        },
        'B' => {
            if c.cur_val() {
                c.set();
                c.pos += 1;
                c.state = 'D';
            }
            else {
                c.set();
                c.pos -= 1;
                c.state = 'A';
            }
        },
        'C' => {
            if c.cur_val() {
                c.reset();
                c.pos -= 1;
                c.state = 'E';
            }
            else {
                c.set();
                c.pos += 1;
                c.state = 'A';
            }
        },
        'D' => {
            if c.cur_val() {
                c.reset();
                c.pos += 1;
                c.state = 'B';
            }
            else {
                c.set();
                c.pos += 1;
                c.state = 'A';
            }
        },
        'E' => {
            if c.cur_val() {
                c.set();
                c.pos -= 1;
                c.state = 'C';
            }
            else {
                c.set();
                c.pos -= 1;
                c.state = 'F';
            }
        },
        'F' => {
            if c.cur_val() {
                c.set();
                c.pos += 1;
                c.state = 'A';
            }
            else {
                c.set();
                c.pos += 1;
                c.state = 'D';
            }
        },
        s => panic!("Unknown state: {:?}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn day_1_sfn(c: &mut CPU) -> () {
        match c.state {
            'A' => {
                if c.cur_val() {
                    c.reset();
                    c.pos -= 1;
                    c.state = 'B';
                }
                else {
                    c.set();
                    c.pos += 1;
                    c.state = 'B';
                }
            },
            'B' => {
                if c.cur_val() {
                    c.set();
                    c.pos += 1;
                    c.state = 'A';
                }
                else {
                    c.set();
                    c.pos -= 1;
                    c.state = 'A';
                }
            },
            s => panic!("Unknown state: {:?}", s)
        }
    }
    #[test]
    fn day_1() {
        assert_eq!(3, run_1(6, day_1_sfn));
    }
}
