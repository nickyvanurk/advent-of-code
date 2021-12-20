use std::collections::VecDeque;

type Image = VecDeque<VecDeque<char>>;

pub fn part1(input: &String) -> u32 {
    let mut input = input.lines();
    let algorithm = input.nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut image: Image = input.skip(1).map(|l| l.chars().collect()).collect();

    print_image(&image);
    enhance_image(&mut image);
    print_image(&image);

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

fn print_image(data: &Image) {
    for row in data {
        for pixel in row {
            print!("{}", pixel);
        }
        println!("");
    }
}

fn enhance_image(data: &mut Image) {
    extend_image(data);
}

fn extend_image(data: &mut Image) {
    data.push_front(VecDeque::from(vec!['.'; data[0].len()]));
    data.push_back(VecDeque::from(vec!['.'; data[0].len()]));

    for row in data {
        row.push_front('.');
        row.push_back('.');
    }
}