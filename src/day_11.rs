use std::fs;
use super::int_computer;
use std::collections::HashMap;

#[test]
fn problem_1() {
    assert_eq!(run(false), 2056);
}

#[test]
fn problem_2() {
    // Writes text -> GLBEPJZP
    assert_eq!(run(true), 0);
}

enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North
        }
    }

    fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North
        }
    }
}

struct Robot {
    x: isize,
    y: isize,
    direction: Direction,
    painted_cells: HashMap<(isize, isize), isize>
}

impl Robot {
    fn paint_command(&mut self, colour: isize) {
        self.painted_cells.insert((self.x, self.y), colour);
    }

    fn move_command(&mut self, command: isize) {
        self.direction = match command {
            0 => self.direction.left(),
            _ => self.direction.right()
        };

        self.move_forward();
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1
        }
    }

    fn read(&self) -> isize {
        *self.painted_cells.get(&(self.x, self.y)).unwrap_or(&0)
    }
}

const BLACK: char = '#';
const WHITE: char = '.';

pub fn run(part_two: bool) -> usize {

    let program = fs::read_to_string("input/11.txt")
        .expect("Failed to read from file")
        .split(',').map(|x| x.parse::<isize>().unwrap()).collect();

    let mut robot = Robot{x: 0, y: 0, direction: Direction::North, painted_cells: HashMap::new()};
    let mut computer = int_computer::Computer::new(program);

    if part_two {
        computer.input(1);
    }

    loop {
        let action = computer.run();
        match action {
            int_computer::Action::Read => computer.input(robot.read()),
            int_computer::Action::Output(val) => {
                robot.paint_command(val);
                if let int_computer::Action::Output(val2) = computer.run() {
                    robot.move_command(val2);
                }
            }
            int_computer::Action::Halt => break,
            _ => {}
        }
    }

    if part_two {

        let x_min = *robot.painted_cells.keys().map(|(x, _y)| x).min().unwrap();
        let x_max = *robot.painted_cells.keys().map(|(x, _y)| x).max().unwrap();
        let y_min = *robot.painted_cells.keys().map(|(_x, y)| y).min().unwrap();
        let y_max = *robot.painted_cells.keys().map(|(_x, y)| y).max().unwrap();

        for y in (y_min..=y_max).rev() {
            for x in x_min..=x_max {
                print!("{}", match robot.painted_cells.get(&(x, y)).unwrap_or(&0) {
                    0 => ' ',
                    1 => 'X',
                    _ => unimplemented!()
                });
            }
            println!();
        }

        0

    } else {
        robot.painted_cells.keys().count()
    }
}