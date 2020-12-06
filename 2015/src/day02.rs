use std::cmp;

pub fn part1(input: String) {
    let total_sq_feet_wrapping_paper: u32 = input
        .lines()
        .map(|line| {
            let dimensions = line
                .split('x')
                .map(|dimension| dimension.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let sides = (
                dimensions[0] * dimensions[1],
                dimensions[1] * dimensions[2],
                dimensions[2] * dimensions[0],
            );
            let smallest_side = cmp::min(sides.0, cmp::min(sides.1, sides.2));

            2 * sides.0 + 2 * sides.1 + 2 * sides.2 + smallest_side
        })
        .sum();

    println!("{}", total_sq_feet_wrapping_paper);
}

pub fn part2(input: String) {
    let total_feet_ribbon: usize = input
        .lines()
        .map(|line| {
            let mut dimensions = line
                .split('x')
                .map(|dimension| dimension.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            dimensions.sort();

            let perimeter = dimensions[0] * 2 + dimensions[1] * 2;
            let cubic_feet = dimensions[0] * dimensions[1] * dimensions[2];

            perimeter + cubic_feet
        })
        .sum();

    println!("{}", total_feet_ribbon);
}
