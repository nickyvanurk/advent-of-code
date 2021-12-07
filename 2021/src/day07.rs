pub fn part1(input: &String) -> u32 {
    let input: Vec<u32>= input.trim().split(',').map(|v| v.parse::<u32>().unwrap()).collect();
    let mut least_fuel: u32 = u32::MAX;

    for &value in &input {
        let mut fuel = 0;

        for &x_position in &input {
            fuel += i32::abs(x_position as i32 - value as i32) as u32
        }

        if fuel < least_fuel {
            least_fuel = fuel;
        }
    }

    least_fuel
}

pub fn part2(input: &String) -> u32 {
    let input: Vec<u32>= input.trim().split(',').map(|v| v.parse::<u32>().unwrap()).collect();

    0
}
