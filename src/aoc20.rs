use std::fs::File;
use std::io::prelude::*;
use std::ops::{Add, AddAssign};

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

#[derive(Debug)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Particle {
    fn update(&mut self) {
        self.vel += self.acc.clone();
        self.pos += self.vel.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc20_add() {
        assert_eq!(Vec3::new(1,2,3), Vec3::new(0,0,0) + Vec3::new(1,2,3));
    }
}
