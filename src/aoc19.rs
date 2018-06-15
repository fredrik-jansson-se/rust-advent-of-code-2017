use std::fs::File;
use std::io::prelude::*;

fn start_col(row: &Vec<char>) -> usize {
    row.iter().enumerate().find(|c| *c.1 == '|').unwrap().0
}

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
            Direction::Down => (0, 1), 
            Direction::Up => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
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
}

#[derive(Debug)]
struct State {
    row: usize,
    col: usize,
    dir: Direction,
    letters: String,
    steps: usize
}

type Map = Vec<Vec<char>>;

fn add(u: usize, i: isize) -> usize {
    (u as isize + i) as usize
}

impl State {
    fn new(col: usize) -> Self {
        State {
            row: 0,
            col: col,
            dir: Direction::Down,
            letters: String::new(),
            steps: 1,
        }
    }

    fn step(&mut self, map: &Map) -> bool {
        if !self.next_step(map) {
            return false
        }

        let c_at_p = map[self.row][self.col];
        if c_at_p.is_uppercase() {
            self.letters.push(c_at_p)
        }

        self.steps+=1;

        true
    }

    fn next_step(&mut self, map: &Map) -> bool {
        let (dc, dr) = self.dir.delta();
        if State::can_go(add(self.row, dr), add(self.col, dc), map) {
            self.row = add(self.row, dr);
            self.col = add(self.col, dc);
            return true
        }

        let old_dir = self.dir.clone();

        self.dir = old_dir.turn_right();
        let (dc, dr) = self.dir.delta();
        if State::can_go(add(self.row, dr), add(self.col, dc), map) {
            self.row = add(self.row, dr);
            self.col = add(self.col, dc);
            return true
        }

        self.dir = old_dir.turn_left();
        let (dc, dr) = self.dir.delta();
        if State::can_go(add(self.row, dr), add(self.col, dc), map) {
            self.row = add(self.row, dr);
            self.col = add(self.col, dc);
            return true
        }

        // println!("Nowhere to go {}, {}", self.row, self.col);

        false
    }

    fn can_go(row: usize, col: usize, map: &Map) -> bool {
        row < map.len() && col < map[row].len() && map[row][col] != ' '
    }
}

pub fn run() {
    let mut file = File::open("day19.txt").unwrap();
    let mut map = String::new();
    file.read_to_string(&mut map).unwrap();
    println!("1: {:?}", run_1(&map));
}

fn run_1(map: &str) -> (String, usize) {
    let vmap:Map = map.lines().map(|s| s.chars().collect()).collect();
    let mut state = State::new(start_col(&vmap[0]));
    loop {
        // println!("State: {:?}", state);
        if !state.step(&vmap) {
            break;
        }
    }
    //16099
    (state.letters, state.steps)
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input=
r"       |          
       |  +--+    
       A  |  C    
   F---|----E|--+ 
       |  |  |  D 
       +B-+  +--+";
        let (letters, _) = run_1(input);
        assert_eq!("ABCDEF", letters);
    }

    #[test]
    fn test_2() {
        let input=
r"       |          
       |  +--+    
       A  |  C    
   F---|----E|--+ 
       |  |  |  D 
       +B-+  +--+";
        let (_, steps) = run_1(input);
        assert_eq!(38, steps);
    }
}
