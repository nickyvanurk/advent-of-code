pub fn part1(input: String) {
    println!("{:?}", get_seat_ids(&input).iter().max().unwrap());
}

pub fn part2(input: String) {
    let mut seat_ids: Vec<u32> = get_seat_ids(&input);

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
        .collect::<Vec<_>>()
        .first()
        .unwrap()
        + 1;

    println!("{}", my_seat);
}

fn get_seat_ids(input: &String) -> Vec<u32> {
    input
        .lines()
        .map(|pass| {
            let mut row_range = (0, 127);
            let mut col_range = (0, 7);

            for c in pass.chars() {
                match c {
                    'F' => row_range.1 = row_range.0 + (row_range.1 - row_range.0) / 2,
                    'B' => row_range.0 = row_range.1 - (row_range.1 - row_range.0) / 2,
                    'L' => col_range.1 = col_range.0 + (col_range.1 - col_range.0) / 2,
                    'R' => col_range.0 = col_range.1 - (col_range.1 - col_range.0) / 2,
                    _ => unreachable!(),
                }
            }

            row_range.0 * 8 + col_range.0
        })
        .collect()
}
