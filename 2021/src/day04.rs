pub fn part1(input: &String) -> u32 {
    let random_numbers = parse_random_numbers(input);
    
    0
}

pub fn part2(input: &String) -> u32 {
    let random_numbers = parse_random_numbers(input);
   
    0
}

fn parse_random_numbers(input: &String) -> Vec<u32> {
    input.trim().split('\n').nth(0).unwrap().split(',').map(|s| s.parse::<u32>().unwrap()).collect()
}