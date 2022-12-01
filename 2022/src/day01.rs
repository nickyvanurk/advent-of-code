use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    input.split("\n\n").map(|s| {
        s.split('\n').map(|n| n.parse::<u32>().unwrap()).sum()
    }).max().unwrap()
}

pub fn part2(input: &str) -> u32 {
    input.split("\n\n").map(|s| {
        s.split('\n').map(|n| n.parse::<u32>().unwrap()).sum::<u32>()
    }).sorted().rev().take(3).sum()
}