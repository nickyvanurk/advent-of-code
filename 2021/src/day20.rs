use std::collections::VecDeque;
use std::fmt;

pub fn part1(input: &String) -> u32 {
    let mut input = input.lines();
    let algorithm = input.nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut image = Image::new(input.skip(1).map(|l| l.chars().collect()).collect());

    println!("{}", image);
    image.enhance();
    println!("{}", image);

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

struct Image {
    data: VecDeque<VecDeque<char>>
}

impl Image {
    fn new(data: VecDeque<VecDeque<char>>) -> Self {
        Self { data }
    }

    fn enhance(&mut self) {
        self.extend();
    }

    fn extend(&mut self) {
        self.data.push_front(VecDeque::from(vec!['.'; self.data[0].len()]));
        self.data.push_back(VecDeque::from(vec!['.'; self.data[0].len()]));

        for row in &mut self.data {
            row.push_front('.');
            row.push_back('.');
        }
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for row in &self.data {
            for &pixel in row {
                s.push(pixel);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}
