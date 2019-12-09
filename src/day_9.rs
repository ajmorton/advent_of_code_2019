use super::int_computer;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[test]
fn problem_1() {
    assert_eq!(run(false), 2752191671);
}

#[test]
fn problem_2() {
    assert_eq!(run(true), 87571);
}

pub fn run(part_two: bool) -> isize {
    let file = File::open("input/9.txt").expect("Error reading file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Error reading line");

    let v: Vec<&str> = line.split(',').collect();
    let prog: Vec<isize> = v.iter().map(|x| x.parse::<isize>().unwrap()).collect();

    let input = if part_two {vec!(2)} else {vec!(1)};

    let mut computer = int_computer::Computer::new_with_input(prog, input);

    let mut output = 0;
    loop {
        match computer.run() {
            int_computer::Action::Halt => break,
            int_computer::Action::Output(val) => {output = val; println!("{}", output)},
            _ => {}
        };
    }

    output
}