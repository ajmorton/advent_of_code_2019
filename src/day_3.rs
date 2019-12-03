use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn run(part_two: bool) -> usize {
    let file = File::open("input/3.txt").expect("Error reading file");
    let mut reader = BufReader::new(file);

    let mut line1 = String::new();
    let mut line2 = String::new();
    reader.read_line(&mut line1).expect("Error reading line");
    reader.read_line(&mut line2).expect("Error reading line");

    let wire1 = get_cells(&line1.trim());
    let wire2 = get_cells(&line2.trim());

    let set1: HashSet<&(isize, isize)> = HashSet::from_iter(wire1.iter());
    let set2: HashSet<&(isize, isize)> = HashSet::from_iter(wire2.iter());
    let intersections = set1.intersection(&set2);

    let mut closest = usize::max_value();

    for val in intersections {

        let dist = if part_two {
            wire1.iter().position(|x| x == *val).unwrap() + 1 +
                wire2.iter().position(|x| x == *val).unwrap() + 1
        } else {
            let manhattan = val.0.abs() + val.1.abs();
            // gross
            let mut x: usize = 0;
            for _ in 0..manhattan {
                x += 1;
            }
            x
        };

        if dist < closest {
            closest = dist;
        }
    }
    closest
}

fn get_cells(string: &str) -> Vec<(isize, isize)> {
    let mut cells: Vec<(isize, isize)> = Vec::new();
    let commands: Vec<&str> = string.split(",").collect();

    let mut pos: (isize, isize) = (0, 0);

    for command in commands {
        let (dir, length) = command.split_at(1);
        for _ in 0..length.parse::<usize>().unwrap() {
            pos = match dir {
                "U" => (pos.0,     pos.1 + 1),
                "D" => (pos.0,     pos.1 - 1),
                "L" => (pos.0 - 1, pos.1    ),
                "R" => (pos.0 + 1, pos.1    ),
                _ => pos,
            };
            cells.push(pos);
        }
    }
    cells
}
