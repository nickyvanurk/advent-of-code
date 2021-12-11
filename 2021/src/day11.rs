pub fn part1(input: &String) -> u32 {
    let mut input: Vec<Vec<u32>> = input.lines().map(|s| s.chars().map(|s| s.to_digit(10).unwrap()).collect()).collect();
    let mut total_flashes = 0;

    for _step in 1..=100 {
        for y in 0..input.len() {
            for x in 0..input[y].len() {
                input[y][x] += 1;
            }
        }

        total_flashes += count_flashes(&mut input);
    }

    total_flashes
}

fn count_flashes(input: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashes = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] >= 10 {
                input[y][x] = 0;
                flashes += 1;

                if x > 0  && input[y][x - 1] > 0 {
                    input[y][x - 1] += 1;
                }

                if x < input[y].len() - 1 && input[y][x + 1] > 0  {
                    input[y][x + 1] += 1;
                }

                if y > 0  && input[y - 1][x] > 0  {
                    input[y - 1][x] += 1;
                }

                if y < input.len() - 1 && input[y + 1][x] > 0  {
                    input[y + 1][x] += 1;
                }

                if x > 0 && y > 0 && input[y - 1][x - 1] > 0  {
                    input[y - 1][x - 1] += 1;
                }

                if x > 0 && y < input.len() - 1 && input[y + 1][x - 1] > 0  {
                    input[y + 1][x - 1] += 1;
                }

                if x < input[y].len() - 1  && y > 0 && input[y - 1][x + 1] > 0  {
                    input[y - 1][x + 1] += 1;
                }

                if x < input[y].len() - 1  && y < input.len() - 1 && input[y + 1][x + 1] > 0  {
                    input[y + 1][x + 1] += 1;
                }

                flashes += count_flashes(input);
            }
        }
    }

    flashes
}

fn print_input(input: &Vec<Vec<u32>>) {
    for row in input {
        for x in row {
            print!("{}", x);
        }

        println!("");
    }
}

pub fn part2(input: &String) -> u32 {
    let mut input: Vec<Vec<u32>> = input.lines().map(|s| s.chars().map(|s| s.to_digit(10).unwrap()).collect()).collect();
    
    0
}