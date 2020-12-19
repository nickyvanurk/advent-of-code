pub fn part1(input: &String) -> u64 {
    let mut homework = input.lines();
    let mut sum = 0;

    while let Some(line) = homework.next() {
        sum += calculate_homework(&line);
    }

    sum
}

pub fn part2(input: &String) -> u32 {
    0
}

fn calculate_homework(line: &str) -> u64 {
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
                let result = calculate_expression(slice).to_string();

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

    calculate_expression(&homework)
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
