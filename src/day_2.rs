use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

use super::int_computer;

pub fn run(part_two: bool) -> isize {
    let file = File::open("input/2.txt").expect("Error reading file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Error reading line");

    let v: Vec<&str> = line.split(',').collect();
    let prog: Vec<isize> = v.iter().map(|x| x.parse::<isize>().unwrap()).collect();

    if part_two {
        for i in 0..99 {
            for j in 0..99 {
                println!("Trying {} {}", i, j);
                if run_prog(&prog, i, j) == 19690720 {
                    return 100 * i + j;
                }
            }
        }
        0
    } else {
        run_prog(&prog, 12, 2)
    }

}

fn run_prog(prog: &Vec<isize>, noun: isize, verb: isize) -> isize {

    let mut prog = prog.clone();
    prog[1] = noun;
    prog[2] = verb;

    let mut computer = int_computer::Computer::new(prog);
    computer.run();
    return computer.get_program()[0];
}