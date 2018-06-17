use std::ops::{Add, AddAssign};
use regex::Regex;
use std::fs;

#[derive(Debug, Clone, PartialEq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Vec3 {
            x: x, 
            y: y,
            z: z,
        }
    }

    fn mul(&self, m: i64) -> Self {
        Vec3::new(self.x * m, self.y * m, self.z * m)
    }

    fn div(&self, m: i64) -> Self {
        Vec3::new(self.x / m, self.y / m, self.z / m)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, 
                  self.y + rhs.y,
                  self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Particle {
    fn new(p: &Vec3, v: &Vec3, a: &Vec3) -> Self {
        Particle {
            pos: p.clone(),
            vel: v.clone(),
            acc: a.clone(),
        }
    }

    fn update(&mut self) {
        self.vel += self.acc.clone();
        self.pos += self.vel.clone();
    }

    fn update_steps(&mut self, steps: i64) {
        self.pos += self.vel.mul(steps) + self.acc.mul(steps*steps).div(2);
        self.vel += self.acc.mul(steps);
    }

    fn extreme(&self) -> i64 {
        -(self.vel.x + self.vel.y + self.vel.z) / (self.acc.x + self.acc.y + self.acc.z)
    }

    fn dist(&self) -> i64 {
        i64::abs(self.pos.x) + i64::abs(self.pos.y) + i64::abs(self.pos.z)
    }
}

fn s2i(s: &str) -> i64 {
    i64::from_str_radix(s, 10).unwrap()
}

fn parse(row: &str) -> Particle {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>"#).unwrap();
    }

    let c = RE.captures(row).unwrap();

    let p = Vec3::new(s2i(&c[1]), s2i(&c[2]), s2i(&c[3]));
    let v = Vec3::new(s2i(&c[4]), s2i(&c[5]), s2i(&c[6]));
    let a = Vec3::new(s2i(&c[7]), s2i(&c[8]), s2i(&c[9]));

    Particle::new(&p, &v, &a)
}

fn run_1(particles: &mut Vec<Particle>) -> usize {
    let mut dists: Vec<i64> = vec![];
    for p in particles.iter() {
        dists.push(p.dist());
    }

    let iter = 10000000;
    for i in 0..iter {
        if 0 == i % 1000 {
            println!("{}", iter - i);
        }
        for(i,  p) in particles.iter_mut().enumerate() {
            p.update();
            let d = p.dist();
            if d < dists[i] {
                dists[i] = d;
            }

        }
    }

    let mut min = particles[0].dist();
    let mut min_idx = 0;
    for(i,  p) in particles.iter().enumerate() {
        if p.dist() < min {
            min = p.dist();
            min_idx = i;
        }
    }
    min_idx
}

fn run_2(particles: &mut Vec<Particle>) -> usize {
    let iter = 100000;
    for idx in 0..iter {
        if 0 == idx % 1000 {
            println!("{} - {}", iter - idx, particles.len());
        }

        for p in particles.iter_mut() {
            p.update();
        }

        let mut i = 0;
        while i < particles.len() {
            let mut d = false;
            for j in (i+1..particles.len()).rev() {
                if particles[i].pos == particles[j].pos {
                    println!("remove {}", j);
                   particles.remove(j);
                   d = true;
                }
            }

            if d {
                particles.remove(i);
            }
            else {
                i += 1;
            }
        }
    }

    particles.len()
    //506 too high
}

pub fn run() {
    let input = fs::read_to_string("day20.txt").unwrap();
    // let mut particles : Vec<Particle> = input.lines().map(parse).collect();
    // println!("aoc20-1: {}", run_1(&mut particles));
    let mut particles : Vec<Particle> = input.lines().map(parse).collect();
    println!("aoc20-2: {}", run_2(&mut particles));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc20_add() {
        assert_eq!(Vec3::new(1,2,3), Vec3::new(0,0,0) + Vec3::new(1,2,3));
    }

    #[test]
    fn aoc20_parse() {
        assert_eq!(Particle::new(
                &Vec3::new(-10088, 3682, -5210),
                &Vec3::new(52, 32, -38),
                &Vec3::new(14, -8, 11)
                ), parse("p=<-10088,3682,-5210>, v=<52,32,-38>, a=<14,-8,11>"));
    }

    #[test]
    fn aoc20_derived() {
        let mut p = Particle::new(
                &Vec3::new(-10088, 3682, -5210),
                &Vec3::new(52, 32, -38),
                &Vec3::new(14, -8, 11)
                );

        let e = p.extreme();

        println!("p extreme: {}", e);

        // assert_eq!(p, p2);
    }

    #[test]
    fn aoc20_2() {
        let input = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\np=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\np=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\np=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";

        let mut particles : Vec<Particle> = input.lines().map(parse).collect();
        assert_eq!(1, run_2(&mut particles));
    }
}
