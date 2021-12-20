pub fn part1(input: &String) -> u32 {
    let mut input = input.lines();
    let algorithm = input.nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut image: Vec<Vec<char>> = input.skip(1).map(|l| l.chars().collect()).collect();

    print_image(&image);

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

fn print_image(data: &Vec<Vec<char>>) {
    for row in data {
        for pixel in row {
            print!("{}", pixel);
        }
        println!("");
    }
}