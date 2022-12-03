use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let input = input.lines().map(|s| (&s[..s.len()/2], &s[s.len()/2..]));
    let mut sum = 0;

    for (a, b) in input {
        let set: HashSet<char> = a.chars().collect();
        let common_char = b.chars().filter(|c| set.contains(&c)).nth(0).unwrap();

        sum += if common_char.is_lowercase() {
            (common_char as u32 - 'a' as u32) + 1
        } else {
            (common_char as u32 - 'A' as u32) + 27
        };
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let input = input.lines().collect::<Vec<&str>>();
    let mut sum = 0;

    for i in (0..input.len()).step_by(3) {
        let set1: HashSet<char> = input[i].chars().collect();
        let set2: HashSet<char> = input[i + 1].chars().collect();
        let common_char = input[i + 2].chars().filter(|c| set1.contains(&c) && set2.contains(&c)).nth(0).unwrap();
        
        sum += if common_char.is_lowercase() {
            (common_char as u32 - 'a' as u32) + 1
        } else {
            (common_char as u32 - 'A' as u32) + 27
        };
    }

    sum
}