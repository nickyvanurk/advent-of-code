use std::collections::HashSet;

pub fn part1(input: &String) -> u32 {
    let (mut points, folds) = parse_input(&input);
    let &(axis, line) = folds.first().unwrap();
    fold(&mut points, axis, line);
    points.len() as u32
}

pub fn part2(input: &String) -> u32 {
    let (mut points, folds) = parse_input(&input);
    folds.iter().for_each(|&(axis, line)| fold(&mut points, axis, line));
    print_points(&points);
    points.len() as u32
}

fn parse_input(input: &String) -> (HashSet<(u32, u32)>, Vec<(&str, u32)>) {
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

    (points, folds)
}

fn fold(points: &mut HashSet<(u32, u32)>, axis: &str, line: u32) {
    let mut points_below_fold = points.clone();

    if axis == "y" {
        points_below_fold.retain(|&(_x, y)| y > line);
        points.retain(|&(_x, y)| y <= line);
        points_below_fold.iter().for_each(|&(x, y)| { points.insert((x, y - 2 * (y - line))); });
    } else {
        points_below_fold.retain(|&(x, _y)| x > line);
        points.retain(|&(x, _y)| x <= line);
        points_below_fold.iter().for_each(|&(x, y)| { points.insert((x - 2 * (x - line), y)); });
    }
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