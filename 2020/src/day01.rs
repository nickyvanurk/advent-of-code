pub fn part1(input: String) {
    let report: Vec<i32> = input
        .trim()
        .split('\n')
        .map(|s: &str| s.parse::<i32>().unwrap())
        .collect();

    let n = report.len();

    for i in 0..n {
        for j in i + 1..n {
            if report[i] + report[j] == 2020 {
                println!("{}", report[i] * report[j]);
                break;
            }
        }
    }
}

pub fn part2(input: String) {
    let report: Vec<i32> = input
        .trim()
        .split('\n')
        .map(|s: &str| s.parse::<i32>().unwrap())
        .collect();

    let n = report.len();

    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if report[i] + report[j] + report[k] == 2020 {
                    println!("{}", report[i] * report[j] * report[k]);
                    break;
                }
            }
        }
    }
}
