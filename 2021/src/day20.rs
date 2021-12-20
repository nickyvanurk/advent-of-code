use std::collections::VecDeque;
use std::fmt;

pub fn part1(input: &String) -> u32 {
    let mut input = input.lines();
    let algorithm = input.nth(0).unwrap().chars().collect::<Vec<char>>();
    let mut image = Image::new(input.skip(1).map(|l| l.chars().collect()).collect());

    println!("{:?}", algorithm.len());

    image.extend();
    image.extend();
    image.extend();
    image.extend();
    image.extend();
    image.extend();
    image.extend();
    image.extend();
    image.extend();
    image.extend();
    image.extend();
    println!("{}", image);
    image.enhance(&algorithm);
    println!("{}", image);
    image.enhance(&algorithm);
    println!("{}", image);

    image.light_pixels()
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

    fn light_pixels(&self) -> u32 {
        self.data.iter().fold(0, |acc, v| acc + v.iter().filter(|&&c| c == '#').count()) as u32
    }

    fn enhance(&mut self, algorithm: &Vec<char>) {
        self.extend();

        let mut data = self.data.clone();

        for y in 1..(self.data.len() - 1) {
            for x in 1..(self.data[y].len() - 1) {
                let mut pixel_str = String::new();

                // Tried to fix bug by not calculating a pixel surrounded by only black pixels. Seems to not produce the correct answer.
                if !vec![(-1,-1),(0,-1),(1,-1),(-1,0),(0,0),(1,0),(-1,1),(0,1),(1,1)].iter().map(|(a, b)| ((x as i32 + a) as usize, (y as i32 + b) as usize)).any(|(x, y)| self.data[y][x] == '#') {
                    continue;
                }

                for coord in vec![(-1,-1),(0,-1),(1,-1),(-1,0),(0,0),(1,0),(-1,1),(0,1),(1,1)] {
                    pixel_str.push(self.data[(y as i32 + coord.1) as usize][(x as i32 + coord.0) as usize]);
                }

                let pixel_num = u32::from_str_radix(&pixel_str.replace("#", "1").replace(".", "0"), 2).unwrap();
                data[y][x] = algorithm[pixel_num as usize];
            }
        }

        self.data = data;
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