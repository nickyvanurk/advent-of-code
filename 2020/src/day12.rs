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

pub fn part2(input: &String) -> u16 {
    let nav_instructions = input
        .lines()
        .map(|line| NavInstruction {
            action: line.chars().nth(0).unwrap(),
            value: line[1..].parse().unwrap(),
        })
        .collect::<Vec<NavInstruction>>();

    let mut ship = Ship::new();
    let mut waypoint = Waypoint {
        position: Point { x: 10, y: 1 },
    };

    for instruction in nav_instructions.iter() {
        if instruction.action != 'F' {
            waypoint.set_from_nav_instruction(&instruction);
            continue;
        }

        for _ in 0..instruction.value {
            ship.move_towards_waypoint(&waypoint);
        }
    }

    ship.get_manhattan_distance()
}

#[derive(Copy, Clone)]
struct Point {
    x: i16,
    y: i16,
}

struct NavInstruction {
    action: char,
    value: u16,
}

struct Ship {
    position: Point,
    rotation: u8,
}

impl Ship {
    fn new() -> Ship {
        Ship {
            position: Point { x: 0, y: 0 },
            rotation: 0,
        }
    }

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

    fn move_towards_waypoint(&mut self, waypoint: &Waypoint) {
        self.position.x += waypoint.position.x;
        self.position.y += waypoint.position.y;
    }

    fn get_manhattan_distance(&self) -> u16 {
        (self.position.x.abs() + self.position.y.abs()) as u16
    }
}

struct Waypoint {
    position: Point,
}

impl Waypoint {
    fn set_from_nav_instruction(&mut self, instruction: &NavInstruction) {
        match instruction.action {
            'N' => self.position.y += instruction.value as i16,
            'S' => self.position.y -= instruction.value as i16,
            'E' => self.position.x += instruction.value as i16,
            'W' => self.position.x -= instruction.value as i16,
            'L' => self.turn(-(instruction.value as i16)),
            'R' => self.turn(instruction.value as i16),
            _ => unreachable!(),
        }
    }

    fn turn(&mut self, degrees: i16) {
        let rotation = degrees / 90;
        let direction = if rotation.is_positive() { 1 } else { -1 };

        for _ in 0..rotation.abs() {
            let temp = self.position.y;
            self.position.y = -self.position.x * direction;
            self.position.x = temp * direction;
        }
    }
}
