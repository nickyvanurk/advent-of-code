pub fn part1(input: String) {
    let highest_seat_id = input
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
        .max()
        .unwrap();

    println!("{:?}", highest_seat_id);
}

pub fn part2(input: String) {
    let boarding_passes: Vec<&str> = input.lines().collect();

    let rows = 128;
    let columns = 8;

    let mut row = 0;
    let mut column = 0;

    let mut seat_ids = vec![];

    for pass in boarding_passes {
        let mut row_range = (0, rows - 1);

        for c in pass.chars().take(7) {
            let range = row_range.1 - row_range.0;

            match c {
                'F' => row_range.1 = row_range.0 + range / 2,
                'B' => row_range.0 = row_range.1 - range / 2,
                _ => unreachable!(),
            }

            row = row_range.0;
        }

        let mut col_range = (0, columns - 1);

        for c in pass.chars().skip(7) {
            let range = col_range.1 - col_range.0;

            match c {
                'L' => col_range.1 = col_range.0 + range / 2,
                'R' => col_range.0 = col_range.1 - range / 2,
                _ => unreachable!(),
            }

            column = col_range.0;
        }

        let seat_id = row * 8 + column;

        seat_ids.push(seat_id);
    }

    seat_ids.sort();

    println!(
        "{:?}",
        seat_ids
            .iter()
            .enumerate()
            .filter(|&(i, id)| {
                let next = match seat_ids.get(i + 1) {
                    Some(&seat_id) => seat_id,
                    None => return false,
                };

                id + 1 != next
            })
            .map(|t| *t.1)
            .collect::<Vec<u32>>()
            .first()
            .unwrap()
            + 1
    );
}
