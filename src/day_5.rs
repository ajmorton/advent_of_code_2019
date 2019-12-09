use super::int_computer;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::int_computer::Action;


#[test]
fn problem_1() {
    assert_eq!(run(false), 7566643);
}

#[test]
fn problem_2() {
    assert_eq!(run(true), 9265694);
}


pub fn run(part_two: bool) -> isize {

    let file = File::open("input/5.txt").expect("Error reading file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Error reading line");

    let v: Vec<&str> = line.split(',').collect();
    let prog: Vec<isize> = v.iter().map(|x| x.parse::<isize>().unwrap()).collect();

    let mut computer = int_computer::Computer::new(prog);

    if part_two {
        computer.input(5);
        while computer.run() != Action::Halt { }

        // return diagnostic code
        return computer.get_output_vals().pop().unwrap();
    } else {
        computer.input(1);
        while computer.run() != Action::Halt { }

        // return diagnostic code
        return computer.get_output_vals().pop().unwrap();
    }
}