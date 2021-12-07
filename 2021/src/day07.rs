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

    let mean = input.iter().sum::<u32>() / input.len() as u32;
    let mut fuel = 0;

    for &x_position in &input {
        let n = i32::abs(x_position as i32 - mean as i32) as u32;
    
        for i in 1..=n {
            fuel += i;
        }
    }

    fuel
}