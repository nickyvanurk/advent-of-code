pub fn part1(input: &String) -> u32 {
    let report: Vec<u32> = input
        .trim()
        .split('\n')
        .map(|s: &str| s.parse::<u32>().unwrap())
        .collect();
    
    let mut previous_depth = report.first().unwrap();
    let mut depth_increased_count = 0;
    for depth in report.iter().skip(1) {
        if depth > previous_depth {
            depth_increased_count += 1;
        }

        previous_depth = depth;
    }

    depth_increased_count
}

pub fn part2(input: &String) -> u32 {
    let report: Vec<u32> = input
        .trim()
        .split('\n')
        .map(|s: &str| s.parse::<u32>().unwrap())
        .collect();

    let mut previous_depth: u32 = report.iter().take(3).sum();
    let mut depth_increased_count = 0;
    for (i, _) in report.iter().skip(1).enumerate() {
        if report.len() - i < 3 {
            break;
        }

        let a = report.iter().nth(i).unwrap();
        let b = report.iter().nth(i + 1).unwrap();
        let c = report.iter().nth(i + 2).unwrap();
        let depth = a + b + c;

        if depth > previous_depth {
            depth_increased_count += 1;
        }

        previous_depth = depth
    }
    
    depth_increased_count
}
