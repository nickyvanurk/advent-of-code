struct Submarine {
    position: u32,
    depth: u32,
    aim: u32,
}

pub fn part1(input: &String) -> u32 {
    let input: Vec<(&str, u32)> = input.trim().split('\n').map(|s| {
        let data: Vec<&str> = s.split(' ').collect();
        (data[0], data[1].parse::<u32>().unwrap())
    }).collect();

    let mut submarine = Submarine {
        position: 0,
        depth: 0,
        aim: 0,
    };
    
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
    let input: Vec<(&str, u32)> = input.trim().split('\n').map(|s| {
        let data: Vec<&str> = s.split(' ').collect();
        (data[0], data[1].parse::<u32>().unwrap())
    }).collect();

    let mut submarine = Submarine {
        position: 0,
        depth: 0,
        aim: 0,
    };
    
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
