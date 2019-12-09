use super::int_computer;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[test]
fn problem_1() {
    assert_eq!(run(false), 440880);
}

#[test]
fn problem_2() {
    assert_eq!(run(true), 3745599);
}

pub fn run(part_two: bool) -> isize {

    let file = File::open("input/7.txt").expect("Error reading file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Error reading line");

    let v: Vec<&str> = line.split(',').collect();
    let prog: Vec<isize> = v.iter().map(|x| x.parse::<isize>().unwrap()).collect();

    let phase_settings = if part_two {vec!(5,6,7,8,9)} else {vec!(0,1,2,3,4)};


    let mut max_output = 0;
    for perm in generate_permutations(phase_settings) {
        let mut array = Vec::new();
        for phase_setting in &perm {
            array.push(int_computer::Computer::new_with_input(prog.clone(), vec!(*phase_setting)));
        }

        let mut input = 0;
        let mut action = int_computer::Action::Continue;
        let mut comp_index = 0;

        while !(comp_index == 0 && action == int_computer::Action::Halt) {
            array[comp_index].input(input);
            action = array[comp_index].run();
            match action {
                int_computer::Action::Output(val) => input = val,
                _ => {}
            };
            comp_index = (comp_index + 1) % array.len();
        }
        max_output = max_output.max(input);
    }
    max_output

}

fn generate_permutations(inputs: Vec<isize>) -> Vec<Vec<isize>> {

    if inputs.len() == 1 {
        return vec!(inputs);
    }

    let mut perms = Vec::new();

    for i in 0..inputs.len() {
        let mut perm: Vec<isize> = inputs.clone();
        let val = perm.remove(i);

        let mut ps = generate_permutations(perm.clone());
        ps = ps.iter().map(|x| {
            let mut y = x.clone();
            y.push(val);
            y
        }).collect();
        perms.append(&mut ps);
    }

    perms
}

#[test]
fn perms() {
    let expected = vec!(vec!(3, 2, 1), vec!(2, 3, 1), vec!(3, 1, 2), vec!(1, 3, 2), vec!(2, 1, 3), vec!(1, 2, 3));
    let result = generate_permutations(vec!(1,2,3));
    assert_eq!(expected, result );
}