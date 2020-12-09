pub fn part1(input: &String) -> u64 {
    let data = parse_data(&input);
    let preamble = 25;

    for sum in preamble..data.len() {
        let set = &data[sum - preamble..sum];
        let found_pair = set
            .iter()
            .any(|&number| set.contains(&(((data[sum] as i64) - (number as i64)) as u64)));

        if !found_pair {
            return data[sum];
        }
    }

    0
}

pub fn part2(input: &String) -> u64 {
    let data = parse_data(&input);
    let mut tail = 0;
    let mut head = 1;
    let mut sum = data[0];
    let target = 400480901;

    while head < data.len() {
        if sum < target {
            sum += data[head];
            head += 1;
        } else if sum > target {
            sum -= data[tail];
            tail += 1;
        } else {
            let set = &data[tail..head + 1];
            return set.iter().min().unwrap() + set.iter().max().unwrap();
        }
    }

    0
}

fn parse_data(input: &String) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}
