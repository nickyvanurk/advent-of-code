use itertools::Itertools;

pub fn part1(input: &str) -> u32 {
    let grid: Vec<Vec<u32>> = input.lines().map(|s| {
        s.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect::<Vec<Vec<u32>>>();

    let mut tallest_tree_indexes = vec![];
    let grid_size = (grid[0].len(), grid.len());

    // Check horizontal
    for y in 0..grid.len() {
        let mut tallest_left = (0, y);
        let mut tallest_right = (grid_size.0 - 1, y);

        for x in 0..grid[0].len() {
            if is_border(x, y, grid_size) {
                tallest_tree_indexes.push((x, y));
            } else {
                let tree_left = (x, y);
                let tree_right = (grid_size.0 - 1 - x, y);

                if grid[tree_left.1][tree_left.0] > grid[tallest_left.1][tallest_left.0] {
                    tallest_tree_indexes.push(tree_left);
                    tallest_left = tree_left;
                }

                if grid[tree_right.1][tree_right.0] > grid[tallest_right.1][tallest_right.0] {
                    tallest_tree_indexes.push(tree_right);
                    tallest_right = tree_right;
                }
            }
        }
    }

    // Check vertical
    for x in 0..grid_size.0 {
        let mut tallest_top = (x, 0);
        let mut tallest_bottom = (x, grid_size.1 - 1);

        for y in 0..grid_size.1 {
            if is_border(x, y, grid_size) {
                tallest_tree_indexes.push((x, y));
            } else {
                let tree_top = (x, y);
                let tree_bottom = (x, grid_size.1 - 1 - y);

                if grid[tree_top.1][tree_top.0] > grid[tallest_top.1][tallest_top.0] {
                    tallest_tree_indexes.push(tree_top);
                    tallest_top = tree_top;
                }

                if grid[tree_bottom.1][tree_bottom.0] > grid[tallest_bottom.1][tallest_bottom.0] {
                    tallest_tree_indexes.push(tree_bottom);
                    tallest_bottom = tree_bottom;
                }
            }
        }
    }

    tallest_tree_indexes.iter().unique().count() as u32
}

pub fn part2(input: &str) -> u32 {
    0
}

fn is_border(x: usize, y: usize, grid_size: (usize, usize)) -> bool {
    x == 0 || y == 0 || x == grid_size.0 - 1 || y == grid_size.1 -1
}