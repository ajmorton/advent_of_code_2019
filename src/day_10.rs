use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use num::integer;

//#[test]
//fn problem_1() {
//    assert_eq!(run(false), "1950");
//}

//#[test]
//fn problem_2() {
//
//}

const ASTEROID: char = '#';
const EMPTY: char = '.';

type Map = Vec<Vec<char>>;

#[derive(PartialEq, Debug)]
struct Coord {
    x: usize,
    y: usize
}

impl Coord {
    fn coord(&self, offset: &Offset) -> Coord {
        Coord{x: (self.x as isize + offset.x) as usize, y: (self.y as isize + offset.y) as usize}
    }

    fn offset(&self, other: &Self) -> Offset {
        let (x1, y1) = (self.x as isize, self.y as isize);
        let (x2, y2) = (other.x as isize, other.y as isize);
        Offset{x: x2 - x1, y: y2 - y1}
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Offset {
    x: isize,
    y: isize
}

impl Offset {
    fn add(&self, other: &Self) -> Self {
        Offset{x: self.x + other.x, y: self.y + other.y}
    }
}

fn get_cell(base: &Coord, offset: &Offset, map: &Map) -> char {
    let coord = base.coord(offset);
    *map.get(coord.y).unwrap().get(coord.x).unwrap()
}

fn get_minjump(offset: &Offset) -> Offset {
    let gcd = integer::gcd(offset.x, offset.y);
    Offset{x: offset.x / gcd, y: offset.y / gcd}
}

fn is_visible(center: &Coord, other: &Coord, map: &Map) -> bool {
    let mut blocked = false;
    let offset = center.offset(other);

    let minjump = get_minjump(&offset);
    let mut jump = minjump.clone();

    while &jump != &offset {
        if get_cell(center, &jump, &map) == ASTEROID {
            blocked = true;
            break;
        }
        jump = jump.add(&minjump);
    }

    !blocked
}

fn count_visible(asteroid_loc: &Coord, map: &Map) -> usize {
    let (height, width) = (map.len(), map.get(0).unwrap().len());

    let mut num_visible = 0;

    for y in 0..height {
        for x in 0..width {
            let coord = &Coord {x, y};
            if coord != asteroid_loc && get_cell(coord, &Offset{x:0, y:0}, &map) == ASTEROID {
                if is_visible(&asteroid_loc, coord, map) {
                    num_visible += 1;
                }
            }
        }
    }
    num_visible
}

pub fn run(part_two: bool) -> usize {
    let file = File::open("input/10.txt").expect("Error reading file");
    let reader = BufReader::new(file);

    let map: Map = reader.lines().map(|line| line.unwrap().chars().collect::<Vec<char>>()).collect();

    let mut max_vis = 0;

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == ASTEROID {
                let asteroid_loc = Coord{x, y};
                let vis = count_visible(&asteroid_loc, &map);
                if vis > max_vis {
                    max_vis = vis;
                }
            }
        }
    }

    if part_two {
        0
    } else {
        max_vis
    }
}