
pub fn run(part_two: bool) -> usize {

    let min = 168630;
    let max = 718098;

    let mut num_valid = 0;

    for num in min..=max {
        let chars: Vec<char> = num.to_string().chars().collect();

        let mut contains_dupe = false;
        let mut increasing = true;

        for i in 0..chars.len() - 1 {
            if chars[i] > chars[i+1] {
                increasing = false;
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

        if increasing && contains_dupe {
            num_valid += 1;
        }
    }
    num_valid
}