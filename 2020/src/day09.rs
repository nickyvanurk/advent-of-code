use std::collections::HashSet;

pub fn part1(input: &String) -> u64 {
    let numbers = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let preamble = 25;
    let mut sum_idx = 25;
    let mut set: HashSet<i64> = HashSet::new();

    let mut sum;
    let mut current;

    for i in 0..numbers.len() {
        sum = numbers[sum_idx];
        current = numbers[i];

        if i == sum_idx {
            set.remove(&numbers[((sum_idx as i64) - preamble) as usize]);
            set.insert(sum);
            sum_idx += 1;
        }

        if set.len() == preamble as usize {
            let mut found = false;

            for j in (i - preamble as usize)..i {
                if set.contains(&(sum - numbers[j])) {
                    found = true;
                    break;
                }
            }

            if !found {
                return sum as u64;
            }
        }

        set.insert(current);
    }

    0
}

pub fn part2(input: &String) -> u64 {
    let numbers = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut set: HashSet<i64> = HashSet::new();
    let mut sum: i64;
    let mut min_idx = 0;
    let target = 400480901;

    for i in 0..numbers.len() {
        set.insert(numbers[i]);

        while set.iter().sum::<i64>() > target {
            set.remove(&numbers[min_idx]);
            min_idx += 1;
        }

        sum = set.iter().sum();

        if sum == target {
            return (set.iter().min().unwrap() + set.iter().max().unwrap()) as u64;
        }
    }

    0
}
