pub fn part1(input: String) {
    let passwords: Vec<Vec<&str>> = input
        .trim()
        .split('\n')
        .map(|s: &str| s.split(':').collect())
        .collect();

    let mut valid_passwords = 0;

    for entry in passwords {
        let policy = entry[0];
        let password = entry[1];

        let (min, max, letter) = match &policy.split('-').collect::<Vec<&str>>()[..] {
            &[min, max] => (
                min.parse::<i32>().unwrap(),
                max.split(' ').collect::<Vec<&str>>()[0]
                    .parse::<i32>()
                    .unwrap(),
                max.split(' ').collect::<Vec<&str>>()[1],
            ),
            _ => unreachable!(),
        };

        let letter_count = password.matches(letter).count() as i32;

        if min <= letter_count && letter_count <= max {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords);
}

pub fn part2(input: String) {
    let passwords: Vec<Vec<&str>> = input
        .trim()
        .split('\n')
        .map(|s: &str| s.split(':').collect())
        .collect();

    let mut valid_passwords = 0;

    for entry in passwords {
        let policy = entry[0];
        let password = entry[1];

        let (pos1, pos2, letter) = match &policy.split('-').collect::<Vec<&str>>()[..] {
            &[pos1, pos2] => (
                pos1.parse::<i32>().unwrap(),
                pos2.split(' ').collect::<Vec<&str>>()[0]
                    .parse::<i32>()
                    .unwrap(),
                pos2.split(' ').collect::<Vec<&str>>()[1]
                    .chars()
                    .nth(0)
                    .unwrap(),
            ),
            _ => unreachable!(),
        };

        let pos1_has_letter = password.chars().nth(pos1 as usize).unwrap() == letter;
        let pos2_has_letter = password.chars().nth(pos2 as usize).unwrap() == letter;

        if (pos1_has_letter != pos2_has_letter) && (pos1_has_letter || pos2_has_letter) {
            valid_passwords += 1;
        }
    }

    println!("{}", valid_passwords);
}
