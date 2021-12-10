use std::collections::HashMap;

pub fn part1(input: &String) -> u32 {
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let score_table = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut score = 0;

    input.lines().for_each(|l| {
        let mut opening_chars: Vec<char> = vec![];

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

pub fn part2(input: &String) -> u64 {
    let pairs = HashMap::from([('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let score_table = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut scores = vec![];

    input.lines().for_each(|l| {
        let mut opening_chars: Vec<char> = vec![];

        for c in l.chars() {
            if pairs.contains_key(&c) {
                opening_chars.push(c);
            } else if pairs[&opening_chars.pop().unwrap()] != c {
                return;
            }
        }

        let mut score = 0;
        for c in opening_chars.iter().rev() {
            score = score * 5 + score_table[&pairs[&c]];
        }
        scores.push(score);
    });

    scores.sort();
    scores[scores.len() / 2]
}