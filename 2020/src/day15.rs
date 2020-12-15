pub fn part1(input: &String) -> u32 {
    let mut spoken_numbers = input
        .trim()
        .split(',')
        .enumerate()
        .map(|(i, n)| (n.parse::<u32>().unwrap(), i as u32 + 1))
        .collect::<Vec<(u32, u32)>>();

    let mut turn: u32 = spoken_numbers.len() as u32 + 1;

    while turn < 2020 {
        let last_spoken_number = spoken_numbers
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _)| k)
            .unwrap();

        if spoken_numbers
            .iter()
            .filter(|&(number, turn)| number == last_spoken_number)
            .count()
            == 1
        {
            spoken_numbers.push((0, turn));
        } else {
            let last_time_spoken = spoken_numbers
                .iter()
                .find(|&(number, turn)| number == last_spoken_number && turn < &(turn - 1))
                .unwrap();

            println!("{:?}", last_time_spoken);
        }

        turn += 1;
    }

    println!("{:?}", spoken_numbers);

    0
}

pub fn part2(input: &String) -> u32 {
    0
}
