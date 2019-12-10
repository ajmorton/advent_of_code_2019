use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[test]
fn problem_1() {
    assert_eq!(run(false), "1950");
}

#[test]
fn problem_2() {
    // spells FKAHL
    assert_eq!(run(true), "111101001001100100101000010000101001001010010100001110011000100101111010000100001010011110100101000010000101001001010010100001000010010100101001011110");
}

const SIZE: usize = 25*6;

pub fn run(part_two: bool) -> String {

    let file = File::open("input/8.txt").expect("Error reading file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Error reading line");

    let chars: Vec<char> = line.chars().collect();

    if part_two {
        let mut picture = vec!['0'; SIZE];
        for i in 0..SIZE {
            for layer in 0..chars.len() / SIZE {
                if chars[i + layer*SIZE] != '2' {
                    picture[i] = chars[i + layer*SIZE];
                    break;
                }
            }
        }

        for i in 0..picture.len() {
            if i % 25 == 0 { println!(); }
            print!("{}", if picture[i] == '1' {"X"} else {" "});
        }
        println!();
        picture.iter().collect::<String>()

    } else {
        let mut min_zeros = usize::max_value();
        let mut ones_times_twos = 0;

        for layer in 0..chars.len() / SIZE {
            println!("layer {}", layer);

            let mut count = [0, 0, 0];
            for i in 0..SIZE {
                match chars[SIZE * layer + i] {
                    '0' => count[0] += 1,
                    '1' => count[1] += 1,
                    '2' => count[2] += 1,
                    _ => unimplemented!()
                };
            }

            if count[0] < min_zeros {
                min_zeros = count[0];
                ones_times_twos = count[1] * count[2];
            }
        }

        ones_times_twos.to_string()
    }
}
