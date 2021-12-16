pub fn part1(input: &String) -> u32 {
    let input = input.trim();
    let bits = decode_input(&input);

    decode_message(&bits);

    0
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

fn decode_message(bits: &Vec<u8>) {
    let version = bits_to_u32(&bits[0..3]);
    let type_id = bits_to_u32(&bits[3..6]);

    println!("Version: {}", version);
    println!("Type ID: {}", type_id);

    if type_id == 4 {
        let value = decode_packet_literal(&bits[0..]);
        println!("Packet literal: {}", value);
    }
}

fn decode_packet_literal(bits: &[u8]) -> u32 {
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

    bits_to_u32(&value[0..])
}

fn bits_to_u32(bits: &[u8]) -> u32 {
    bits.iter().fold(0, |acc, &b| acc*2 + b as u32)
}