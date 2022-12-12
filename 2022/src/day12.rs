pub fn part1(input: &str) -> u32 {
    let (map, start, target) = parse_map(&input);

    for row in &map {
        println!("{:?}", row);
    }

    0
}

pub fn part2(input: &str) -> u32 {
    0
}

fn parse_map(input: &str) -> (Vec<Vec<u32>>, Point, Point) {
    let mut map = vec![];
    let mut start = Point { x: 0, y: 0 };
    let mut target = Point { x: 0, y: 0 };

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];

        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Point { x, y };
                    row.push('a' as u32 - 'a' as u32);
                },
                'E' => {
                    target = Point { x, y };
                    row.push('z' as u32 - 'a' as u32);
                },
                _ => row.push(c as u32 - 'a' as u32)
            }
        }

        map.push(row);
    }

    (map, start, target)
}

struct Point {
    x: usize,
    y: usize,
}