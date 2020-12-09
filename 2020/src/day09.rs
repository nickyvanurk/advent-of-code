use std::collections::HashSet;

pub fn part1(input: &String) -> u64 {
    let numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let preamble = 25;
    let mut sum_idx = 25;
    let mut set: HashSet<u64> = HashSet::new();

    let mut sum;
    let mut current;

    for i in 0..numbers.len() {
        sum = numbers[sum_idx];
        current = numbers[i];

        if i == sum_idx {
            set.remove(&numbers[sum_idx - preamble]);
            set.insert(sum);
            sum_idx += 1;
        }

        if set.len() == preamble {
            let mut found = false;

            for j in (i - preamble)..i {
                if set.contains(&(((sum as i64) - numbers[j] as i64) as u64)) {
                    found = true;
                    break;
                }
            }

            if !found {
                return sum;
            }
        }

        set.insert(current);
    }

    0
}

pub fn part2(input: &String) -> u64 {
    let numbers = input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut set: HashSet<u64> = HashSet::new();
    let mut min_idx = 0;
    let mut sum = 0;
    let target = 400480901;

    for i in 0..numbers.len() {
        set.insert(numbers[i]);
        sum += numbers[i];

        while sum > target {
            set.remove(&numbers[min_idx]);
            sum -= numbers[min_idx];
            min_idx += 1;
        }

        if sum == target {
            return set.iter().min().unwrap() + set.iter().max().unwrap();
        }
    }

    0
}
