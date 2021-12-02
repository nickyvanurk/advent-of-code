pub fn part1(input: &String) -> u32 {
    let input = parse_input(input);
    let mut submarine = build_submarine();
    
    for (instruction, value) in input {
        match instruction {
            "forward" => submarine.position += value,
            "up" => submarine.depth -= value,
            "down" => submarine.depth += value,
            _ => println!("Invalid instruction")
        }
    }

    submarine.position * submarine.depth
}

pub fn part2(input: &String) -> u32 {
    let input = parse_input(input);
    let mut submarine = build_submarine();
    
    for (instruction, value) in input {
        match instruction {
            "forward" => {
                submarine.position += value;
                submarine.depth += submarine.aim * value;
            },
            "up" => submarine.aim -= value,
            "down" => submarine.aim += value,
            _ => println!("Invalid instruction")
        }
    }

    submarine.position * submarine.depth
}

struct Submarine {
    position: u32,
    depth: u32,
    aim: u32,
}

fn parse_input(input: &String) -> Vec<(&str, u32)> {
    input.trim().split('\n').map(|s| {
        let data: Vec<&str> = s.split(' ').collect();
        (data[0], data[1].parse::<u32>().unwrap())
    }).collect()
}

fn build_submarine() -> Submarine {
    Submarine {
        position: 0,
        depth: 0,
        aim: 0,
    }
}
