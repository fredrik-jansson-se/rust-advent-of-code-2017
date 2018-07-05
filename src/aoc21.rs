use std::fs;
// use std::collections;

type Map = Vec<Vec<bool>>;

#[derive(Debug, PartialEq)]
struct Rule {
    sources: Vec<Map>,
    dest: Map,
    size: usize,
}

impl Rule {
    fn new(source: Map, dest: Map) -> Self {
        let size = source.len();
        let mut sources = Vec::new();
        let mut r = source.clone();

        for _ in 0..3 {
            r = rotate(&r);
            sources.push(r.clone());
        }

        r = flip(&source);
        sources.push(r.clone());
        for _ in 0..3 {
            r = rotate(&r);
            sources.push(r.clone());
        }

        sources.push(source);

        Rule {
            sources: sources,
            dest: dest,
            size: size,
        }
    }
}

fn start_map() -> Map {
    vec![
        vec!{false, true,  false},
        vec!{false, false, true},
        vec!{true,  true,  true},
    ]
}

fn parse_map(s: &str) -> Map {
    let mut m = Map::new();

    let lns = s.split("/");
    for row in lns {
        let mut map_row = Vec::new();
        for c in row.chars() {
            map_row.push(c == '#');
        }
        m.push(map_row);
    }

    m
}

fn parse_rule(s: &str) -> Rule {
    let row: Vec<&str> = s.split(" => ").collect();
    let src = parse_map(&row[0]);
    let dst = parse_map(&row[1]);
    Rule::new(src, dst)
}

fn new_map(size: usize) -> Map {
    let mut m = Map::new();

    for _ in 0..size {
        let row = vec![false; size];
        m.push(row);
    }

    m
}

fn expand(map: Map, rules: &[Rule]) -> Map {
    let step = if map.len() % 2 == 0 {
        2
    }
    else {
        3
    };

    let new_step = step + 1;
    let mut new_map = new_map(map.len() / step * new_step);

    let mut r = 0;
    let mut new_r = 0;
    while r < map.len() {
        let mut c = 0;
        let mut new_c = 0;
        while c < map.len() {
            let mut found = false;
            for rule in rules {
                if rule.size != step {
                    continue;
                }
                for s in &rule.sources {
                    if is_match(s, &map, r, c) {
                        // println!("Match");
                        // print_map(s);
                        set_pattern(&rule.dest, &mut new_map, new_r, new_c);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if !found {
                print_map(&map);
                println!("row: {} col: {} step: {}", r, c, step);
                panic!("No rule!!");
            }

            c += step;
            new_c += new_step;
        }
        r += step;
        new_r += new_step;
    }
    println!("new_map size: {}", new_map.len());
    new_map
}

pub fn run() {
    let input = fs::read_to_string("day21.txt").unwrap();
    let rules : Vec<Rule> = input.lines().map(parse_rule).collect();
    println!("aoc21-1: {}", run_1(&rules, 5));
    println!("aoc21-2: {}", run_1(&rules, 18));
}

fn run_1(rules: &[Rule], iters: usize) -> usize {
    let mut map = start_map();
    println!("start map");
    print_map(&map);
    for _ in 0..iters {
        map = expand(map, rules);
        // println!("result");
        // print_map(&map);
    }
    let mut cnt = 0;
    for row in map.iter() {
        for c in row.iter() {
            if *c {
                cnt+=1;
            }
        }
    }
    cnt
}

fn is_match(pattern: &Map, map: &Map, check_row: usize, check_col: usize) -> bool {
    for (r, row) in pattern.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col != map[r+check_row][c+check_col] {
                return false;
            }
        }
    }
    true
}

fn set_pattern(pattern: &Map, map: &mut Map, check_row: usize, check_col: usize) {
    for (r,row) in pattern.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if *col {
                map[r+check_row][c+check_col] = true;
            }
        }
    }
}

fn rotate(pattern: &Map) -> Map {
    if pattern.len() == 2 {
        rotate2(pattern)
    }
    else {
        rotate3(pattern)
    }
}

fn rotate2(pattern: &Map) -> Map {
    let mut r = pattern.clone();

    r[0][0] = pattern[0][1];
    r[0][1] = pattern[1][1];
    r[1][0] = pattern[0][0];
    r[1][1] = pattern[1][0];

    r
}

fn rotate3(pattern: &Map) -> Map {
    let mut r = pattern.clone();

    // 1 2 3 
    // 4 5 6
    // 7 8 9
    //
    // 3 6 9
    // 2 5 8
    // 1 4 7

    r[0][0] = pattern[0][2];
    r[0][1] = pattern[1][2];
    r[0][2] = pattern[2][2];

    r[1][0] = pattern[0][1];
    r[1][2] = pattern[2][1];

    r[2][0] = pattern[0][0];
    r[2][1] = pattern[1][0];
    r[2][2] = pattern[2][0];


    r
}

fn flip(pattern: &Map) -> Map {
    let mut f = pattern.clone();

    for (r, row) in pattern.iter().enumerate() {
        let w = row.len();
        for (c, col) in row.iter().enumerate() {
            f[r][w - c - 1] = *col;
        }
    }

    f
}

fn print_map(m: &Map) {
    println!("");
    for row in m.iter() {
        for c in row.iter() {
            if *c {
                print!("#");
            }
            else {
                print!(".");
            }
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc21_parse_rule() {
        assert_eq!(parse_rule("../.# => ##./#../..."), 
                   Rule::new(parse_map("../.#"), parse_map("##./#../...")));
        assert_eq!(parse_rule(".#./..#/### => #..#/..../..../#..#"), Rule::new(
                parse_map(".#./..#/###"), parse_map("#..#/..../..../#..#")
                ));
    }

    #[test]
    fn aoc21_check_match() {
        // * *
        //  *
        // * *
        let vv : Vec<Vec<bool>> = vec![
            vec!{ true,  false, true },
            vec!{ false, true,  false },
            vec!{ true,  false, true },
        ];

        // * 
        //  *
        let t1 : Vec<Vec<bool>> = vec![
            vec!{ true, false },
            vec!{ false, true },
        ];

        assert_eq!(true, is_match(&t1, &vv, 0, 0));
        assert_eq!(true, is_match(&t1, &vv, 1, 1));
        assert_eq!(false, is_match(&t1, &vv, 0, 1));
        assert_eq!(false, is_match(&t1, &vv, 1, 0));
    }

    #[test]
    fn aoc21_check_set_pattern() {
        let mut vv : Vec<Vec<bool>> = vec![
            vec!{ false, false, false },
            vec!{ false, false, false },
            vec!{ false, false, false },
        ];

        // * 
        //  *
        let t1 : Vec<Vec<bool>> = vec![
            vec!{ true, false },
            vec!{ false, true },
        ];

        set_pattern(&t1, &mut vv, 0, 0);
        set_pattern(&t1, &mut vv, 1, 1);

        assert_eq!(true, is_match(&t1, &vv, 0, 0));
        assert_eq!(true, is_match(&t1, &vv, 1, 1));
        assert_eq!(false, is_match(&t1, &vv, 0, 1));
        assert_eq!(false, is_match(&t1, &vv, 1, 0));
    }

    #[test]
    fn aoc21_test_rotate_2() {
        let t1 : Vec<Vec<bool>> = vec![
            vec!{ true, false },
            vec!{ false, true },
        ];

        let t2 : Vec<Vec<bool>> = vec![
            vec!{ false, true },
            vec!{ true, false },
        ];

        let r1 = rotate(&t1);

        assert_eq!(true, is_match(&r1, &t2, 0, 0));
    }

    #[test]
    fn aoc21_rotate_3() {
        let t1 : Vec<Vec<bool>> = vec![
            vec!{ true, false, false},
            vec!{ false, true, false },
            vec!{ false, false, true },
        ];

        let t1_r : Vec<Vec<bool>> = vec![
            vec!{ false, false, true },
            vec!{ false, true, false },
            vec!{ true, false, false },
        ];

        let r1 = rotate(&t1);

        assert_eq!(true, is_match(&r1, &t1_r, 0, 0));

        let r2 = rotate(&r1);

        let t3 : Vec<Vec<bool>> = vec![
            vec!{ true, false, false },
            vec!{ false, true, false },
            vec!{ false, false, true },
        ];

        assert_eq!(true, is_match(&r2, &t3, 0, 0));

        let t4 = vec![
            vec!{ false, true, false},
            vec!{ false, false, true },
            vec!{ true, true, true },
        ];

        let t4_r = vec![
            vec!{ false, true, true},
            vec!{ true, false, true },
            vec!{ false, false, true },
        ];

        assert_eq!(true, is_match(&t4_r, &rotate(&t4), 0, 0));
    }

    #[test]
    fn aoc21_test_flip() {
        let t1 : Vec<Vec<bool>> = vec![
            vec!{ true, false, false},
            vec!{ false, true, false },
            vec!{ false, false, true },
        ];

        let t1_f: Vec<Vec<bool>> = vec![
            vec!{ false, false, true},
            vec!{ false, true, false },
            vec!{ true, false, false },
        ];
        
        assert_eq!(true, is_match(&flip(&t1), &t1_f, 0, 0));

        let t2 : Vec<Vec<bool>> = vec![
            vec!{ true, false },
            vec!{ true, true },
        ];

        let t2_f : Vec<Vec<bool>> = vec![
            vec!{ false, true },
            vec!{ true, true },
        ];


        assert_eq!(true, is_match(&flip(&t2), &t2_f, 0, 0));
    }

    #[test]
    fn aoc21_1() {
        let input = r"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        let rules : Vec<Rule> = input.lines().map(parse_rule).collect();

        for rule in &rules {
            for s in &rule.sources {
                print_map(s);
                println!("-----------------");
            }
        }

        assert_eq!(12, run_1(&rules, 2));
    }
}
