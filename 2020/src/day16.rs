use std::collections::HashMap;

pub fn part1(input: &String) -> u32 {
    let mut rules_map = HashMap::<String, (Range, Range)>::new();

    let mut split = input.split("\n\n");
    let rules = split.next().unwrap().replace("or ", "");
    rules.lines().for_each(|l| {
        let mut rule = l.split(':');
        let name = rule.next().unwrap().replace(":", "");
        let mut ranges = rule.next().unwrap().trim().split(' ');
        let range1 = ranges
            .next()
            .unwrap()
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let range2 = ranges
            .next()
            .unwrap()
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        rules_map.insert(
            name,
            (
                Range {
                    min: range1[0],
                    max: range1[1],
                },
                Range {
                    min: range2[0],
                    max: range2[1],
                },
            ),
        );
    });
    let numbers_my_ticket = split
        .next()
        .unwrap()
        .replace("your ticket:\n", "")
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let numbers_nearby_tickets = split
        .next()
        .unwrap()
        .replace("nearby tickets:\n", "")
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut invalid_values_nearby_tickets = Vec::<u32>::new();

    for numbers in numbers_nearby_tickets {
        for number in numbers {
            let invalid = rules_map.values().all(|ranges| {
                !((number >= ranges.0.min && number <= ranges.0.max)
                    || (number >= ranges.1.min && number <= ranges.1.max))
            });

            if invalid {
                invalid_values_nearby_tickets.push(number);
            }
        }
    }

    invalid_values_nearby_tickets.iter().sum()
}

pub fn part2(input: &String) -> u64 {
    let mut rules_map = HashMap::<String, (Range, Range)>::new();
    let mut rules_keys_ordered = Vec::<String>::new();

    let mut split = input.split("\n\n");
    let rules = split.next().unwrap().replace("or ", "");
    rules.lines().for_each(|l| {
        let mut rule = l.split(':');
        let name = rule.next().unwrap().replace(":", "");
        rules_keys_ordered.push(name.clone());
        let mut ranges = rule.next().unwrap().trim().split(' ');
        let range1 = ranges
            .next()
            .unwrap()
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let range2 = ranges
            .next()
            .unwrap()
            .split('-')
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        rules_map.insert(
            name,
            (
                Range {
                    min: range1[0],
                    max: range1[1],
                },
                Range {
                    min: range2[0],
                    max: range2[1],
                },
            ),
        );
    });
    let numbers_my_ticket = split
        .next()
        .unwrap()
        .replace("your ticket:\n", "")
        .split(',')
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let numbers_nearby_tickets = split
        .next()
        .unwrap()
        .replace("nearby tickets:\n", "")
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut valid_nearby_tickets = Vec::<Vec<u32>>::new();

    valid_nearby_tickets.push(numbers_my_ticket.clone());

    for numbers in numbers_nearby_tickets {
        let mut valid_ticket = true;

        for &number in numbers.iter() {
            let invalid = rules_map.values().all(|ranges| {
                !((number >= ranges.0.min && number <= ranges.0.max)
                    || (number >= ranges.1.min && number <= ranges.1.max))
            });

            if invalid {
                valid_ticket = false;
                break;
            }
        }

        if valid_ticket {
            valid_nearby_tickets.push(numbers);
        }
    }

    let mut rules_ordered = [""; 20];
    let mut rules_num = 0;

    while rules_num < 20 {
        for rule in rules_keys_ordered.iter() {
            let ranges = rules_map.get(rule).unwrap();
            let mut valid_rule_num = 0;
            let mut last_valid_index = 0;

            for i in 0..rules_map.len() {
                if rules_ordered[i] != "" {
                    continue;
                }

                for (j, numbers) in valid_nearby_tickets.iter().enumerate() {
                    let valid_rule = (numbers[i] >= ranges.0.min && numbers[i] <= ranges.0.max)
                        || (numbers[i] >= ranges.1.min && numbers[i] <= ranges.1.max);

                    if !valid_rule {
                        break;
                    } else if j == (valid_nearby_tickets.len() - 1) {
                        valid_rule_num += 1;
                        last_valid_index = i;
                    }
                }
            }

            if valid_rule_num == 1 {
                rules_ordered[last_valid_index] = rule as &str;
                rules_num += 1;
            }
        }
    }

    let mut product: u64 = 1;

    for (i, rule) in rules_ordered.iter().enumerate() {
        if rule.contains("departure") {
            product *= numbers_my_ticket[i] as u64;
        }
    }

    product
}

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}
