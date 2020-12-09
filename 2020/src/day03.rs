pub fn part1(input: &String) -> u32 {
    let map = input.lines().collect();

    get_trees_from_slope(3, 1, &map) as u32
}

pub fn part2(input: &String) -> u64 {
    let map = input.lines().collect();

    (get_trees_from_slope(1, 1, &map)
        * get_trees_from_slope(3, 1, &map)
        * get_trees_from_slope(5, 1, &map)
        * get_trees_from_slope(7, 1, &map)
        * get_trees_from_slope(1, 2, &map)) as u64
}

fn get_trees_from_slope(x_delta: usize, y_delta: usize, map: &Vec<&str>) -> usize {
    map.iter()
        .enumerate()
        .filter(|(y, row)| {
            return if y % y_delta == 0 {
                row.as_bytes()[y / y_delta * x_delta % row.len()] as char == '#'
            } else {
                false
            };
        })
        .count()
}
