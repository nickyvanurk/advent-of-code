pub fn part1(input: &str) -> String {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = get_stacks(input[0]);
    let instructions = get_instructions(input[1]);
    run_instructions(&instructions, &mut stacks, false);
    get_crates_on_top(&stacks)
}

pub fn part2(input: &str) -> String {
    let input: Vec<&str> = input.split("\n\n").collect();
    let mut stacks = get_stacks(input[0]);
    let instructions = get_instructions(input[1]);
    run_instructions(&instructions, &mut stacks, true);
    get_crates_on_top(&stacks)
}

fn get_crates_on_top(stacks: &Vec<Vec<char>>) -> String {
    stacks.iter().map(|stack| *stack.last().unwrap()).collect()
}

fn run_instructions(instructions: &Vec<(usize, usize, usize)>, stacks: &mut Vec<Vec<char>>, multiple_blocks_at_once: bool) {
    for &(take, from, to) in instructions {
        let new_len = stacks[from].len() - take;
        let mut elements = if multiple_blocks_at_once {
            stacks[from].split_off(new_len)
        } else {
            stacks[from].split_off(new_len).into_iter().rev().collect()
        };
        stacks[to].append(&mut elements);
    }
}

fn get_instructions(input: &str) -> Vec<(usize, usize, usize)> {
    input.lines().map(|s| {
        let instructions: Vec<usize> = s.split(' ').filter(|word| word.parse::<usize>().is_ok())
            .map(|c| c.parse::<usize>().unwrap()).collect();
        (instructions[0], instructions[1] - 1, instructions[2] - 1)
    }).collect()
}

fn get_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stack_numbers_indexes = vec![];
    for (idx, c) in input.lines().last().unwrap().chars().enumerate() {
        if c != ' ' {
            stack_numbers_indexes.push(idx);
        }
    }

    let mut stacks: Vec<Vec<char>> = vec![vec![]; stack_numbers_indexes.len()];
    for line in input.lines().rev().skip(1) {
        for (idx, &index) in stack_numbers_indexes.iter().enumerate() {
            let c = line.chars().nth(index).unwrap();
            if c != ' ' {
                stacks[idx].push(c);
            }
        }
    }

    stacks
}