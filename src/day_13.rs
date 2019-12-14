use super::int_computer::{Computer, Action};
use std::collections::HashMap;
use std::{fs, thread, time};

#[test]
fn problem_1() {
    assert_eq!(run(false), 180);
}

#[test]
fn problem_2() {
    assert_eq!(run(true), 8777);
}

// enable to print game screen to console
const PRINT_TO_CONSOLE: bool = false;

type CellId = isize;

const EMPTY: CellId = 0;
const WALL: CellId = 1;
const BLOCK: CellId = 2;
const PADDLE: CellId = 3;
const BALL: CellId = 4;

enum GameState {
    End,
    AwaitingInput
}


struct Game {
    screen: HashMap<(usize, usize), CellId>,
    score: isize,
    ball_x_pos: isize,
    paddle_x_pos: isize
}

impl Game {
    fn new() -> Self {
        Self { screen: HashMap::new(), score: 0, ball_x_pos: 0, paddle_x_pos: 0}
    }

    fn set_pixel(&mut self, x: isize, y: isize, val: CellId) {
//        println!("SET ({}, {}), {}", x, y, val);
        match (x, y) {
            (-1, 0) => self.score = val,
            _ => {self.screen.insert((x as usize, y as usize), val);},
        };

        // store to speed up performance of update_paddle rather than searching screen for positions
        if val == PADDLE {
            self.paddle_x_pos = x;
        } else if val == BALL {
            self.ball_x_pos = x;
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> &CellId {
        self.screen.get(&(x, y)).unwrap_or(&EMPTY)
    }

    fn get_score(&self) -> isize {
        self.score
    }

    fn print_screen(&self) {
        let x_max = *self.screen.keys().map(|(x, _y)| x).max().unwrap();
        let y_min = *self.screen.keys().map(|(_x, y)| y).min().unwrap();
        let y_max = *self.screen.keys().map(|(_x, y)| y).max().unwrap();

        // send a control character to clear screen
        print!("{}[2J", 27 as char);

        println!("SCORE {}", self.score);

        for y in y_min..=y_max {
            for x in 0..=x_max {
                print!("{}", match *self.get_pixel(x, y) {
                    EMPTY => " ",
                    WALL => "|",
                    BLOCK => "B",
                    PADDLE => "_",
                    BALL => "o",
                    _ => unimplemented!()
                })
            }
            println!();
        }
    }

    fn update_paddle(&self) -> isize {
        // automated to keep paddle under ball

        if PRINT_TO_CONSOLE {
             let sleep_time = time::Duration::from_millis(100);
             thread::sleep(sleep_time);
        }

        if self.paddle_x_pos > self.ball_x_pos {
            -1
        } else if self.paddle_x_pos < self.ball_x_pos {
            1
        } else {
            0
        }

    }

}


fn update_screen(game: &mut Game, computer: &mut Computer) {
    loop {
        match computer.run_for(3) {
            None => break,
            Some(output) => {
                match output.len() {
                    3 => {
                        match (&output[0], &output[1], &output[2]) {
                            (&Action::Output(x), &Action::Output(y), &Action::Output(tile)) => game.set_pixel(x, y, tile),
                            _ => unimplemented!()
                        }
                    }
                    1 => if output[0] == Action::Read {
                        if PRINT_TO_CONSOLE {
                            game.print_screen();
                        }
                        computer.input(game.update_paddle())
                    }
                    _ => unimplemented!()
                }
            }
        }
    }
}

pub fn run(part_two: bool) -> isize {
    let mut program: Vec<isize> = fs::read_to_string("input/13.txt")
        .expect("Failed to read from file")
        .split(',').map(|x| x.parse::<isize>().unwrap()).collect();

    if part_two {
        program[0] = 2;
    }

    let mut computer = Computer::new(program);
    let mut game = Game::new();


    update_screen(&mut game, &mut computer);

    if part_two {
        game.get_score()
    } else {
        game.screen.values().filter(|tile| tile == &&BLOCK).count() as isize
    }

}