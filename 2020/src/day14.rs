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
        let adress = parsed[0];
        let mut value = parsed[1];

        for (idx, c) in mask.chars().rev().enumerate() {
            if c == '1' {
                value |= u64::pow(2, idx as u32);
            } else if c == '0' {
                value &= !u64::pow(2, idx as u32);
            }
        }

        if mem.contains_key(&adress) {
            *mem.get_mut(&adress).unwrap() = value;
        } else {
            mem.insert(adress, value);
        }
    }

    mem.values().sum()
}

pub fn part2(input: &String) -> u32 {
    0
}
