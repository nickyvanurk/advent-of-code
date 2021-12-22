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
        x.0 >= 0 && y.0 >= 0 && z.0 >= 0 &&
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

    for step in steps {
        println!("{:?}", step);
    }

    0
}

#[derive(Debug)]
struct Step {
    action: String,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

fn parse_step_axis_into_tuple_and_translate(data: &str, translation: i32) -> (i32, i32) {
    let mut parts = data[2..].split("..");
    (parts.next().unwrap().parse::<i32>().unwrap() + translation,
     parts.next().unwrap().parse::<i32>().unwrap() + translation)
}

fn overlap(step1: &Step, step2: &Step) {

}