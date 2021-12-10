use std::collections::HashMap;

pub fn part1(input: &String) -> u32 {
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let mut opening_chars: Vec<char> = vec![];

    let score_table = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut score = 0;

    input.lines().for_each(|l| {
        l.chars().for_each(|c| {
            if pairs.contains_key(&c) {
                opening_chars.push(c);
            } else if pairs[&opening_chars.pop().unwrap()] != c {
                score += score_table[&c];
            }
        })
    });

    score
}

pub fn part2(input: &String) -> u16 {
    let input = input.lines();

    0
}