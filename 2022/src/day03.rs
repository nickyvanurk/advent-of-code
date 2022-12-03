use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    input.lines().map(|s| (&s[..s.len()/2], &s[s.len()/2..])).map(|(a, b)| {
        let set: HashSet<char> = a.chars().collect();
        let common_char = b.chars().filter(|c| set.contains(&c)).nth(0).unwrap();
        if common_char.is_lowercase() { (common_char as u32 - 'a' as u32) + 1 } else { (common_char as u32 - 'A' as u32) + 27 }
    }).sum()
}

pub fn part2(input: &str) -> u32 {
    input.lines().collect::<Vec<&str>>().chunks(3).map(|chunk| {
        let set1: HashSet<char> = chunk[0].chars().collect();
        let set2: HashSet<char> = chunk[1].chars().collect();
        let common_char = chunk[2].chars().filter(|c| set1.contains(&c) && set2.contains(&c)).nth(0).unwrap();
        if common_char.is_lowercase() { (common_char as u32 - 'a' as u32) + 1 } else { (common_char as u32 - 'A' as u32) + 27 }
    }).sum::<u32>()
}