mod int_computer;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

fn main() {
    println!("{}", day_5::run(true));
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

#[test]
fn test_day_3() {
    assert_eq!(day_3::run(false), 1264);
    assert_eq!(day_3::run(true), 37390);
}

#[test]
fn test_day_4() {
    assert_eq!(day_4::run(false), 1686);
    assert_eq!(day_4::run(true), 1145);
}

#[test]
fn test_day_5() {
    assert_eq!(day_5::run(false), 7566643);
    assert_eq!(day_5::run(true), 9265694);
}
