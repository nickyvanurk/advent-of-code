use itertools::Itertools;
use std::collections::HashMap;

pub fn part1(input: &String) -> u32 {
    let mut template = String::from(input.lines().nth(0).unwrap());
    let rules: Vec<Vec<&str>> = input.lines().skip(2).map(|l| l.split(" -> ").collect()).collect();
    
    for _step in 0..10 {
        let mut last_char = template.chars().nth(0).unwrap();
        let chars: Vec<char> = template.clone().chars().collect();
        let mut insert_count = 0;

        for i in 1..chars.len() {
            let rule_key = format!("{}{}", last_char, chars[i]);

            for rule in &rules {
                if rule[0] == rule_key {
                    template.insert_str(i + insert_count, rule[1]);
                    insert_count += 1;
                }
            }

            last_char = chars[i];
        }
    }

    let quantities = template.chars()
                             .sorted()
                             .group_by(|&x| x)
                             .into_iter().map(|(_, group)| group.count() as u32)
                             .collect::<Vec<u32>>();

    quantities.iter().max().unwrap() - quantities.iter().min().unwrap()
}

pub fn part2(input: &String) -> u64 {
    let template = String::from(input.lines().nth(0).unwrap());
    let rules: HashMap<String, &str> = input.lines().skip(2).map(|l| {
        let mut split = l.split(" -> ");
        (String::from(split.next().unwrap()), split.next().unwrap())
    }).collect();

    let mut pairs_count: HashMap<String, u64> = rules.keys().map(|k| (String::from(k), 0)).collect();
    let mut chars_count: HashMap<char, u64> = HashMap::new();
    for (i, c) in template.chars().enumerate() {
        *chars_count.entry(c).or_insert(0) += 1;

        if i > 0 {
            *pairs_count.entry(format!("{}{}", template.chars().nth(i-1).unwrap(), c)).or_insert(0) += 1;
        }
    }

    for _ in 0..40 {
        for (pair, count) in pairs_count.clone() {
            if count == 0 {
                continue;
            }

            let c = rules[&pair];

            *chars_count.entry(c.as_bytes()[0] as char).or_insert(0) += count;

            *pairs_count.get_mut(&pair).unwrap() -= count;
            *pairs_count.get_mut(&format!("{}{}", pair.as_bytes()[0] as char, c)).unwrap() += count;
            *pairs_count.get_mut(&format!("{}{}", c, pair.as_bytes()[1] as char)).unwrap() += count;
        }
    }

    chars_count.iter().map(|(_, &count)| count).max().unwrap() - chars_count.iter().map(|(_, &count)| count).min().unwrap()
}