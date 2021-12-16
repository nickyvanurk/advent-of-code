pub fn part1(input: &String) -> u32 {
    let input = input.trim();
    let bits = decode_input(&input);

    let mut sum_packet_versions = 0;
    decode_message(&bits[0..], &mut sum_packet_versions);

    sum_packet_versions
}

pub fn part2(input: &String) -> u32 {
    let input = input.trim();

    0
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

fn decode_message(bits: &[u8], sum_packet_versions: &mut u32) -> u32 {
    let version = bits_to_u32(&bits[0..3]);
    let type_id = bits_to_u32(&bits[3..6]);

    *sum_packet_versions += version;

    // println!("Version: {}", version);
    // println!("Type ID: {}", type_id);

    if type_id == 4 {
        let (value, final_idx) = decode_packet_literal(&bits[0..]);
        // println!("Packet literal: {}", value);
        return final_idx;
    } else {
        let length_type_id = bits_to_u32(&bits[6..7]);
        // println!("Length type id: {}", length_type_id);

        if length_type_id == 0 {
            let total_bits_sub_packets = bits_to_u32(&bits[7..22]);
            // println!("Total bits sub-packets: {}", total_bits_sub_packets);

            let mut current_idx = 22;
            while current_idx < 22 + total_bits_sub_packets {
                current_idx += decode_message(&bits[current_idx as usize..], sum_packet_versions);
                // println!("Current idx: {}", current_idx);
            }

            return current_idx;
        } else if length_type_id == 1 {
            let total_sub_packets = bits_to_u32(&bits[7..18]);
            // println!("Total sub-packets: {}", total_sub_packets);

            let mut current_idx = 18;
            let mut current_packet = 1;
            while current_packet <= total_sub_packets {
                current_idx += decode_message(&bits[current_idx as usize..], sum_packet_versions);
                current_packet += 1;
                // println!("Current idx: {}", current_idx);
            }

            return current_idx;
        }
    }

    println!("Error(decode_message): did not return correct packet index");
    return 0;
}

fn decode_packet_literal(bits: &[u8]) -> (u32, u32) {
    let mut value = vec![];
    let mut start_idx = 6;
    let group_size = 5;
    let mut should_continue = true;

    while should_continue {
        let nibble = &bits[start_idx+1..start_idx + group_size];
        value.extend_from_slice(nibble);

        should_continue = bits_to_u32(&bits[start_idx..start_idx+1]) == 1;
        start_idx += group_size;
    }

    (bits_to_u32(&value[0..]), start_idx as u32)
}

fn bits_to_u32(bits: &[u8]) -> u32 {
    bits.iter().fold(0, |acc, &b| acc as u64*2 + b as u64) as u32
}