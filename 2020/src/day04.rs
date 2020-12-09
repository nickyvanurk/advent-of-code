use regex::Regex;

pub fn part1(input: &String) -> u8 {
    let required_fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let valid_passports = input
        .split("\n\n")
        .filter(|&passport| required_fields.iter().all(|field| passport.contains(field)))
        .count();

    valid_passports as u8
}

pub fn part2(input: &String) -> u8 {
    let validation_rules = vec![
        Regex::new(r"byr:(19[2-9][0-9]|200[0-2])").unwrap(),
        Regex::new(r"iyr:(201[0-9]|2020)").unwrap(),
        Regex::new(r"eyr:(202[0-9]|2030)").unwrap(),
        Regex::new(r"hgt:(((1[5-8][0-9]|19[0-3])cm)|((59|6[0-9]|7[0-6])in))").unwrap(),
        Regex::new(r"hcl:#[0-9a-f]{6}").unwrap(),
        Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)").unwrap(),
        Regex::new(r"pid:\d{9}(\s|[a-z]|$)").unwrap(),
    ];

    let valid_passports = input
        .split("\n\n")
        .filter(|&passport| validation_rules.iter().all(|re| re.is_match(passport)))
        .count();

    valid_passports as u8
}
