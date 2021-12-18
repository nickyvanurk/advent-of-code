use std::collections::VecDeque;

pub fn part1(input: &String) -> u32 {
    let input = input.lines();

    for line in input {
        if explode(&line) {
            println!("Explosion!");
        }
    }

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

fn explode(line: &str) -> bool {
    let mut depth = 1;
    let mut numbers: VecDeque<u8> = VecDeque::new();
    let mut brackets = vec!['['];

    let mut last_character = '[';
    let mut to_add = 0;

    let mut exploded = false;

    for character in line.chars().skip(1) {
        match character {
            '[' => {
                depth += 1;
                brackets.push(character);
            }
            ']' => {
                depth -= 1;
                if to_add == 0 {
                    brackets.push(character);
                }
            }
            '0'..='9' => {
                let number = character.to_digit(10).unwrap() as u8 + to_add;
                to_add = 0;

                if last_character == ',' && depth > 4 {
                    let last_number = numbers.pop_back().unwrap();

                    if let Some(previous_number) = numbers.back_mut() {
                        *previous_number += last_number;
                    }

                    brackets.pop();
                    numbers.push_back(0);

                    to_add = number;
                    exploded = true;
                } else {
                    numbers.push_back(number);
                }
            }
            _ => (),
        }

        last_character = character;
    }

    println!("{:?}", line);
    println!("{:?}", brackets);
    println!("{:?}", numbers);

    let mut previous_bracket = '[';
    let mut new_line = String::new();
    for (i, &bracket) in brackets.iter().enumerate() {
        match bracket {
            '[' => {
                if previous_bracket != bracket {
                    new_line.push(',')
                }

                new_line.push(bracket);

                if let Some(next_bracket) = brackets.iter().nth(i + 1) {
                    if bracket != *next_bracket {
                        new_line = format!("{}{}", new_line, numbers.pop_front().unwrap());
                        new_line.push(',');
                    }
                }
            }
            ']' => {
                println!("p {}", previous_bracket);
                println!("c {}", bracket);
                if numbers.len() >= 2 {
                    if previous_bracket != bracket {
                        new_line = format!("{}{}", new_line, numbers.pop_front().unwrap());
                        new_line.push(']');
                    } else {
                        new_line.push(',');
                        new_line = format!("{}{}", new_line, numbers.pop_front().unwrap());
                        new_line.push(']');
                    }
                }
            }
            _ => (),
        }

        previous_bracket = bracket;
    }

    println!("{:?}", new_line);

    exploded
}
