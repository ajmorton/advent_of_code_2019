use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub fn run(part_two: bool) -> usize {
    let file = File::open("input/2.txt").expect("Error reading file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Error reading line");

    let v: Vec<&str> = line.split(',').collect();
    let prog: Vec<usize> = v.iter().map(|x| x.parse::<usize>().unwrap()).collect();

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

fn run_prog(prog: &Vec<usize>, noun: usize, verb: usize) -> usize {

    let mut prog = prog.clone();
    prog[1] = noun;
    prog[2] = verb;    

    const ADD: usize = 1;
    const MULT: usize = 2;
    const END: usize = 99;

    let mut index = 0;
    while prog[index] != END {
        let val_1 = prog[prog[index + 1]];
        let val_2 = prog[prog[index + 2]];
        let dest = prog[index + 3];

        prog[dest] = match prog[index] {
            ADD  => val_1 + val_2,
            MULT => val_1 * val_2,
            _ => 0
        };
        index += 4;
    }
    prog[0]
}