use std::collections::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn part1(input: &String) -> u32 {
    let size = input.lines().nth(0).unwrap().chars().count() as u32;
    let data = input.replace("\n", "").chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<u32>>();
    let grid = Grid::new(data, size, size);

    let start = Point { x: 0, y: 0 };
    let end = Point { x: (size - 1) as i32, y: (size - 1) as i32 };

    get_lowest_risk_score(&grid, &start, &end)
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines().map(|l| l.chars().map(|d| d.to_digit(10).unwrap()).collect()).collect::<Vec<Vec<u32>>>();

    0
}

fn get_lowest_risk_score(grid: &Grid, start: &Point, end: &Point) -> u32 {
    let mut frontier = BinaryHeap::new();
    frontier.push(Cell{ position: *start, cost: 0 });

    let mut came_from = HashMap::from([(*start, *start)]);
    let mut risk_so_far = HashMap::from([(*start, grid.risk_level(&start))]);

    while !frontier.is_empty() {
        let current = &frontier.pop().unwrap();

        if current.position == *end {
            break;
        }

        for next in grid.neighbors(&current.position) {
            let next_risk_level = risk_so_far[&current.position] + grid.risk_level(&next);
            if !risk_so_far.contains_key(&next) || next_risk_level < risk_so_far[&next] {
                *risk_so_far.entry(next).or_insert(0) = next_risk_level;
                frontier.push(Cell { position: next, cost: next_risk_level });
                *came_from.entry(next).or_insert(*start) = current.position;
            }
        }
    }

    let mut total_risk = 0;
    let mut current = *end;
    while current != *start {
        total_risk += grid.risk_level(&current);
        current = came_from[&current];
    }

    total_risk
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.x.cmp(&self.x).then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Cell {
    position: Point,
    cost: u32,
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    data: Vec<u32>,
    width: u32,
    height: u32,
}

impl Grid {
    fn new(data: Vec<u32>, width: u32, height: u32) -> Grid {
        Grid { data, width, height }
    }

    fn in_bounds(&self, p: &Point) -> bool {
        let Point { x, y } = *p;
        0 <= x && x < self.width as i32 && 0 <= y && y < self.height as i32
    }

    fn risk_level(&self, p: &Point) -> u32 {
        let Point { x, y } = *p;
        self.data[(y * self.width as i32 + x) as usize]
    }

    fn neighbors(&self, p: &Point) -> Vec<Point> {
        let Point { x, y } = *p;
        let neighbors = vec![Point{ x: x+1, y }, Point{ x: x-1, y }, Point{ x, y: y+1 }, Point{ x, y: y-1 }];
        neighbors.into_iter().filter(|p| self.in_bounds(p)).collect()
    }
}