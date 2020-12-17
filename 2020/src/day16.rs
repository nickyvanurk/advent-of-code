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
            let valid = rules_map.values().all(|ranges| {
                !((number >= ranges.0.min && number <= ranges.0.max)
                    || (number >= ranges.1.min && number <= ranges.1.max))
            });

            if valid {
                invalid_values_nearby_tickets.push(number);
            }
        }
    }

    invalid_values_nearby_tickets.iter().sum()
}

pub fn part2(input: &String) -> u32 {
    0
}

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}
