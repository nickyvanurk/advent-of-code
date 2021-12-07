pub fn part1(input: &String) -> u32 {
    let mut input: Vec<u32>= input.trim().split(',').map(|v| v.parse::<u32>().unwrap()).collect();

    input.sort();
    let median = input[input.len() / 2];
    let mut fuel = 0;

    for &x_position in &input {
        fuel += i32::abs(x_position as i32 - median as i32) as u32
    }

    fuel
}

pub fn part2(input: &String) -> u32 {
    let input: Vec<u32>= input.trim().split(',').map(|v| v.parse::<u32>().unwrap()).collect();

    let mean = input.iter().sum::<u32>() as f32 / input.len() as f32;
    let mut least_fuel = u32::MAX;

    for value in [mean.floor(), mean.ceil()] {
        let mut fuel = 0;

        for &x_position in &input {
            let n = i32::abs(x_position as i32 - value as i32) as u32;
            fuel += n * (n + 1) / 2;
        }

        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }

    least_fuel
}