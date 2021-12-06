pub fn part1(input: &String) -> u32 {
    let mut input: Vec<u8> = input.trim().split(',').map(|v| v.parse::<u8>().unwrap()).collect();
    let days = 80;

    for _ in 1..=days {
        for i in 0..input.len() {
            if input[i] >= 1 {
                input[i] -= 1
            } else {
                input[i] = 6;
                input.push(8);
            }
        }
    }

    input.len() as u32
}

pub fn part2(input: &String) -> u32 {
    let input: Vec<u8> = input.trim().split(',').map(|v| v.parse().unwrap()).collect();

    0
}
