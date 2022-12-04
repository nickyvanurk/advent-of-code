pub fn part1(input: &str) -> u32 {
    input.lines().map(|s| {
        let v: Vec<u32> = s.replace(',', "-").split('-').map(|s| s.parse::<u32>().unwrap()).collect();
        v[0] >= v[2] && v[1] <= v[3] || v[2] >= v[0] && v[3] <= v[1]
    }).filter(|&b| b).count() as u32
}

pub fn part2(input: &str) -> u32 {
    input.lines().map(|s| {
        let v: Vec<u32> = s.replace(',', "-").split('-').map(|s| s.parse::<u32>().unwrap()).collect();
        v[2] <= v[1] && v[3] >= v[0]
    }).filter(|&b| b).count() as u32
}