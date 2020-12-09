use itertools::Itertools;

pub fn part1(input: &String) -> u32 {
    input
        .split("\n\n")
        .map(|group| group.replace("\n", "").chars().unique().count())
        .sum::<usize>() as u32
}

pub fn part2(input: &String) -> u32 {
    input
        .split("\n\n")
        .map(|group| {
            group
                .replace("\n", "")
                .chars()
                .filter(|&c| group.matches(c).count() == group.lines().count())
                .unique()
                .count()
        })
        .sum::<usize>() as u32
}
