// extern crate combine;
extern crate regex;
#[macro_use] extern crate lazy_static;

use std::env;

mod aoc1;
mod aoc2;
mod aoc3;
mod aoc16;
mod aoc18;  
mod aoc19;
mod aoc20;
mod aoc21;
mod aoc22;
mod aoc23;
mod aoc24;
mod aoc25;

fn main() {
    let mut a = env::args();
    a.next();

    let day = match a.next() {
        Some(s) => usize::from_str_radix(&s, 10).unwrap(),
        None => 0,
    };

    match day {
        1 => aoc1::run(),
        2 => aoc2::run(),
        20 => aoc20::run(),
        21 => aoc21::run(),
        22 => aoc22::run(),
        23 => aoc23::run(),
        24 => aoc24::run(),
        25 => aoc25::run(),
        _ => ()
    }
}
