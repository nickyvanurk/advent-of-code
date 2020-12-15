pub fn part1(input: &String) -> u32 {
    let mut spoken_numbers = input
        .trim()
        .split(',')
        .enumerate()
        .map(|(i, n)| (n.parse::<u32>().unwrap(), i as u32 + 1))
        .collect::<Vec<(u32, u32)>>();

    for turn in spoken_numbers.len() + 1..=2020 {
        let last_spoken_number = spoken_numbers
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _)| k)
            .unwrap();

        if spoken_numbers
            .iter()
            .filter(|&(number, _)| number == last_spoken_number)
            .count()
            == 1
        {
            spoken_numbers.push((0, turn as u32));
        } else {
            let last_time_spoken = spoken_numbers
                .iter()
                .filter(|&(number, _)| number == last_spoken_number)
                .rev()
                .skip(1)
                .collect::<Vec<&(u32, u32)>>()[0];

            spoken_numbers.push(((turn as u32 - 1) - last_time_spoken.1, turn as u32));
        }
    }

    spoken_numbers.last().unwrap().0
}

pub fn part2(input: &String) -> u32 {
    0
}
