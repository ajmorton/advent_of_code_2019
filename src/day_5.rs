use super::int_computer;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn run(part_two: bool) -> isize {

    let file = File::open("input/5.txt").expect("Error reading file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Error reading line");

    let v: Vec<&str> = line.split(',').collect();
    let prog: Vec<isize> = v.iter().map(|x| x.parse::<isize>().unwrap()).collect();



    if part_two {
        let mut computer = int_computer::Computer::new_with_input(prog, vec!(5));
        computer.run();
        // return diagnostic code
        return computer.get_output_vals().pop().unwrap();
    } else {
        let mut computer = int_computer::Computer::new_with_input(prog, vec!(1));
        computer.run();
        // return diagnostic code
        return computer.get_output_vals().pop().unwrap();
    }
}