pub fn part1(input: &String) -> u16 {
    let report = input.lines().map(|s: &str| s.parse::<u16>().unwrap());
    report.clone().zip(report.skip(1)).filter(|(a, b)| a < b).count() as u16
}

pub fn part2(input: &String) -> u16 {
    let report = input.lines().map(|s: &str| s.parse::<u16>().unwrap());
    report.clone().zip(report.skip(3)).filter(|(a, b)| a < b).count() as u16
}
