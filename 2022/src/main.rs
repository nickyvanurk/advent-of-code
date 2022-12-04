use std::env;
use std::io;

use advent_of_code::*;

macro_rules! run_day {
    ($day:path, $input:expr) => {{
        use $day::*;
        println!(
            "{}: part1 = {:?}, part2 = {:?}",
            stringify!($day),
            part1($input),
            part2($input)
        );
    }};
}

fn main() {
    // Get day string
    let args: Vec<String> = env::args().collect();
    let mut day = String::new();

    if args.len() >= 2 {
        day = args[1].clone();
    } else {
        println!("Enter day: ");
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read line");
    }

    // Parse day as number
    day = day.trim().to_string();
    let day_num: u8 = match day.parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid day number: {}", day);
            return;
        }
    };

    let input = read_input(day_num);

    match day_num {
        1 => run_day!(day01, input.trim()),
        2 => run_day!(day02, input.trim()),
        3 => run_day!(day03, input.trim()),
        4 => run_day!(day04, input.trim()),
        // 5 => run_day!(day05, input.trim()),
        // 6 => run_day!(day06, input.trim()),
        // 7 => run_day!(day07, input.trim()),
        // 8 => run_day!(day08, input.trim()),
        // 9 => run_day!(day09, input.trim()),
        // 10 => run_day!(day10, input.trim()),
        // 11 => run_day!(day11, input.trim()),
        // 12 => run_day!(day12, input.trim()),
        // 13 => run_day!(day13, input.trim()),
        // 14 => run_day!(day14, input.trim()),
        // 15 => run_day!(day15, input.trim()),
        // 16 => run_day!(day16, input.trim()),
        // 17 => run_day!(day17, input.trim()),
        // 18 => run_day!(day18, input.trim()),
        // 19 => run_day!(day19, input.trim()),
        // 20 => run_day!(day20, input.trim()),
        // 21 => run_day!(day21, input.trim()),
        // 22 => run_day!(day22, input.trim()),
        // 23 => run_day!(day23, input.trim()),
        _ => println!("Invalid day number: {}", day_num),
    }
}
