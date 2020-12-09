pub fn part1(input: &String) -> u32 {
    let instructions = parse_instructions(&input);
    let (success, accumulator) = run_boot_code(&instructions);

    if !success {
        return accumulator as u32;
    }

    0
}

pub fn part2(input: &String) -> u32 {
    let mut instructions = parse_instructions(&input);

    for i in 0..instructions.len() {
        if !vec!["jmp", "nop"].contains(&instructions[i].operation) {
            continue;
        }

        switch_operations(&"jmp", &"nop", &mut instructions[i]);

        let (success, accumulator) = run_boot_code(&instructions);

        if success {
            return accumulator as u32;
        }

        switch_operations(&"jmp", &"nop", &mut instructions[i]);
    }

    0
}

fn switch_operations<'a>(
    operation_a: &'a str,
    operation_b: &'a str,
    instruction: &mut Instruction<'a>,
) {
    if instruction.operation == operation_a {
        instruction.operation = operation_b;
    } else if instruction.operation == operation_b {
        instruction.operation = operation_a;
    }
}

fn run_boot_code(instructions: &Vec<Instruction<'_>>) -> (bool, i32) {
    let mut visited = vec![false; instructions.len()];
    let mut instruction_idx = 0;
    let mut accumulator = 0;

    while instruction_idx != instructions.len() {
        let instruction = &instructions[instruction_idx];
        let instruction_executed = &mut visited[instruction_idx];

        if *instruction_executed {
            return (false, accumulator);
        }

        *instruction_executed = true;

        if instruction.operation == "acc" {
            accumulator += instruction.argument
        } else if instruction.operation == "jmp" {
            instruction_idx = ((instruction_idx as i32) + instruction.argument - 1) as usize;
        }

        instruction_idx += 1;
    }

    (true, accumulator)
}

fn parse_instructions(input: &String) -> Vec<Instruction<'_>> {
    input
        .lines()
        .map(|line| Instruction {
            operation: &line[..3],
            argument: line[4..].parse().unwrap(),
        })
        .collect()
}

#[derive(Copy, Clone)]
struct Instruction<'a> {
    operation: &'a str,
    argument: i32,
}
