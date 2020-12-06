pub fn part1(input: String) {
    let valid_passwords = input
        .lines()
        .filter(|s| {
            let v: Vec<&str> = s.split(|c| vec!['-', ':', ' '].contains(&c)).collect();
            let min = v[0].parse::<u8>().unwrap();
            let max = v[1].parse::<u8>().unwrap();
            let letter_occurrences = v[4].matches(v[2]).count() as u8;

            min <= letter_occurrences && letter_occurrences <= max
        })
        .count();

    println!("{}", valid_passwords);
}

pub fn part2(input: String) {
    let valid_passwords = input
        .lines()
        .filter(|s| {
            let v: Vec<&str> = s.split(|c| vec!['-', ':', ' '].contains(&c)).collect();
            let pos1 = v[0].parse::<usize>().unwrap();
            let pos2 = v[1].parse::<usize>().unwrap();
            let pos1_has_letter = v[4][pos1 - 1..pos1] == *v[2];
            let pos2_has_letter = v[4][pos2 - 1..pos2] == *v[2];

            pos1_has_letter ^ pos2_has_letter
        })
        .count();

    println!("{}", valid_passwords);
}
