use std::collections::HashMap;

pub fn part1(input: &String) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

    for line in input.lines() {
        if line.contains("mask") {
            mask = line.split(" = ").nth(1).unwrap();
            continue;
        }

        let parsed = line
            .replace("mem[", "")
            .replace("]", "")
            .split(" = ")
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let address = parsed[0];
        let mut value = parsed[1];

        for (idx, c) in mask.chars().rev().enumerate() {
            if c == '1' {
                value |= u64::pow(2, idx as u32);
            } else if c == '0' {
                value &= !u64::pow(2, idx as u32);
            }
        }

        if mem.contains_key(&address) {
            *mem.get_mut(&address).unwrap() = value;
        } else {
            mem.insert(address, value);
        }
    }

    mem.values().sum()
}

pub fn part2(input: &String) -> u64 {
    let mut mem = HashMap::<u64, u64>::new();
    let mut mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

    for line in input.lines() {
        if line.contains("mask") {
            mask = line.split(" = ").nth(1).unwrap();
            continue;
        }

        let parsed = line
            .replace("mem[", "")
            .replace("]", "")
            .split(" = ")
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let mut address = parsed[0];
        let value = parsed[1];

        let mut addresses: Vec<u64> = vec![];

        for (idx, c) in mask.chars().rev().enumerate() {
            if c == '1' {
                address |= u64::pow(2, idx as u32);
            }
        }

        for i in 0..u64::pow(2, mask.matches("X").count() as u32) {
            for (j, (idx, _)) in mask
                .chars()
                .rev()
                .enumerate()
                .filter(|&(_, c)| c == 'X')
                .enumerate()
            {
                let p = u64::pow(2, j as u32 + 1);
                let set_bit = i % p >= p / 2;

                if set_bit {
                    address |= 1 << idx;
                } else {
                    address &= !(1 << idx);
                }
            }

            addresses.push(address);
        }

        for &address in addresses.iter() {
            if mem.contains_key(&address) {
                *mem.get_mut(&address).unwrap() = value;
            } else {
                mem.insert(address, value);
            }
        }
    }

    mem.values().sum()
}
