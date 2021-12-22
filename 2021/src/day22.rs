use std::collections::HashMap;

pub fn part1(input: &String) -> u32 {
    let input = input.lines();
    let mut steps = vec![];
    let region_size = 100;
    let region_half_size = region_size / 2;

    for line in input {
        let mut s = line.split(&[' ', ','][..]);
        steps.push(Step { 
            action: String::from(s.next().unwrap()),
            cuboid: Cuboid {
                x: parse_step_axis_into_tuple_and_translate(s.next().unwrap(), region_half_size),
                y: parse_step_axis_into_tuple_and_translate(s.next().unwrap(), region_half_size),
                z: parse_step_axis_into_tuple_and_translate(s.next().unwrap(), region_half_size),
            }
        });
    }

    steps = steps.into_iter().filter(|Step { cuboid, .. }| {
        let Cuboid { x, y, z } = cuboid;
        x.0 >= 0 && y.0 >= 0 && z.0 >= 0 &&
        x.1 <= region_size && y.1 <= region_size && z.1 <= region_size
    }).collect();

    let size = (region_size + 1) as usize;
    let mut reactor = vec![vec![vec![0u32; size]; size]; size];

    for Step { action, cuboid } in steps {
        for x in cuboid.x.0..=cuboid.x.1 {
            for y in cuboid.y.0..=cuboid.y.1 {
                for z in cuboid.z.0..=cuboid.z.1 {
                    if action == "on" {
                        reactor[x as usize][y as usize][z as usize] = 1;
                    } else {
                        reactor[x as usize][y as usize][z as usize] = 0;
                    }
                }
            }
        }
    }

    reactor.iter().flatten().flatten().sum::<u32>()
}

pub fn part2(input: &String) -> i64 {
    let input = input.lines();
    let mut steps = vec![];
    let region_size = 100;
    let region_half_size = region_size / 2;

    for line in input {
        let mut s = line.split(&[' ', ','][..]);
        steps.push(Step { 
            action: String::from(s.next().unwrap()),
            cuboid: Cuboid {
                x: parse_step_axis_into_tuple_and_translate(s.next().unwrap(), 0),
                y: parse_step_axis_into_tuple_and_translate(s.next().unwrap(), 0),
                z: parse_step_axis_into_tuple_and_translate(s.next().unwrap(), 0),
            }
        });
    }

    let mut counts: HashMap<Cuboid, i32> = HashMap::new();
    for Step { action, cuboid } in steps {
        let mut new_counts: HashMap<Cuboid, i32> = HashMap::new();

        for (c, _) in &counts {
            if let Some(overlap_cuboid) = overlap(&cuboid, &c) {
                *new_counts.entry(overlap_cuboid).or_insert(0) -= counts[c];
            }
        }

        if action == "on" {
            *new_counts.entry(cuboid).or_insert(0) += 1;
        }

        for (c, _) in &new_counts {
            *counts.entry(*c).or_insert(0) += new_counts[c];
        }
    }

    counts.into_iter().fold(0, |acc, (cuboid, count)| acc + volume(&cuboid) * count as i64)
}

#[derive(Debug, Eq, Hash, Copy, Clone)]
struct Cuboid {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl PartialEq for Cuboid {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x &&
        self.y == other.y &&
        self.z == other.z
    }
}

#[derive(Debug)]
struct Step {
    action: String,
    cuboid: Cuboid,
}

fn parse_step_axis_into_tuple_and_translate(data: &str, translation: i32) -> (i32, i32) {
    let mut parts = data[2..].split("..");
    (parts.next().unwrap().parse::<i32>().unwrap() + translation,
     parts.next().unwrap().parse::<i32>().unwrap() + translation)
}

fn overlap(c1: &Cuboid, c2: &Cuboid) -> Option<Cuboid> {
    if c1.x.1 < c2.x.0 || c2.x.1 < c1.x.0 ||
       c1.y.1 < c2.y.0 || c2.y.1 < c1.y.0 || 
       c1.z.1 < c2.z.0 || c2.z.1 < c1.z.0{
        return None;
    }

    Some(Cuboid {
        x: (std::cmp::max(c1.x.0, c2.x.0), std::cmp::min(c1.x.1, c2.x.1)),
        y: (std::cmp::max(c1.y.0, c2.y.0), std::cmp::min(c1.y.1, c2.y.1)),
        z: (std::cmp::max(c1.z.0, c2.z.0), std::cmp::min(c1.z.1, c2.z.1)),
    })
}

fn volume(c: &Cuboid) -> i64 {
    let Cuboid { x, y, z } = c;
    (x.1 - x.0 + 1) as i64 * (y.1 - y.0 + 1) as i64 * (z.1 - z.0 + 1) as i64
}