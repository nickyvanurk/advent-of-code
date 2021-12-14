use itertools::Itertools;

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

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}