pub fn part1(input: &String) -> u32 {
    let mut output_joltages = input
        .lines()
        .map(|joltage| joltage.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    output_joltages.sort();

    let mut steps = (1, 1);

    for i in 1..output_joltages.len() {
        let step = output_joltages[i] - output_joltages[i - 1];

        match step {
            1 => steps.0 += 1,
            3 => steps.1 += 1,
            _ => (),
        }
    }

    steps.0 * steps.1
}

pub fn part2(input: &String) -> u64 {
    let mut output_joltages = input
        .lines()
        .map(|joltage| joltage.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    output_joltages.sort();
    let device_joltage = *output_joltages.last().unwrap();
    let mut perm = [0usize; 256];

    for i in 0..4 {
        perm[i] = 1;
    }

    for joltage in output_joltages {
        let n = perm[joltage as usize];

        for i in 1..=3 {
            perm[(joltage as usize) + i] += n;
        }
    }

    perm[device_joltage as usize] as u64
}
