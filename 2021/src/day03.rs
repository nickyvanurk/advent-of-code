pub fn part1(input: &String) -> u32 {
    let input: Vec<u16> = input.trim().lines().map(|s| u16::from_str_radix(s, 2).unwrap()).collect();

    let mut bits: String = String::from("");
    for i in 0..12 {
        bits += &get_most_common_bit(&input, i).to_string();
    }
    
    let gamma_rate = u16::from_str_radix(&bits, 2).unwrap();
    let epsilon_rate = !gamma_rate & 0b0000111111111111;

    gamma_rate as u32 * epsilon_rate as u32
}

pub fn part2(input: &String) -> u32 {
    let input: Vec<u16> = input.trim().lines().map(|s| u16::from_str_radix(s, 2).unwrap()).collect();
    
    let mut oxygen_generator_rating = 0;
    let mut input_left: Vec<u16> = input.clone();
    for i in 0..12 {
        let bit = get_most_common_bit(&input_left, i);

        input_left = input_left.iter().cloned().filter(|data| {
            bit == data >> (11 - i) & 1
        }).collect();

        if input_left.iter().count() == 1 {
            oxygen_generator_rating = *input_left.first().unwrap();
            break;
        }
    }

    let mut co2_scrubber_rating = 0;
    let mut input_left = input.clone();
    for i in 0..12 {
        let bit = get_most_common_bit(&input_left, i);

        input_left = input_left.iter().cloned().filter(|data| {
            bit != data >> (11 - i) & 1
        }).collect();

        if input_left.iter().count() == 1 {
            co2_scrubber_rating = *input_left.first().unwrap();
            break;
        }
    }
    
    oxygen_generator_rating as u32 * co2_scrubber_rating as u32
}

fn get_most_common_bit(input: &Vec<u16>, idx: usize) -> u16 {
    let mut bit_num = 0;
    let mut input_length = 0;

    for data in input.iter() {
        input_length += 1;
        bit_num += data >> (11 - idx) & 1;
    }

    if bit_num >= (input_length - bit_num) { 1 } else { 0 }
}