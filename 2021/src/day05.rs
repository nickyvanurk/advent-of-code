use std::collections::HashMap;
use std::cmp;

pub fn part1(input: &String) -> u16 {
    let mut vents: HashMap<(u16, u16), u16> = HashMap::new();
    
    input.trim().split('\n').for_each(|s| {
        let points: Vec<Vec<u16>> = s.split(" -> ").map(|p| p.split(',').map(|i| i.parse::<u16>().unwrap()).collect::<Vec<u16>>()).collect();

        let x1 = points[0][0];
        let x2 = points[1][0];

        let y1 = points[0][1];
        let y2 = points[1][1];

        if x1 == x2 {
            for y in cmp::min(y1, y2)..=cmp::max(y1, y2) {
                let counter = vents.entry((x1, y)).or_insert(0);
                *counter += 1;
            }
        }

        if y1 == y2 {
            for x in cmp::min(x1, x2)..=cmp::max(x1, x2) {
                let counter = vents.entry((x, y1)).or_insert(0);
                *counter += 1;
            }
        }
    });

    vents.into_values().filter(|&v| v > 1).count() as u16
}

pub fn part2(input: &String) -> u16 {
    let input = input.lines().map(|s| s.parse::<u16>().unwrap());

    0
}