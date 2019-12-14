use std::fs;
use std::cmp::Ordering;

#[test]
fn problem_1() {
    assert_eq!(run(false), 6220);
}

#[derive(Debug, Clone, Copy)]
struct Pos{x: isize, y: isize, z: isize}

impl Pos {
    fn get_potential_energy(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

#[derive(Debug, Copy, Clone)]
struct Vel{x: isize, y: isize, z: isize}

impl Vel {
    fn add(&self, other: &Self) -> Self {
        Vel{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }

    fn get_kinetic_energy(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

#[derive(Debug)]
struct Moon {
    pos: Pos,
    vel: Vel
}

impl Moon {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Moon{pos: Pos{x, y, z}, vel: Vel{x:0, y:0, z:0}}
    }

    fn get_gravity(&self, other: &Self) -> Vel {
        let cmp = |x1: isize, x2: isize| match &x1.partial_cmp(&x2).unwrap() {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1
        };

        Vel{x: cmp(self.pos.x, other.pos.x), y: cmp(self.pos.y, other.pos.y), z: cmp(self.pos.z, other.pos.z)}
    }

    fn update_vel(&self, gravity: Vel) -> Self {
        Moon {
            pos: self.pos,
            vel: Vel {x: self.vel.x + gravity.x, y: self.vel.y + gravity.y, z: self.vel.z + gravity.z}
        }
    }

    fn update_pos(&self) -> Self {
        Moon{
            pos: Pos { x: self.pos.x + self.vel.x,
                       y: self.pos.y + self.vel.y,
                       z: self.pos.z + self.vel.z},
            vel: self.vel
        }
    }

    fn get_energy(&self) -> usize {
        self.pos.get_potential_energy() * self.vel.get_kinetic_energy()
    }
}

pub fn run(part_two: bool) -> usize {
    let input = fs::read_to_string("input/12.txt").expect("Failed to read file");
    let mut moons: Vec<Moon> = input.lines().map(
        |line| {
            let (x,y, z) = scan_fmt!(line, "<x={}, y={}, z={}>", isize, isize, isize).unwrap();
            Moon::new(x,y,z)
        }
    ).collect();

    for _time_step in 1..=1000 {
        let gravities: Vec<Vel> = moons.iter().map(
            |moon| moons.iter().map(|m| moon.get_gravity(m))
                .fold(Vel{x:0, y:0, z:0}, |v1, v2| v1.add(&v2))).collect();

        moons = moons.iter().zip(gravities).map(|(moon, grav)| moon.update_vel(grav).update_pos()).collect();
    }

    moons.iter().map(|moon| moon.get_energy()).sum()


}