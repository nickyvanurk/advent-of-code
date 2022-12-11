extern crate queues;
use queues::*;

extern crate num;
use num::*;

pub fn part1(input: &str) -> u128 {
    let monkey_data = input.split("\n\n");
    let mut monkeys: Vec<Monkey> = vec!{};

    for data in monkey_data {
        let mut items: Queue<u128> = queue![];
        let mut operation = "".to_string();
        let mut test = 0;
        let mut throw_to = (0, 0);

        for line in data.lines().skip(1) {
            if line.contains("Starting items") {
                let v: Vec<u128> = line.split(':').nth(1).unwrap().trim().split(", ").map(|s| s.parse::<u128>().unwrap()).collect();
                for item in v {
                    items.add(item);
                }
            } else if line.contains("Operation") {
                operation = line.split("= old ").nth(1).unwrap().to_string();
            } else if line.contains("Test") {
                test = line.split("divisible by ").nth(1).unwrap().parse::<u128>().unwrap();
            } else if line.contains("If true") {
                throw_to.0 = line.split("throw to monkey ").nth(1).unwrap().parse::<u128>().unwrap();
            } else if line.contains("If false") {
                throw_to.1 = line.split("throw to monkey ").nth(1).unwrap().parse::<u128>().unwrap();
            }
        }

        monkeys.push(Monkey { items, operation, test, throw_to, inspection_count: 0 });
    }

    for _ in 0..20 {
        for idx in 0..monkeys.len() {
            while let Ok(mut item) = monkeys[idx].items.remove() {
                let t = monkeys[idx].operation.split_whitespace().collect::<Vec<&str>>();
                let instruction = t[0];
                let value = t[1];

                if value.parse::<u128>().is_ok() {
                    if instruction == "*" {
                        item *= value.parse::<u128>().unwrap();
                    } else if instruction == "+" {
                        item += value.parse::<u128>().unwrap();
                    }
                } else {
                    item *= item;
                }

                item /= 3;

                if item % monkeys[idx].test == 0 {
                    let idx = monkeys[idx].throw_to.0 as usize;
                    monkeys[idx].items.add(item);
                } else {
                    let idx = monkeys[idx].throw_to.1 as usize;
                    monkeys[idx].items.add(item);
                }

                monkeys[idx].inspection_count += 1;
            }
        }
    }

    let mut inspection_counts = vec![];
    for monkey in monkeys {
        inspection_counts.push(monkey.inspection_count);
    }

    inspection_counts.sort();
    inspection_counts = inspection_counts.into_iter().rev().collect();

    inspection_counts[0] * inspection_counts[1]
}

pub fn part2(input: &str) -> u128 {
    let monkey_data = input.split("\n\n");
    let mut monkeys: Vec<Monkey> = vec!{};

    let mut test_lcm = 1;

    for data in monkey_data {
        let mut items: Queue<u128> = queue![];
        let mut operation = "".to_string();
        let mut test = 0;
        let mut throw_to = (0, 0);

        for line in data.lines().skip(1) {
            if line.contains("Starting items") {
                let v: Vec<u128> = line.split(':').nth(1).unwrap().trim().split(", ").map(|s| s.parse::<u128>().unwrap()).collect();
                for n in v {
                    items.add(n);
                }
            } else if line.contains("Operation") {
                operation = line.split("= old ").nth(1).unwrap().to_string();
            } else if line.contains("Test") {
                test = line.split("divisible by ").nth(1).unwrap().parse::<u128>().unwrap();
                test_lcm = integer::lcm(test_lcm, test);
            } else if line.contains("If true") {
                throw_to.0 = line.split("throw to monkey ").nth(1).unwrap().parse::<u128>().unwrap();
            } else if line.contains("If false") {
                throw_to.1 = line.split("throw to monkey ").nth(1).unwrap().parse::<u128>().unwrap();
            }
        }

        monkeys.push(Monkey { items, operation, test, throw_to, inspection_count: 0 });
    }

    for _ in 0..10000 {
        for idx in 0..monkeys.len() {
            while let Ok(mut item) = monkeys[idx].items.remove() {
                let t = monkeys[idx].operation.split_whitespace().collect::<Vec<&str>>();
                let instruction = t[0];
                let value = t[1];

                if value != "old" {
                    if instruction == "*" {
                        item *= value.parse::<u128>().unwrap();
                    } else if instruction == "+" {
                        item += value.parse::<u128>().unwrap();
                    }
                } else {
                    item *= item;
                }

                item %= test_lcm;

                if item % monkeys[idx].test == 0 {
                    let idx = monkeys[idx].throw_to.0 as usize;
                    monkeys[idx].items.add(item);
                } else {
                    let idx = monkeys[idx].throw_to.1 as usize;
                    monkeys[idx].items.add(item);
                }

                monkeys[idx].inspection_count += 1;
            }
        }
    }

    let mut inspection_counts = vec![];
    for monkey in monkeys {
        inspection_counts.push(monkey.inspection_count);
    }

    inspection_counts.sort();
    inspection_counts = inspection_counts.into_iter().rev().collect();

    inspection_counts[0] * inspection_counts[1]
}

struct Monkey {
    items: Queue<u128>,
    operation: String,
    test: u128,
    throw_to: (u128, u128),
    inspection_count: u128,
}