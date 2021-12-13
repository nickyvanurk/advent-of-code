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

    let &(axis, fold) = folds.first().unwrap();

    if axis == "y" {
        let mut points_below_fold = points.clone();
        points_below_fold.retain(|&(_x, y)| y > fold);
        points.retain(|&(_x, y)| y <= fold);

        for &(x, y) in &points_below_fold {
            points.insert((x, y - 2 * (y - fold)));
        }
    } else {
        let mut points_below_fold = points.clone();
        points_below_fold.retain(|&(x, _y)| x > fold);
        points.retain(|&(x, _y)| x <= fold);

        for &(x, y) in &points_below_fold {
            points.insert((x - 2 * (x - fold), y));
        }
    }

    points.len() as u32
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

fn print_points(points: &HashSet<(u32, u32)>) {
    let width = points.iter().max_by_key(|&(x, _y)| x).unwrap().0;
    let height = points.iter().max_by_key(|&(_x, y)| y).unwrap().1;

    for y in 0..=height {
        for x in 0..=width {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!("");
    }
}