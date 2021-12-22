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

pub fn part1(input: &String) -> u32 {
    let input = input.lines();

    let mut xs = vec![];
    let mut ys = vec![];
    let mut zs = vec![];

    for line in input {
        let mut s = line.split(&[' ', ','][..]);

        let state = s.next().unwrap();
        let x = parse_step_axis_into_tuple(s.next().unwrap());
        let y = parse_step_axis_into_tuple(s.next().unwrap());
        let z = parse_step_axis_into_tuple(s.next().unwrap());

        if state == "off" {
            xs.push(x);
            ys.push(z);
            zs.push(y);
        }
    }

    merge_ranges(&mut xs);
    merge_ranges(&mut ys);
    merge_ranges(&mut zs);
    
    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

fn parse_step_axis_into_tuple(data: &str) -> (i32, i32) {
    let mut parts = data[2..].split("..");
    (parts.next().unwrap().parse::<i32>().unwrap(), parts.next().unwrap().parse::<i32>().unwrap())
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