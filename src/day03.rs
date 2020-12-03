pub fn part1(input: String) {
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|s| s.chars().map(|x| x == '#').collect())
        .collect();

    println!("{}", get_trees_from_slope(3, 1, &map));
}

pub fn part2(input: String) {
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|s| s.chars().map(|x| x == '#').collect())
        .collect();

    println!(
        "{}",
        get_trees_from_slope(1, 1, &map)
            * get_trees_from_slope(3, 1, &map)
            * get_trees_from_slope(5, 1, &map)
            * get_trees_from_slope(7, 1, &map)
            * get_trees_from_slope(1, 2, &map),
    );
}

fn get_trees_from_slope(x_delta: usize, y_delta: usize, map: &Vec<Vec<bool>>) -> usize {
    let mut trees = 0;

    for y in (y_delta..map.len()).step_by(y_delta) {
        if map[y][(y / y_delta) * x_delta % map[0].len()] {
            trees += 1;
        }
    }

    trees
}
