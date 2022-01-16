fn main() {
    let puzzle_input = include_str!("../input/day5.txt");
    let mut instructions: Vec<i64> = puzzle_input
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut i = 0;
    while i <= instructions.len() {
        let opcode = extract_opcode(&mut instructions, i);

        match opcode {
            1 => {
                add(&mut instructions, i);
                i += 4;
            },
            2 => {
                multiply(&mut instructions, i);
                i += 4;
            },
            3 => {
                input(&mut instructions, i);
                i += 2;
            },
            4 => {
                output(&mut instructions, i);
                i += 2;
            },
            5 => {
                jump_if_true(&mut instructions, &mut i);
            },
            6 => {
                jump_if_false(&mut instructions, &mut i);
            },
            7 => {
                less_than(&mut instructions, i);
                i += 4;
            },
            8 => {
                equals(&mut instructions, i);
                i += 4;
            },
            9 => break,
            _ => {
                println!("Invalid");
                i += 1;
            }
        }
    }
}

fn extract_opcode(instructions: &mut Vec<i64>, i: usize) -> i64 {
   instructions[i] % 10
}

fn get_param_1_mode(instruction: i64) -> i64 {
    if instruction.to_string().len() < 3 { return 0 };
    instruction.to_string().chars().rev().nth(2).unwrap() as i64 - '0' as i64
}

fn get_param_2_mode(instruction: i64) -> i64 {
    if instruction.to_string().len() < 4 { return 0 };
    instruction.to_string().chars().rev().nth(3).unwrap() as i64 - '0' as i64
}

fn get_param_3_mode(instruction: i64) -> i64 {
    if instruction.to_string().len() < 5 { return 0 };
    instruction.to_string().chars().rev().nth(4).unwrap() as i64 - '0' as i64
}

fn add(instructions: &mut Vec<i64>, i: usize) {
    let param_1_position: usize = match get_param_1_mode(instructions[i]) {
        0 => instructions[i + 1] as usize,
        1 => i + 1,
        _ => 0
    };

    let param_2_position: usize = match get_param_2_mode(instructions[i]) {
        0 => instructions[i + 2] as usize,
        1 => i + 2,
        _ => 0
    };

    let write_location: usize = match get_param_3_mode(instructions[i]) {
        0 => instructions[i + 3] as usize,
        1 => i + 3,
        _ => 0
    };

    instructions[write_location] = instructions[param_1_position] +
                                   instructions[param_2_position];
}

fn multiply(instructions: &mut Vec<i64>, i: usize) {
    let param_1_position: usize = match get_param_1_mode(instructions[i]) {
        0 => instructions[i + 1] as usize,
        1 => i + 1,
        _ => 0
    };

    let param_2_position: usize = match get_param_2_mode(instructions[i]) {
        0 => instructions[i + 2] as usize,
        1 => i + 2,
        _ => 0
    };

    let write_location: usize = match get_param_3_mode(instructions[i]) {
        0 => instructions[i + 3] as usize,
        1 => i + 3,
        _ => 0
    };

    instructions[write_location] = instructions[param_1_position] *
                                   instructions[param_2_position];
}

fn input(instructions: &mut Vec<i64>, i: usize) {
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).unwrap();
    user_input = user_input.trim().to_string();

    let write_location: usize = match get_param_1_mode(instructions[i]) {
        0 => instructions[i + 1] as usize,
        1 => i + 1,
        _ => 0
    };

    instructions[write_location] = user_input.parse::<i64>().unwrap();
}

fn output(instructions: &mut Vec<i64>, i: usize) {
    let read_location: usize = match get_param_1_mode(instructions[i]) {
        0 => instructions[i + 1] as usize,
        1 => i + 1,
        _ => 0
    };

    println!("{}", instructions[read_location]);
}

fn jump_if_true(instructions: &mut Vec<i64>, i: &mut usize) {
    let param_1_position: usize = match get_param_1_mode(instructions[*i]) {
        0 => instructions[*i + 1] as usize,
        1 => *i + 1,
        _ => 0
    };

    let param_2_position: usize = match get_param_2_mode(instructions[*i]) {
        0 => instructions[*i + 2] as usize,
        1 => *i + 2,
        _ => 0
    };

    if instructions[param_1_position] != 0 {
        *i = instructions[param_2_position] as usize;
    } else {
        *i += 3;
    }
}

fn jump_if_false(instructions: &mut Vec<i64>, i: &mut usize) {
    let param_1_position: usize = match get_param_1_mode(instructions[*i]) {
        0 => instructions[*i + 1] as usize,
        1 => *i + 1,
        _ => 0
    };

    let param_2_position: usize = match get_param_2_mode(instructions[*i]) {
        0 => instructions[*i + 2] as usize,
        1 => *i + 2,
        _ => 0
    };

    if instructions[param_1_position] == 0 {
        *i = instructions[param_2_position] as usize;
    } else {
        *i += 3;
    }
}

fn less_than(instructions: &mut Vec<i64>, i: usize) {
    let param_1_position: usize = match get_param_1_mode(instructions[i]) {
        0 => instructions[i + 1] as usize,
        1 => i + 1,
        _ => 0
    };

    let param_2_position: usize = match get_param_2_mode(instructions[i]) {
        0 => instructions[i + 2] as usize,
        1 => i + 2,
        _ => 0
    };

    let write_location: usize = match get_param_3_mode(instructions[i]) {
        0 => instructions[i + 3] as usize,
        1 => i + 3,
        _ => 0
    };

    if instructions[param_1_position] < instructions[param_2_position] {
        instructions[write_location] = 1;
    } else {
        instructions[write_location] = 0;
    }
}

fn equals(instructions: &mut Vec<i64>, i: usize) {
    let param_1_position: usize = match get_param_1_mode(instructions[i]) {
        0 => instructions[i + 1] as usize,
        1 => i + 1,
        _ => 0
    };

    let param_2_position: usize = match get_param_2_mode(instructions[i]) {
        0 => instructions[i + 2] as usize,
        1 => i + 2,
        _ => 0
    };

    let write_location: usize = match get_param_3_mode(instructions[i]) {
        0 => instructions[i + 3] as usize,
        1 => i + 3,
        _ => 0
    };

    if instructions[param_1_position] == instructions[param_2_position] {
        instructions[write_location] = 1;
    } else {
        instructions[write_location] = 0;
    }
}
