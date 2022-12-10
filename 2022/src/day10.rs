pub fn part1(input: &str) -> i32 {
    let mut instructions: Vec<Vec<&str>> = input.lines().map(|s| s.split_whitespace().collect()).rev().collect();
    let mut register = 1;
    let mut cycles = 0;
    let mut signal_strength = 0;

    let mut temp = 0;
    loop {
        cycles += 1;

        if vec![20, 60, 100, 140, 180, 220].contains(&cycles) {
            signal_strength += cycles * register;
        }
        
        register += temp;
        let ready = temp == 0;
        temp = 0;

        if ready {
            let instruction = instructions.pop().unwrap();

            match instruction[0] {
                "noop" => (),
                "addx" => temp = instruction[1].parse::<i32>().unwrap(),
                _ => (),
            }
        }
    
        if instructions.len() == 0 && temp == 0 {
            break;
        }
    }

    signal_strength
}

pub fn part2(input: &str) {
    let mut instructions: Vec<Vec<&str>> = input.lines().map(|s| s.split_whitespace().collect()).rev().collect();
    let mut register = 1;
    let mut cycles = 0;
    let mut pixels: Vec<char> = "###.....................................".chars().collect();

    let mut temp = 0;
    loop {
        cycles += 1;

        print!("{}", if pixels[(cycles - 1) as usize % 40] == '#' { '#' } else { '.' });
        if vec![40, 80, 120, 160, 200, 240].contains(&cycles) {
            println!();
        }

        register += temp;
        let ready = temp == 0;
        temp = 0;

        if ready {
            let instruction = instructions.pop().unwrap();
            match instruction[0] {
                "noop" => (),
                "addx" => temp = instruction[1].parse::<i32>().unwrap(),
                _ => (),
            }
        }

        pixels = vec!['.'; 40];
        if register >= -1 && register <= 40 {
            for index in vec![register - 1, register, register + 1] {
                if index >= 0 && index <= 39 {
                    pixels[index as usize] = '#';
                }
            }
        }

        if instructions.len() == 0 && temp == 0 {
            break;
        }
    }
    
    println!();
}