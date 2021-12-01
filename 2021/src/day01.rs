pub fn part1(input: &String) -> u32 {
    let report = input.lines().map(|s: &str| s.parse::<u32>().unwrap());
    report.clone().zip(report.skip(1)).filter(|(a, b)| a < b).count() as u32
}

pub fn part2(input: &String) -> u32 {
    let report = input.lines().map(|s: &str| s.parse::<u32>().unwrap());
    report.clone().zip(report.skip(3)).filter(|(a, b)| a < b).count() as u32
}
