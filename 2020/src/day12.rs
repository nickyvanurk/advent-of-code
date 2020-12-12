pub fn part1(input: &String) -> u16 {
    let nav_instructions = input
        .lines()
        .map(|line| NavInstruction {
            action: line.chars().nth(0).unwrap(),
            value: line[1..].parse().unwrap(),
        })
        .collect::<Vec<NavInstruction>>();

    let mut ship = Ship {
        position: Point { x: 0, y: 0 },
        rotation: 1,
    };

    for instruction in nav_instructions.iter() {
        ship.execute_nav_instruction(instruction);
    }

    ship.get_manhattan_distance()
}

pub fn part2(input: &String) -> u32 {
    0
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct NavInstruction {
    action: char,
    value: u16,
}

struct Ship {
    position: Point,
    rotation: u8,
}

impl Ship {
    fn execute_nav_instruction(&mut self, instruction: &NavInstruction) {
        match instruction.action {
            'N' => self.position.y -= instruction.value as i16,
            'S' => self.position.y += instruction.value as i16,
            'E' => self.position.x += instruction.value as i16,
            'W' => self.position.x -= instruction.value as i16,
            'L' => self.turn(-(instruction.value as i16)),
            'R' => self.turn(instruction.value as i16),
            'F' => self.move_forward(instruction.value),
            _ => unreachable!(),
        }
    }

    fn turn(&mut self, degrees: i16) {
        self.rotation = (self.rotation as i8 + (degrees / 90) as i8) as u8 % 4;
    }

    fn move_forward(&mut self, distance: u16) {
        match self.rotation {
            0 => self.position.y -= distance as i16,
            1 => self.position.x += distance as i16,
            2 => self.position.y += distance as i16,
            3 => self.position.x -= distance as i16,
            _ => unreachable!(),
        }
    }

    fn get_manhattan_distance(&self) -> u16 {
        (self.position.x.abs() + self.position.y.abs()) as u16
    }
}
