use std::collections::HashSet;

pub fn part1(input: &str) -> u32 {
    let mut head_position = Point { x: 0, y: 0 };
    let mut tail_position = Point { x: 0, y: 0 };

    let mut tail_visited_points = HashSet::new();
    tail_visited_points.insert(tail_position);

    for line in input.lines() {
        let a = line.split(' ').collect::<Vec<&str>>();
        let instruction = a[0];
        let value = a[1].parse::<i32>().unwrap();

        for _ in 0..value {
            match instruction {
                "U" => head_position.y -= 1,
                "D" => head_position.y += 1,
                "L" => head_position.x -= 1,
                "R" => head_position.x += 1,
                _ => println!("Instruction not found.")
            }

            if tail_position.distance_to(&head_position) >= 2.0 {
                if i32::abs(head_position.x - tail_position.x) > 0 {
                    let direction = (head_position.x - tail_position.x) / i32::abs(head_position.x - tail_position.x);
                    tail_position.x += direction;
                }

                if i32::abs(head_position.y - tail_position.y) > 0 {
                    let direction = (head_position.y - tail_position.y) / i32::abs(head_position.y - tail_position.y);
                    tail_position.y += direction;
                }

                tail_visited_points.insert(tail_position);
            }
        }
    }

    tail_visited_points.len() as u32
}

pub fn part2(input: &str) -> u32 {
    let starting_position = Point { x: 0, y: 0 };

    let mut knots = vec![];
    for _ in 0..10 {
        knots.push(starting_position);
    }

    let mut tail_visited_points = HashSet::new();
    tail_visited_points.insert(starting_position);

    for line in input.lines() {
        let a = line.split(' ').collect::<Vec<&str>>();
        let instruction = a[0];
        let value = a[1].parse::<i32>().unwrap();

        for _ in 0..value {
            match instruction {
                "U" => knots[0].y -= 1,
                "D" => knots[0].y += 1,
                "L" => knots[0].x -= 1,
                "R" => knots[0].x += 1,
                _ => println!("Instruction not found.")
            }

            for i in 1..knots.len() {
                if knots[i].distance_to(&knots[i - 1]) >= 2.0 {
                    if i32::abs(knots[i - 1].x - knots[i].x) > 0 {
                        let direction = (knots[i - 1].x - knots[i].x) / i32::abs(knots[i - 1].x - knots[i].x);
                        knots[i].x += direction;
                    }

                    if i32::abs(knots[i - 1].y - knots[i].y) > 0 {
                        let direction = (knots[i - 1].y - knots[i].y) / i32::abs(knots[i - 1].y - knots[i].y);
                        knots[i].y += direction;
                    }

                    if i == knots.len() - 1 {
                        tail_visited_points.insert(knots[i]);
                    }
                }
            }
        }
    }

    tail_visited_points.len() as u32
}

#[derive(Copy, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance_to(&self, p: &Point) -> f32 {
        f32::sqrt((p.x - &self.x).pow(2) as f32 + (p.y - &self.y).pow(2) as f32)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}