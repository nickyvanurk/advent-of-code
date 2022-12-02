use std::collections::HashMap;

pub fn part1(input: &str) -> u32 {
    let mut map = HashMap::new();

    map.insert("A X", 1 + 3);
    map.insert("A Y", 2 + 6);
    map.insert("A Z", 3 + 0);

    map.insert("B X", 1 + 0);
    map.insert("B Y", 2 + 3);
    map.insert("B Z", 3 + 6);

    map.insert("C X", 1 + 6);
    map.insert("C Y", 2 + 0);
    map.insert("C Z", 3 + 3);

    input.lines().map(|s| map.get(s).unwrap()).sum()
}

pub fn part2(input: &str) -> u32 {
    let mut map = HashMap::new();

    map.insert("A X", 3 + 0);
    map.insert("A Y", 1 + 3);
    map.insert("A Z", 2 + 6);

    map.insert("B X", 1 + 0);
    map.insert("B Y", 2 + 3);
    map.insert("B Z", 3 + 6);

    map.insert("C X", 2 + 0);
    map.insert("C Y", 3 + 3);
    map.insert("C Z", 1 + 6);

    input.lines().map(|s| map.get(s).unwrap()).sum()
}