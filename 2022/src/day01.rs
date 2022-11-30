use itertools::Itertools;

pub fn part1(input: &String) -> u16 {
    let report = input.lines().map(|s| s.parse::<u16>().unwrap());
    report.tuple_windows().filter(|(a, b)| a < b).count() as u16
}

pub fn part2(input: &String) -> u16 {
    let report = input.lines().map(|s| s.parse::<u16>().unwrap());
    report.tuple_windows().filter(|(a, _, _, d)| a < d).count() as u16
}
