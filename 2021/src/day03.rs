pub fn part1(input: &String) -> u32 {
    let input = input.trim().lines().map(|s| u16::from_str_radix(s, 2).unwrap());
    let mut nth_bit_num = [0; 12];
    let mut input_length = 0;

    for data in input {
        input_length += 1;

        for i in 0..12 {
            nth_bit_num[i] += data >> (11 - i) & 1;
        }
    }
    
    let bits: String = nth_bit_num.map(|i| if i > input_length / 2 { '1' } else { '0' }).iter().collect();
    let gamma_rate = u16::from_str_radix(&bits, 2).unwrap();
    let epsilon_rate = !gamma_rate & 0b0000111111111111;

    gamma_rate as u32 * epsilon_rate as u32
}

pub fn part2(input: &String) -> u16 {
    0
}
