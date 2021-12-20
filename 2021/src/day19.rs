use std::collections::HashMap;
use std::ops::Sub;
use std::ops::Add;
use itertools::Itertools;

pub fn part1(input: &String) -> u32 {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let scanners = input.into_iter().map(|data| parse_scanner(&data)).collect::<Vec<Scanner>>();

    find_relative_beacons(&scanners);

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector3 {
    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0
    }
}

impl Sub for Vector3 {
    type Output = Self; 

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vector3 {
    type Output = Self; 

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Beacon {
    position: Vector3,
}

impl Beacon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { position: Vector3 { x, y, z } }
    }

    fn relative_position(&self, other: &Self) -> Vector3 {
        let mut difference = self.position - other.position;
        difference.x = i32::abs(difference.x);
        difference.y = i32::abs(difference.y);
        difference.z = i32::abs(difference.z);
        difference
    }
}

#[derive(Debug)]
struct Scanner {
    id: u32,
    up: Vector3,
    beacons: Vec<Beacon>,
    relative_beacon_positions: HashMap<Vector3, (Beacon, Beacon)>,
}

impl Scanner {
    fn new(id: u32) -> Self {
        Self {
            id,
            up: Vector3 { x: 0, y: 1, z: 0},
            beacons: vec![],
            relative_beacon_positions: HashMap::new(),
        }
    }

    fn add_beacon(&mut self, x: i32, y: i32, z: i32) {
        let beacon = Beacon::new(x, y, z);
        for b in &self.beacons {
            self.relative_beacon_positions.insert(beacon.relative_position(b), (beacon, *b));
        }
        self.beacons.push(beacon);
    }

    fn get_overlapping_beacons(&self, other: &Self) -> Vec<(Beacon, Beacon)> {
        let mut overlapping_beacons = vec![];
        for i in 0..self.beacons.len()-1 {
            let beacon_a = &self.beacons[i];
            for j in i+1..self.beacons.len() {
                let beacon_b = &self.beacons[j];
                let relative_position = beacon_a.relative_position(beacon_b);

                if let Some((a, b)) = other.relative_beacon_positions.get(&relative_position) {
                    if overlapping_beacons.iter().all(|(x, _)| x != beacon_a && x != beacon_b) {
                        overlapping_beacons.push((*beacon_a, *a));
                        overlapping_beacons.push((*beacon_b, *b));
                    }
                }
            }
        }
        overlapping_beacons
    }
}

fn parse_scanner(data: &str) -> Scanner {
    let mut lines = data.lines();
    let scanner_id = lines.nth(0).unwrap().chars().filter(|c| c.is_digit(10)).last().unwrap().to_digit(10).unwrap();
    let mut scanner = Scanner::new(scanner_id);

    for line in lines {
        let v = line.split(',').map(|d| d.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        scanner.add_beacon(v[0], v[1], v[2]);
    }

    scanner
}

fn find_relative_beacons(scanners: &Vec<Scanner>) {
    for i in 0..scanners.len()-1 {
        println!("Scanner: {},", i);
        let scanner_a = &scanners[i];
        
        for j in i+1..scanners.len() {
            let scanner_b = &scanners[j];
            let shared_beacons = scanner_b.get_overlapping_beacons(scanner_a);

            if shared_beacons.len() < 12 {
                continue;
            }

            let mut facing = Vector3 { x: 0, y: 0, z: 0};
            for beacon in &shared_beacons {
                let p0 = beacon.0.position;
                let p1 = beacon.1.position;

                // facing, then 'up' from remaining 2 axis


                    // Kinda unsure how to proceed

                let items = vec![1, 1, 1, -1, -1, -1];
                for f in items.iter().permutations(3).unique() {
                    let x = p0.x * f[0] + p1.x;
                    let y = p0.y * f[1] + p1.y;
                    let z = p0.z * f[2] + p1.z;

                    for up in vec![1, 1, -1, -1].iter().permutations(2).unique() {
                        println!("{:?}", up);

                    }

                    let found = shared_beacons.iter().all(|(a, b)| {
                        let a = a.position;
                        let b = b.position;
                        
                        a.x * f[0] + b.x == x &&
                        a.y * f[1] + b.y == y &&
                        a.z * f[2] + b.z == z
                    });

                    if found {
                        facing.x = x;
                        facing.y = y;
                        facing.z = z;
                        break;
                    }
                }

                println!("");
               
                if !facing.is_zero() {
                    break;
                }
            }
            

            if !facing.is_zero() {
                println!("Scanner {} facing: {:?}", i, facing);
            } else {
                println!("Scanner {} facing not found", i);
            }

            println!("");

        }

        println!("");
    }
}