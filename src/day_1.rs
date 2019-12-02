#![allow(dead_code)]

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn run(part_two: bool) -> isize {
    let file = File::open("input/1.txt").expect("Error reading file");
    let reader = BufReader::new(file);

    let weights = reader.lines().map(|x| x.unwrap().parse::<isize>().unwrap());

    let sum = if part_two {
        weights.fold(0isize, |sum, val| sum + compute_fuel_part_two(val))
    } else {
        weights.fold(0isize, |sum, val| sum + compute_fuel_part_one(val))
    };
    sum
}

fn compute_fuel_part_one(mass: isize) -> isize {
    mass / 3 - 2
}

fn compute_fuel_part_two(mass: isize) -> isize {
    let fuel = mass / 3 - 2;
    match fuel {
        x if x <= 0 => 0,
        _ => fuel + compute_fuel_part_two(fuel)
    }
}

