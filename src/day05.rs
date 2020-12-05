pub fn part1(input: String) {
    println!("{:?}", get_seat_ids(&input).iter().max().unwrap());
}

pub fn part2(input: String) {
    let mut seat_ids: Vec<u16> = get_seat_ids(&input);

    seat_ids.sort();

    let my_seat = seat_ids
        .iter()
        .enumerate()
        .filter(|(i, &id)| {
            let next = match seat_ids.get(i + 1) {
                Some(&seat_id) => seat_id,
                None => return false,
            };

            id + 1 != next
        })
        .map(|(_, &id)| id)
        .nth(0)
        .unwrap()
        + 1;

    println!("{}", my_seat);
}

fn get_seat_ids(input: &String) -> Vec<u16> {
    input
        .lines()
        .map(|passport| {
            let mut b = 0;

            for (i, c) in passport.chars().enumerate() {
                if c == 'B' || c == 'R' {
                    b += u16::pow(2, (passport.len() - 1 - i) as u32);
                }
            }

            (b >> 3) * 8 + (b & 0x7)
        })
        .collect()
}
