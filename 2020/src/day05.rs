pub fn part1(input: &String) -> u32 {
    *get_seat_ids(&input).iter().max().unwrap() as u32
}

pub fn part2(input: &String) -> u32 {
    let seat_ids = get_seat_ids(&input);
    let n = seat_ids.iter().max().unwrap() + seat_ids.iter().min().unwrap();
    let sum = seat_ids.iter().sum::<usize>();

    (n * (seat_ids.len() + 1) / 2 - sum) as u32 // Gauss sum
}

fn get_seat_ids(input: &String) -> Vec<usize> {
    input
        .lines()
        .map(|p| {
            let b = binary_encode_passport(&p);
            (b >> 3) * 8 + (b & 7)
        })
        .collect()
}

fn binary_encode_passport(passport: &str) -> usize {
    usize::from_str_radix(
        &passport
            .chars()
            .map(|c| if "BR".contains(c) { "1" } else { "0" })
            .collect::<String>(),
        2,
    )
    .unwrap()
}
