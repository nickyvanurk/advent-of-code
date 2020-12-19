pub fn part1(input: &String) -> u64 {
    let mut homework = input.lines();
    let mut sum = 0;

    while let Some(line) = homework.next() {
        sum += calculate_homework(&line, false);
    }

    sum
}

pub fn part2(input: &String) -> u64 {
    let mut homework = input.lines();
    let mut sum = 0;

    while let Some(line) = homework.next() {
        sum += calculate_homework(&line, true);
    }

    sum
}

fn calculate_homework(line: &str, addition_first: bool) -> u64 {
    let chars = line.replace(" ", "").chars().collect::<Vec<char>>();
    let mut homework = Vec::<String>::new();
    let mut open_idxs = Vec::<usize>::new();
    let mut i = 0;

    for c in chars.iter() {
        homework.push(c.to_string());

        match c {
            '(' => open_idxs.push(i),
            ')' => {
                let idx = open_idxs.pop().unwrap();
                let slice = &homework[idx + 1..i];
                let result = if addition_first {
                    calculate_expression_addition_first(slice).to_string()
                } else {
                    calculate_expression(slice).to_string()
                };

                for j in 0..slice.len() + 2 {
                    homework.pop();

                    if j > 0 {
                        i -= 1;
                    }
                }

                homework.push(result);
            }
            _ => (),
        }

        i += 1;
    }

    if addition_first {
        calculate_expression_addition_first(&homework)
    } else {
        calculate_expression(&homework)
    }
}

fn calculate_expression(chars: &[String]) -> u64 {
    let mut answer = 0;

    for (i, c) in chars.iter().enumerate() {
        if c.parse::<u64>().is_ok() && answer == 0 {
            answer = c.parse::<u64>().unwrap();
        } else if c == "+" && chars[i + 1].parse::<u64>().is_ok() {
            answer += chars[i + 1].parse::<u64>().unwrap();
        } else if c == "*" && chars[i + 1].parse::<u64>().is_ok() {
            answer *= chars[i + 1].parse::<u64>().unwrap();
        }
    }

    answer
}

fn calculate_expression_addition_first(chars: &[String]) -> u64 {
    let mut numbers = Vec::<u64>::new();

    for (i, c) in chars.iter().enumerate() {
        if c.parse::<u64>().is_ok() && numbers.is_empty() {
            numbers.push(c.parse::<u64>().unwrap());
        } else if c == "+" && chars[i + 1].parse::<u64>().is_ok() {
            *numbers.last_mut().unwrap() += chars[i + 1].parse::<u64>().unwrap();
        } else if c == "*" && chars[i + 1].parse::<u64>().is_ok() {
            numbers.push(chars[i + 1].parse::<u64>().unwrap());
        }
    }

    numbers.iter().fold(1, |acc, n| acc * n)
}
