pub fn part1(input: &String) -> u32 {
    let input: Vec<Vec<u32>> = input.lines().map(|s| s.chars().map(|s| s.to_digit(10).unwrap()).collect()).collect();
    let mut low_points = vec![];

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let digit = input[y][x];

            if x > 0 && digit >= input[y][x - 1] ||
               x < input[y].len() - 1 && digit >= input[y][x + 1] ||
               y > 0 && digit >= input[y - 1][x] ||
               y < input.len() - 1 && digit >= input[y + 1][x] {
                continue;
            }

            low_points.push(digit);
        }
    }
    
    low_points.iter().map(|d| d + 1).sum::<u32>()
}

pub fn part2(input: &String) -> u32 {
    let input: Vec<Vec<u32>> = input.lines().map(|s| s.chars().map(|s| s.to_digit(10).unwrap()).collect()).collect();
    let mut low_points_coords = vec![];

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let digit = input[y][x];

            if x > 0 && digit >= input[y][x - 1] ||
               x < input[y].len() - 1 && digit >= input[y][x + 1] ||
               y > 0 && digit >= input[y - 1][x] ||
               y < input.len() - 1 && digit >= input[y + 1][x] {
                continue;
            }

            low_points_coords.push((x, y));
        }
    }

    let mut basin_sizes = vec![];
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    for coords in low_points_coords {
        basin_sizes.push(get_basin_size(&input, &mut visited, coords));
    }

    basin_sizes.sort();
    basin_sizes.iter_mut().rev().take(3).fold(1, |acc, &mut x| acc * x)
}

fn get_basin_size(input: &Vec<Vec<u32>>, visited: &mut Vec<Vec<bool>>, start: (usize, usize)) -> u32 {
    let (x, y) = start;
    let mut basin_size = 1;
    let point = input[y][x];

    visited[y][x] = true;

    if x > 0 && !visited[y][x - 1] && point < input[y][x - 1] && input[y][x - 1] != 9  {
        basin_size += get_basin_size(&input, visited, (x - 1, y));
    }

    if x < input[y].len() - 1 && !visited[y][x + 1] && point < input[y][x + 1] && input[y][x + 1] != 9 {
        basin_size += get_basin_size(&input, visited, (x + 1, y));
    }

    if y > 0 && !visited[y - 1][x] && point < input[y - 1][x] && input[y - 1][x] != 9  {
        basin_size += get_basin_size(&input, visited, (x, y - 1));
    }

    if y < input.len() - 1 && !visited[y + 1][x] && point < input[y + 1][x] && input[y + 1][x] != 9 {
        basin_size += get_basin_size(&input, visited, (x, y + 1));
    }

    basin_size
}