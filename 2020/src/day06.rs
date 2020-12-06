use itertools::Itertools;

pub fn part1(input: String) {
    println!(
        "{}",
        input
            .split("\n\n")
            .map(|group| group.replace("\n", "").chars().unique().count())
            .sum::<usize>()
    );
}

pub fn part2(input: String) {
    println!(
        "{}",
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
            .sum::<usize>()
    );
}
