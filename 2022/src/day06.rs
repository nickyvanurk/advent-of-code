use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    for (idx, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
        if (a != b && a != c && a != d) && (b != c && b != d) && (c != d) {
            return idx as u32 + 4;
        }
    }

    0
}

pub fn part2(input: &str) -> u32 {
    for (idx, window) in input.chars().collect::<Vec<char>>().windows(14).enumerate() {
        if window.len() == window.into_iter().unique().count() {
            return idx as u32 + 14;
        }
    }

    0
}