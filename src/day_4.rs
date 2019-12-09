
#[test]
fn problem_1() {
    assert_eq!(run(false), 1686);
}

#[test]
fn problem_2() {
    assert_eq!(run(true), 1145);
}


pub fn run(part_two: bool) -> usize {

    let min = 168630;
    let max = 718098;

    let mut num_valid = 0;

    let mut num = min;

    while num <= max {
        let mut chars: Vec<char> = num.to_string().chars().collect();

        let mut contains_dupe = false;
        let mut increasing = true;

        for i in 0..chars.len() - 1 {
            if chars[i] > chars[i+1] {
                increasing = false;

                // skip to next valid increasing number
                for j in i..chars.len() {
                    chars[j] = chars[i];
                }
                num = chars.iter().collect::<String>().parse::<isize>().unwrap() - 1;
                break;
            }

            if chars[i] == chars[i+1] {
                if part_two {
                    contains_dupe |= match i {
                        0 => chars[i] != chars[i + 2],
                        4 => chars[i] != chars[i - 1],
                        _ => chars[i] != chars[i - 1] && chars[i] != chars[i + 2]
                    };
                } else {
                    contains_dupe = true;
                }
            }
        }
        num += 1;

        if increasing && contains_dupe {
            num_valid += 1;
        }
    }
    num_valid
}