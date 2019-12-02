#![allow(dead_code)]
mod day_1;
mod day_2;

fn main() {
    println!("{}", day_2::run(true));
}


#[test]
fn test_day_1() { 
    assert_eq!(day_1::run(false), 3125750); 
    assert_eq!(day_1::run(true), 4685788);
}

#[test]
fn test_day_2() { 
    assert_eq!(day_2::run(false), 4930687); 
    assert_eq!(day_2::run(true), 5335);
}
