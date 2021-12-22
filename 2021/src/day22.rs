// Reboot reactor

// Reactor:
// 3D grid of cubes (octree)

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

    for line in input {
        let mut s = line.split(&[' ', ','][..]);

        let state = if s.next().unwrap() == "on" { 1 } else { 0 };
        let x = parse_step_axis_into_tuple(s.next().unwrap());
        let y = parse_step_axis_into_tuple(s.next().unwrap());
        let z = parse_step_axis_into_tuple(s.next().unwrap());
        
        println!("{:?}", state);
        println!("{:?}", x);
        println!("{:?}", y);
        println!("{:?}", z);
    }

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