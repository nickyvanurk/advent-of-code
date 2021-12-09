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

    0
}
