use regex::Regex;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day24.txt").unwrap();
    // println!("aoc24-1: {}", run_1(&input));
    println!("aoc24-2: {:?}", run_2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Component {
    left: usize,
    right: usize,
}

impl Component {
    fn new(left: usize, right: usize) -> Self {
        Component {
            left: left,
            right: right
        }
    }

    fn valid_start(&self) -> bool {
        self.left == 0 || self.right == 0
    }

    fn can_follow(&self, val: usize) -> bool {
        self.left == val || self.right == val
    }

    fn get_other_side(&self, val:usize) -> usize {
        if self.left == val {
            self.right
        }
        else {
            self.left
        }
    }
}

fn s2i(s: &str) -> usize {
    usize::from_str_radix(s, 10).unwrap()
}

fn parse(row: &str) -> Component {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(\d+)/(\d+)"#).unwrap();
    }
    let c = RE.captures(row).unwrap();
    Component::new(s2i(&c[1]), s2i(&c[2]))
}

fn filter_copy(components: &[Component], f: &Component) -> Vec<Component> {
    let mut s = Vec::new();

    for c in components {
        if c != f {
            s.push(c.clone());
        }
    }

    s
}

#[derive(Debug, Clone)]
struct Part {
    value: usize,
    children: Vec<Part>
}

impl Part {
    fn new(value: usize) -> Self {
        Part {
            value: value,
            children: Vec::new(),
        }
    }   

    fn find_children(&mut self, parts: Vec<Component>) -> Vec<Component> {
        let (c,o) : (Vec<Component>, Vec<Component>) = parts.iter().partition(|&p| p.can_follow(self.value));

        self.children = c.iter().map(|&p| Part::new(p.get_other_side(self.value))).collect();

        o
    }
}

fn build_bridges(p: &mut Part, components: Vec<Component>) {
    let p_val = p.value;
    let childs = components.iter().filter(|&c| c.can_follow(p_val));

    for c in childs {
        let mut new_p = Part::new(c.get_other_side(p_val));
        let comp: Vec<Component> = components.iter().cloned().filter(|&cc| *c != cc).collect();
        build_bridges(&mut new_p, comp);
        p.children.push(new_p);
    }
}

fn max_bridge(p: &Part) -> usize {
    let c_max = match p.children.iter().map(|&ref c| max_bridge(&c)).max() {
        Some(m) => m,
        None => 0
    };
    if c_max != 0 {
        2 * p.value + c_max
    }
    else {
        p.value
    }
}

fn longest_bridge(p: &Part) -> (usize, usize) {
    let mut c_max_depth = 0;
    let mut mb = 0;
    let mut c_max_bridge = 0;
    for c in p.children.iter() {
        let (cd, cb) = longest_bridge(c);
        if cd > c_max_depth {
            println!("cd: {} cb: {}", cd, cb);
            c_max_depth = cd;
            c_max_bridge = cb;
        }
        else if cd == c_max_depth && cb > c_max_bridge {
            c_max_bridge = cb;
        }
    }
    let p_val = if p.children.len() > 0 {
        p.value * 2
    }
    else {
        p.value
    };

    (1 + c_max_depth, p_val + c_max_bridge)
}

fn run_1(input: &str) -> usize {
    let components : Vec<Component> = input.lines().map(parse).collect();

    let mut first = Part::new(0);

    build_bridges(&mut first, components.clone());

    // println!("parts: {:?}", first);

    max_bridge(&first)
}

fn run_2(input: &str) -> (usize, usize) {
    let components : Vec<Component> = input.lines().map(parse).collect();

    let mut first = Part::new(0);

    build_bridges(&mut first, components.clone());

    // println!("parts: {:?}", first);

    longest_bridge(&mut first)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc24_parse() {
        assert_eq!(Component::new(0,2), parse("0/2"));
        assert_eq!(Component::new(10,1), parse("10/1"));
    }

    #[test]
    fn aoc24_1() {
        assert_eq!(31, run_1("0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10"));
    }

    #[test]
    fn aoc24_2() {
        assert_eq!((5, 19), run_2("0/2\n2/2\n2/3\n3/4\n3/5\n0/1\n10/1\n9/10"));
    }
}
