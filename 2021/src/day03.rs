pub fn part1(input: &String) -> u32 {
    let input: Vec<&str> = input.trim().lines().collect();

    let mut bits: String = String::from("");
    for i in 0..12 {
        bits += &get_nth_most_common_bit(&input, i).to_string();
    }
    
    let gamma_rate = u16::from_str_radix(&bits, 2).unwrap();
    let epsilon_rate = !gamma_rate & 0b0000111111111111;

    gamma_rate as u32 * epsilon_rate as u32
}

pub fn part2(input: &String) -> u32 {
    let input: Vec<&str> = input.trim().lines().collect();
    
    let oxygen_generator_rating = get_rating(&input, true);
    let co2_scrubber_rating = get_rating(&input, false);

    oxygen_generator_rating as u32 * co2_scrubber_rating as u32
}

fn get_rating(input: &Vec<&str>, most_common: bool) -> u16 {
    let mut input_clone = input.clone();

    for i in 0..12 {
        let bit = get_nth_most_common_bit(&input_clone, i);
        input_clone = input_clone.iter().cloned().filter(|line| {
            (if most_common { bit } else { !bit & 1 }) == get_nth_bit(line, i)
        }).collect();

        if input_clone.len() == 1 {
            break;
        }
    }

    binary_string_to_u16(input_clone.first().unwrap())
}

fn get_nth_most_common_bit(input: &Vec<&str>, idx: usize) -> u16 {
    let total_positive_bits = input.iter().fold(0, |acc, line| acc + get_nth_bit(line, idx));
    (total_positive_bits as f64 / input.len() as f64).round() as u16
}

fn get_nth_bit(line: &str, idx: usize) -> u16 {
    (line.as_bytes()[idx] as char).to_digit(2).unwrap() as u16
}

fn binary_string_to_u16(s: &str) -> u16 {
    u16::from_str_radix(s, 2).unwrap()
}