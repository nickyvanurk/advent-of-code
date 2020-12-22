pub fn part1(input: &String) -> u32 {
    let mut rules = input
        .split("\n\n")
        .nth(0)
        .unwrap()
        .lines()
        .collect::<Vec<&str>>();
    let messages = input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .collect::<Vec<&str>>();

    rules.sort_by(|a, b| {
        a.split(':')
            .nth(0)
            .unwrap()
            .parse::<u16>()
            .unwrap()
            .cmp(&b.split(':').nth(0).unwrap().parse::<u16>().unwrap())
    });

    let pos_a = rules.iter().position(|&r| r.contains("a")).unwrap() as u16;
    let pos_b = rules.iter().position(|&r| r.contains("b")).unwrap() as u16;

    let rules_parsed = rules
        .iter()
        .map(|r| {
            if r.contains("a") || r.contains("b") {
                return Vec::<Vec<u16>>::new();
            }

            let rule = r.split(':').nth(1).unwrap().trim();
            rule.split(" | ")
                .map(|x| {
                    x.split_whitespace()
                        .map(|n| n.parse::<u16>().unwrap())
                        .collect::<Vec<u16>>()
                })
                .collect::<Vec<Vec<u16>>>()
        })
        .collect::<Vec<Vec<Vec<u16>>>>();

    let result = get_messages_from_rule(0, &rules_parsed, pos_a, pos_b);
    let mut matched_messages = 0;

    for message in result.iter() {
        if messages.contains(&&message[..]) {
            matched_messages += 1;
        }
    }

    matched_messages
}

pub fn part2(input: &String) -> u32 {
    0
}

fn get_messages_from_rule(
    rule_idx: u16,
    rules: &Vec<Vec<Vec<u16>>>,
    pos_a: u16,
    pos_b: u16,
) -> Vec<String> {
    let mut result = Vec::<String>::new();

    if rule_idx == pos_a {
        result.push("a".to_string());
    } else if rule_idx == pos_b {
        result.push("b".to_string())
    } else {
        for branch in &rules[rule_idx as usize] {
            let mut resu = Vec::<Vec<String>>::new();

            for &rule_num in branch.iter() {
                resu.push(get_messages_from_rule(rule_num, &rules, pos_a, pos_b));
            }

            let mut temp = Vec::<String>::new();

            if resu.len() > 1 {
                for i in 0..resu.len() - 1 {
                    temp = if temp.len() == 0 {
                        get_permutations(&resu[i], &resu[i + 1])
                    } else {
                        get_permutations(&temp, &resu[i + 1])
                    };
                }
            } else {
                temp.append(&mut resu[0]);
            }

            result.append(&mut temp);
        }
    }

    result
}

fn get_permutations(s1: &Vec<String>, s2: &Vec<String>) -> Vec<String> {
    let mut result = Vec::<String>::new();

    for i in 0..s1.len() {
        for j in 0..s2.len() {
            let mut s: String = "".to_string();
            s += &s1[i][..];
            s += &s2[j][..];
            result.push(s);
        }
    }

    result
}
