use std::collections::HashMap;

pub fn part1(input: &String) -> u32 {
    let mut input: Vec<u8> = input.trim().split(',').map(|v| v.parse::<u8>().unwrap()).collect();
    let days = 80;

    for _ in 1..=days {
        for i in 0..input.len() {
            if input[i] >= 1 {
                input[i] -= 1
            } else {
                input[i] = 6;
                input.push(8);
            }
        }
    }

    input.len() as u32
}

pub fn part2(input: &String) -> u64 {
    let mut fish = HashMap::<u16, i64>::new();
    input.trim().split(',').for_each(|v| {
        add_fish(&mut fish, v.parse::<u16>().unwrap(), 1);
    });
    let days = 256;

    for _ in 1..=days {
        for i in 0..=9 { // add one extra day in hashmap to temporarily store added fish
            if let Some(&num_fish) = &fish.get(&(i as u16)) {
                if i == 0 {
                    add_fish(&mut fish, i, -num_fish);
                    add_fish(&mut fish, 7, num_fish);
                    add_fish(&mut fish, 9, num_fish);
                } else {
                    add_fish(&mut fish, i, -num_fish);
                    add_fish(&mut fish, i - 1, num_fish);
                }
            }
        }
    }

    fish.into_values().fold(0, |acc, v| acc + v as u64)
}

fn add_fish(fish: &mut HashMap<u16, i64>, days_till_spawn: u16, value: i64) {
    let counter = fish.entry(days_till_spawn).or_insert(0);
    *counter += value;
}