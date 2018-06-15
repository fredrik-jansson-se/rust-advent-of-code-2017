use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
enum Direction {
    Up, 
    Down,
    Left, 
    Right
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Down => (1, 0), 
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_around(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    row: isize,
    col: isize,
}

impl Pos {
    fn new(row: isize, col: isize) -> Self {
        Pos {
            row: row,
            col: col,
        }
    }
}

type Map = HashSet<Pos>;

struct State {
    pos: Pos,
    dir: Direction,
    infected: usize,
    map: Map,
}

impl State {
    fn new(pos: Pos, map: Map) -> Self {
        State {
            pos: pos,
            dir: Direction::Up,
            infected: 0,
            map: map,
        }
    }

    fn is_clean(&self) -> bool {
        !self.map.contains(&self.pos)
    }

    fn step (&mut self) {
        let new_dir = if self.is_clean() {
            self.infected += 1;
            self.map.insert(self.pos.clone());
            self.dir.turn_left()
        }
        else {
            self.map.remove(&self.pos);
            self.dir.turn_right()
        };

        self.dir = new_dir;

        let (dr, dc) = self.dir.delta();
        // println!("dir: {:?} ({}, {})", self.dir, dr, dc);
        self.pos.row += dr;
        self.pos.col += dc;
    }
}

fn parse_map(s: &str) -> (Pos, Map) {
    let mut map = Map::new(); 
    let mut height = 0;
    let mut width = 0;
    for (r, row) in s.lines().enumerate() {
        height = r + 1;
        for (c, col) in row.chars().enumerate() {
            width = c + 1;
            if col == '#' {
                map.insert(Pos::new(r as isize, c as isize));
            }
        }
    }
    (Pos::new((height/2) as isize, (width/2) as isize), map)
}

fn parse_map2(s: &str) -> (Pos, Map2) {
    let mut map = Map2::new(); 
    let mut height = 0;
    let mut width = 0;
    for (r, row) in s.lines().enumerate() {
        height = r + 1;
        for (c, col) in row.chars().enumerate() {
            width = c + 1;
            if col == '#' {
                map.insert(Pos::new(r as isize, c as isize),
                    CellState::Infected);
            }
        }
    }
    (Pos::new((height/2) as isize, (width/2) as isize), map)
}

fn run_1(s: &str, iters: usize) -> usize {
    let (start, map) = parse_map(s);
    let mut state = State::new(start, map);

    for i in 0..iters {
        // println!("{:?} - {:?}", state.pos, state.dir);
        state.step();
    }

    state.infected
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum CellState {
    Weakened,
    Infected,
    Flagged,
}

type Map2 = HashMap<Pos, CellState>;

#[derive(Debug)]
struct State2 {
    pos: Pos,
    dir: Direction,
    infected: usize,
    map: Map2,
}

impl State2 {
    fn new(pos: Pos, map: Map2) -> Self {
        State2 {
            pos: pos,
            dir: Direction::Up,
            infected: 0,
            map: map,
        }
    }

    fn is_clean(&self) -> bool {
        !self.map.contains_key(&self.pos)
    }

    fn step (&mut self) {
        let new_dir = if self.is_clean() {
            self.map.insert(self.pos.clone(), CellState::Weakened);
            self.dir.turn_left()
        }
        else {
            match self.map.get(&self.pos).unwrap() {
                CellState::Weakened => {
                    self.infected += 1;
                    self.map.insert(self.pos.clone(), CellState::Infected);
                    self.dir.clone()
                },
                CellState::Infected => {
                    self.map.insert(self.pos.clone(), CellState::Flagged);
                    self.dir.turn_right()
                },
                CellState::Flagged => {
                    self.map.remove(&self.pos);
                    self.dir.turn_around()
                }
            }
        };

        self.dir = new_dir;

        let (dr, dc) = self.dir.delta();
        // println!("dir: {:?} ({}, {})", self.dir, dr, dc);
        self.pos.row += dr;
        self.pos.col += dc;
    }
}

fn run_2(s: &str, iters: usize) -> usize {
    let (start, map) = parse_map2(s);
    let mut state = State2::new(start, map);

    for i in 0..iters {
        if i % 10000 == 0 {
            println!("{}", iters - i);
        }
        // println!("{:?} - {:?}", state.pos, state.dir);
        state.step();
    }

    state.infected
}

pub fn run() {
    let mut file = File::open("day22.txt").unwrap();
    let mut map = String::new();
    file.read_to_string(&mut map).unwrap();
    // println!("day22 - 1: {}", run_1(&map, 10000));
    println!("day22 - 2: {}", run_2(&map, 10000000));
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc22() {
        let m = r"..#
#..
...";
        let (start, map) = parse_map(m);
        assert_eq!(true, map.contains(&Pos::new(0, 2)));
        assert_eq!(true, map.contains(&Pos::new(1, 0)));
        assert_eq!(Pos::new(1,1), start);

        assert_eq!(5, run_1(m, 7));
        assert_eq!(41, run_1(m, 70));
        assert_eq!(5587, run_1(m, 10000));


        assert_eq!(26, run_2(m, 100));
        assert_eq!(2511944, run_2(m, 10000000));
    }
}
