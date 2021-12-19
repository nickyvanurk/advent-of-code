pub fn part1(input: &String) -> u32 {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let scanners = input.into_iter().map(|data| parse_scanner(&data)).collect::<Vec<Scanner>>();

    println!("{:?}", scanners[0]);

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

#[derive(Debug)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Beacon {
    position: Vector3,
}

#[derive(Debug)]
struct Scanner {
    id: u32,
    up: Vector3,
    beacons: Vec<Beacon>,
}

fn parse_scanner(data: &str) -> Scanner {
    let mut lines = data.lines();
    let scanner_id = lines.nth(0).unwrap().chars().filter(|c| c.is_digit(10)).last().unwrap().to_digit(10).unwrap();

    let mut scanner = Scanner {
        id: scanner_id,
        up: Vector3 { x: 0, y: 1, z: 0},
        beacons: vec![],
    };

    for line in lines {
        let v = line.split(',').map(|d| d.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        scanner.beacons.push(Beacon {
            position: Vector3 {
                x: v[0],
                y: v[1],
                z: v[2],
            }
        });
    }

    scanner
}