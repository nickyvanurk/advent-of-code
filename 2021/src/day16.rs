pub fn part1(input: &String) -> u32 {
    let input = input.trim();

    for byte  in hex::decode(input).unwrap() {
        let nibble1 = byte >> 4;
        let nibble2 = byte & 0b00001111;

        println!("{:04b}", nibble1);
        println!("{:04b}", nibble2);
    }

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.trim();

    0
}
