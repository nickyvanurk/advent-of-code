pub fn part1(input: &String) -> u32 {
    let input = input.trim();
    let bits = decode_input(&input);

    let mut sum_packet_versions = 0;
    decode_message(&bits[0..], &mut Some(&mut sum_packet_versions));

    sum_packet_versions
}

pub fn part2(input: &String) -> u64 {
    let input = input.trim();
    let bits = decode_input(&input);

    decode_message(&bits[0..], &mut None).0
}

fn decode_input(input: &str) -> Vec<u8> {
    let bytes = hex::decode(input).unwrap();
    let mut bits = vec![];

    for byte in bytes {
        for i in (0..8).rev() {
            bits.push((byte >> i) & 1);
        }
    }

    bits
}

fn decode_message(bits: &[u8], sum_packet_versions: &mut Option<&mut u32>) -> (u64, usize) {
    let version = bits_to_u64(&bits[0..3]);
    let type_id = bits_to_u64(&bits[3..6]);

    if let Some(sum_versions) = sum_packet_versions {
        *(*sum_versions) += version as u32;
    }

    if type_id == 4 {
        let result = decode_packet_literal(&bits[0..]);
        return result;
    } else {
        let length_type_id = bits_to_u64(&bits[6..7]);

        if length_type_id == 0 {
            let total_bits_sub_packets = bits_to_u64(&bits[7..22]);

            let mut values: Vec<u64> = vec![];
            let mut current_idx = 22 as usize;
            while current_idx < (22 + total_bits_sub_packets) as usize {
                let (value, idx) = decode_message(&bits[current_idx as usize..], sum_packet_versions);
                values.push(value);
                current_idx += idx;
            }

            return (calculate_value(&values, &type_id), current_idx);
        } else if length_type_id == 1 {
            let total_sub_packets = bits_to_u64(&bits[7..18]);

            let mut values = vec![];
            let mut current_idx = 18;
            let mut current_packet = 1;
            while current_packet <= total_sub_packets {
                let (value, idx) = decode_message(&bits[current_idx as usize..], sum_packet_versions);
                values.push(value);
                current_idx += idx;
                current_packet += 1;
            }

            return (calculate_value(&values, &type_id), current_idx);
        }
    }

    println!("Error(decode_message): did not return correct packet index");
    return (0, 0);
}

fn calculate_value(values: &Vec<u64>, type_id: &u64) -> u64 {
    match type_id {
        0 => values.iter().sum(),
        1 => values.iter().product(),
        2 => *values.iter().min().unwrap(),
        3 => *values.iter().max().unwrap(),
        5 => if values[0] > values[1] { 1 } else { 0 },
        6 => if values[0] < values[1] { 1 } else { 0 },
        7 => if values[0] == values[1] { 1 } else { 0 },
        _ => 0,
    }
}

fn decode_packet_literal(bits: &[u8]) -> (u64, usize) {
    let mut value = vec![];
    let mut start_idx = 6;
    let group_size = 5;
    let mut should_continue = true;

    while should_continue {
        let nibble = &bits[start_idx+1..start_idx + group_size];
        value.extend_from_slice(nibble);

        should_continue = bits_to_u64(&bits[start_idx..start_idx+1]) == 1;
        start_idx += group_size;
    }

    (bits_to_u64(&value[0..]), start_idx)
}

fn bits_to_u64(bits: &[u8]) -> u64 {
    bits.iter().fold(0, |acc, &b| acc as u64*2 + b as u64) as u64
}