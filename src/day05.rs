pub fn part1(input: String) {
    println!("{}", get_seat_ids(&input).iter().max().unwrap());
}

pub fn part2(input: String) {
    let seat_ids = get_seat_ids(&input);
    let n = seat_ids.iter().max().unwrap() + seat_ids.iter().min().unwrap();
    let sum = seat_ids.iter().sum::<usize>();

    println!("{}", n * (seat_ids.len() + 1) / 2 - sum); // Gauss sum
}

fn get_seat_ids(input: &String) -> Vec<usize> {
    input
        .lines()
        .map(|p| (encode(&p) >> 3) * 8 + (encode(&p) & 7))
        .collect()
}

fn encode(passport: &str) -> usize {
    usize::from_str_radix(
        &passport
            .chars()
            .map(|c| if "BR".contains(c) { "1" } else { "0" })
            .collect::<String>(),
        2,
    )
    .unwrap()
}
