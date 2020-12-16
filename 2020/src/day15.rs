use std::collections::HashMap;

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
    let mut spoken_numbers = input
        .trim()
        .split(',')
        .enumerate()
        .map(|(i, n)| (n.parse::<u32>().unwrap(), i as u32 + 1))
        .collect::<HashMap<u32, u32>>();

    let mut last = spoken_numbers.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let mut last_spoken_number: (u32, u32) = (*last.0, *last.1);
    spoken_numbers.remove(&last_spoken_number.0);

    for turn in spoken_numbers.len() + 2..=30000000 {
        if !spoken_numbers.contains_key(&last_spoken_number.0) {
            spoken_numbers.insert(last_spoken_number.0, last_spoken_number.1);
            last_spoken_number = (0, turn as u32);
        } else {
            last = spoken_numbers.get_key_value(&last_spoken_number.0).unwrap();

            let l = last_spoken_number;
            last_spoken_number = ((turn as u32 - 1) - last.1, turn as u32);
            *spoken_numbers.get_mut(&l.0).unwrap() = l.1;
        }
    }

    last_spoken_number.0
}
