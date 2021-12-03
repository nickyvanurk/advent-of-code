use std::env;
use std::fs;

pub mod day01;
pub mod day02;
pub mod day03;

pub fn read_input(day_num: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("inputs").join(format!("day{:02}.txt", day_num));
    fs::read_to_string(filename).expect("Error while reading")
}
