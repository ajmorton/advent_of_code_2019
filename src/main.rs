#![allow(dead_code)]

#[macro_use] extern crate scan_fmt;

mod int_computer;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;

fn main() {
    println!("{}", day_12::run(false));
}
