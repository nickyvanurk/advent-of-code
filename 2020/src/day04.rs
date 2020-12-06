use regex::Regex;

pub fn part1(input: String) {
    let input: Vec<&str> = input.lines().collect();
    let mut passports: Vec<Vec<&str>> = vec![vec![]];

    for (i, line) in input.iter().enumerate() {
        if !line.is_empty() {
            passports.last_mut().unwrap().extend(
                line.split(|l| vec![':', ' '].contains(&l))
                    .collect::<Vec<&str>>(),
            );
        }

        let next = match input.get(i + 1) {
            Some(value) => value,
            None => continue,
        };

        if next.is_empty() {
            passports.push(vec![]);
        }
    }

    println!(
        "{}",
        passports
            .iter()
            .filter(|p| {
                p.contains(&"byr")
                    && p.contains(&"iyr")
                    && p.contains(&"eyr")
                    && p.contains(&"hgt")
                    && p.contains(&"hcl")
                    && p.contains(&"ecl")
                    && p.contains(&"pid")
            })
            .count()
    );
}

pub fn part2(input: String) {
    println!(
        "{}",
        input
            .trim()
            .split("\n\n")
            .filter(|&p| {
                return if !Regex::new(r"byr:(19[2-9][0-9]|200[0-2])")
                    .unwrap()
                    .is_match(p)
                {
                    false
                } else if !Regex::new(r"iyr:(201[0-9]|2020)").unwrap().is_match(p) {
                    false
                } else if !Regex::new(r"eyr:(202[0-9]|2030)").unwrap().is_match(p) {
                    false
                } else if !Regex::new(r"hgt:(((1[5-8][0-9]|19[0-3])cm)|((59|6[0-9]|7[0-6])in))")
                    .unwrap()
                    .is_match(p)
                {
                    false
                } else if !Regex::new(r"hcl:#[0-9a-f]{6}").unwrap().is_match(p) {
                    false
                } else if !Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)")
                    .unwrap()
                    .is_match(p)
                {
                    false
                } else if !Regex::new(r"pid:\d{9}(\s|[a-z]|$)").unwrap().is_match(p) {
                    false
                } else {
                    true
                };
            })
            .count()
    );
}
