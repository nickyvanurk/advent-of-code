use std::collections::HashSet;

pub fn part1(input: &String) -> u32 {
    let mut points = HashSet::new();
    let mut folds: Vec<(&str, u32)> = vec![];

    for line in input.lines() {
        if line.contains(',') {
            let mut split = line.split(',');
            let values = (split.next().unwrap().parse::<u32>().unwrap(), split.next().unwrap().parse::<u32>().unwrap());
            points.insert(values);
        } else if line.contains('=') {
            folds.push((&line[11..12], line[13..].parse::<u32>().unwrap()));
        }
    }

    println!("{:?}", points);
    println!("{:?}", folds);

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}