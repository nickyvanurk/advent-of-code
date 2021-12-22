// Reboot reactor

// Reactor:
// 3D grid of cubes

// Each cube on and off, 1 and 0.
// They are all off at start of reboot

// Input: reboot steps (set cubes on or off), then it's rebooted.
// Each step is a cuboid.

// Initialization procedure has cuboids of min -50 and max 50. Ignore
// other steps for now.

// Execute reboot steps
// How many cubes are on in -50 50 region?

use itertools::izip;

#[derive(Debug)]
struct Step {
    action: String,
    x: (u32, u32),
    y: (u32, u32),
    z: (u32, u32),
}

// struct State {
//     xs: Vec<(i32, i32)>,
//     ys: Vec<(i32, i32)>,
//     zs: Vec<(i32, i32)>,
//     cubes: i32,
// }

// impl State {
//     fn new() -> Self {
//         Self {
//             xs: vec![],
//             ys: vec![],
//             zs: vec![],
//             cubes: 0,
//         }
//     }

//     fn cubes(&mut self) -> u32 {
//         let mut total = 0;
        
//         total as u32
//     }

//     fn turn_on(&mut self, step: &Step) {
//         let Step { x, y, z } = step;

//         self.xs.push(*x);
//         self.ys.push(*y);
//         self.zs.push(*z);

//         println!("Permutations: {:?}", (x.1 - x.0 + 1)*(y.1 - y.0 + 1)*(z.1 - z.0 + 1));


//         println!("ON: {:?}", self.xs);

//         let cubes = ((x.1 - x.0) + 1) * ((y.1 - y.0) + 1) * ((z.1 - z.0) + 1);
//         println!("ON Cubes: {}", cubes);
//         self.cubes += cubes;

//         for (x, y, z) in izip!(overlaps(&self.xs), overlaps(&self.ys), overlaps(&self.zs)) {
//             let cubes = ((x.1 - x.0) + 1) * ((y.1 - y.0) + 1) * ((z.1 - z.0) + 1);
//             println!("ON Overlap: {}", cubes);
//             self.cubes -= cubes;
//         }

//         println!("ON Total: {}", self.cubes);

//         // second turn_on, check xs for ranges
//     }

//     fn turn_off(&mut self, step: &Step) {
//         let Step { x, y, z } = step;
//         println!("-OFF: {:?}", self.xs);
//         for (min, max) in &mut self.xs {
//             if x.0 >= *min && x.0 <= *max {
//                 *max = x.0;
//             }

//             if x.1 >= *min && x.1 <= *max {
//                 *min = x.1;
//             }
//         }

//         for (min, max) in &mut self.ys {
//             if y.0 >= *min && y.0 <= *max {
//                 *max = y.0;
//             }

//             if y.1 >= *min && y.1 <= *max {
//                 *min = y.1;
//             }
//         }

//         for (min, max) in &mut self.zs {
//             if z.0 >= *min && z.0 <= *max {
//                 *max = z.0;
//             }

//             if z.1 >= *min && z.1 <= *max {
//                 *min = z.1;
//             }
//         }
//         println!("OFF: {:?}", self.xs);



//         for (x, y, z) in izip!(&self.xs, &self.ys, &self.zs) {
//             let mut cubes = ((x.1 - x.0) + 1) * ((y.1 - y.0) + 1) * ((z.1 - z.0) + 1);
//             println!("OFF Cubes: {}", cubes);
//         }

//         println!("OFF Total: {}", self.cubes);


//         // self.merge();

//         // let mut xs = self.xs.clone(); xs.push(*x);
//         // let mut ys = self.xs.clone(); ys.push(*y);
//         // let mut zs = self.xs.clone(); zs.push(*z);

//         // for (x, y, z) in izip!(overlaps(&xs), overlaps(&ys), overlaps(&zs)) {
//         //     let cubes = ((x.1 - x.0) + 1) * ((y.1 - y.0) + 1) * ((z.1 - z.0) + 1);
//         //     println!("OFF Overlap: {}", cubes);
//         //     self.cubes -= cubes;
//         // }

//     }

//     fn merge(&mut self) {
//         merge_ranges(&mut self.xs);
//         merge_ranges(&mut self.ys);
//         merge_ranges(&mut self.zs);
//     }
// }

pub fn part1(input: &String) -> u32 {
    let input = input.lines();
    let mut steps = vec![];
    let region_size = 100;
    let region_half_size = region_size / 2;

    for line in input {
        let mut s = line.split(&[' ', ','][..]);
        steps.push(Step { 
            action: String::from(s.next().unwrap()),
            x: parse_step_axis_into_tuple_and_translate(s.next().unwrap(), region_half_size),
            y: parse_step_axis_into_tuple_and_translate(s.next().unwrap(), region_half_size),
            z: parse_step_axis_into_tuple_and_translate(s.next().unwrap(), region_half_size),
        });
    }

    steps = steps.into_iter().filter(|Step { x, y, z, .. }| {
        x.1 <= region_size && y.1 <= region_size && z.1 <= region_size
    }).collect();

    let size = (region_size + 1) as usize;
    let mut reactor = vec![vec![vec![0u32; size]; size]; size];

    for step in steps {
        for x in step.x.0..=step.x.1 {
            for y in step.y.0..=step.y.1 {
                for z in step.z.0..=step.z.1 {
                    if step.action == "on" {
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

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

fn parse_step_axis_into_tuple_and_translate(data: &str, translation: u32) -> (u32, u32) {
    let mut parts = data[2..].split("..");
    ((parts.next().unwrap().parse::<i32>().unwrap() + translation as i32) as u32,
     (parts.next().unwrap().parse::<i32>().unwrap() + translation as i32) as u32)
}

fn merge_ranges(ranges: &mut Vec<(i32, i32)>) {
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut stack = vec![ranges[0]];

    for range in ranges.iter() {
        let last = stack.last_mut().unwrap();
        if range.0 > last.1 {
            stack.push(*range);
        } else if range.1 > last.1 {
            last.1 = range.1;
        }
    }

    *ranges = stack;
}

fn overlaps(ranges: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // let mut ranges = ranges.iter().map(|&(a, b)| [a, b]).flatten().collect::<Vec<i32>>();
    // ranges.sort();
    let mut ranges = ranges.clone();
    ranges.sort_by(|a, b| a.1.cmp(&b.1));

    let mut overlaps = vec![];
    let mut last = ranges[0];

    for range in ranges.iter().skip(1) {
        if last.1 >= range.0 {
            let opening = std::cmp::max(last.0, range.0);
            let closing = std::cmp::min(last.1, range.1);
            // println!("Overlap ({}-{}): {:?}", opening, closing, i32::abs(opening - closing) + 1);
            overlaps.push((opening, closing));
        }

        last = *range;
    }

    overlaps
}

// fn overlaps(ranges: &Vec<(&&str, &(i32, i32))>) -> Vec<i32> {
//     // let mut ranges = ranges.iter().map(|&(a, b)| [a, b]).flatten().collect::<Vec<i32>>();
//     // ranges.sort();
//     let mut ranges = ranges.clone();
//     ranges.sort_by(|(_, a), (_, b)| a.1.cmp(&b.1));

//     let mut overlaps = vec![];
//     let mut last = (ranges[0].1.0, ranges[0].1.1);
//     let mut last_state = *ranges[0].0;

//     println!("{:?}", &ranges);

//     for (&state, &range) in ranges.iter().skip(1) {
        

//         if last.1 > range.0 {
//             let opening = std::cmp::max(last.0, range.0);
//             let closing = std::cmp::min(last.1, range.1);
//             println!("Overlap ({}-{}): {:?}", opening, closing, i32::abs(opening - closing) + 1);
//             let cubes = i32::abs(opening - closing) + 1;
//             overlaps.push(if last_state == "on" { -cubes } else { -cubes });
//         }

//         last_state = &state;
//         last.0 = range.0;
//         last.1 = range.1;
//     }

//     overlaps
// }