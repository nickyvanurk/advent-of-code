use std::collections::HashMap;

pub fn part1(input: &String) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let mut mask = "";

    for line in input.lines() {
        if line.contains("mask") {
            mask = line.split(" = ").nth(1).unwrap();
            continue;
        }

        let parsed = parse_address_value(line);
        let address = parsed[0];
        let mut value = parsed[1];

        for (idx, c) in mask.chars().rev().enumerate() {
            if c == '1' {
                value = set_bit(value, u64::pow(2, idx as u32));
            } else if c == '0' {
                value = unset_bit(value, u64::pow(2, idx as u32));
            }
        }

        insert_mem_value(&mut mem, address, value);
    }

    mem.values().sum()
}

pub fn part2(input: &String) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let mut mask = "";

    for line in input.lines() {
        if line.contains("mask") {
            mask = line.split(" = ").nth(1).unwrap();
            continue;
        }

        let parsed = parse_address_value(line);
        let mut address = parsed[0];
        let value = parsed[1];

        let mut addresses: Vec<u64> = vec![];

        for (idx, _) in mask.chars().rev().enumerate().filter(|&(_, c)| c == '1') {
            address |= u64::pow(2, idx as u32);
        }

        for i in 0..u64::pow(2, mask.matches("X").count() as u32) {
            for (j, (idx, _)) in mask
                .chars()
                .rev()
                .enumerate()
                .filter(|&(_, c)| c == 'X')
                .enumerate()
            {
                let pow = u64::pow(2, j as u32 + 1);

                if i % pow >= pow / 2 {
                    address = set_bit(address, 1 << idx);
                } else {
                    address = unset_bit(address, 1 << idx);
                }
            }

            addresses.push(address);
        }

        for &address in addresses.iter() {
            insert_mem_value(&mut mem, address, value);
        }
    }

    mem.values().sum()
}

fn set_bit(num: u64, bit: u64) -> u64 {
    num | bit
}

fn unset_bit(num: u64, bit: u64) -> u64 {
    num & !bit
}

fn parse_address_value(line: &str) -> Vec<u64> {
    line.replace("mem[", "")
        .replace("]", "")
        .split(" = ")
        .map(|num| num.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn insert_mem_value(mem: &mut HashMap<u64, u64>, address: u64, value: u64) {
    if mem.contains_key(&address) {
        *mem.get_mut(&address).unwrap() = value;
    } else {
        mem.insert(address, value);
    }
}
