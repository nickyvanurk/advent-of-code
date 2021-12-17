// Note: I reversed the y order, negative is up

pub fn part1(input: &String) -> u16 {
    let input = input.trim().replace("target area: ", "").replace("x=", "").replace("y=", "").replace(",", "").replace("-", "")
                           .split(' ').map(|range| range.split("..").map(|s| s.parse::<i16>().unwrap()).collect::<Vec<i16>>())
                           .collect::<Vec<Vec<i16>>>();

    let target_area = TargetArea::new(&input);
    let mut probe = Probe::new();
   
    for x_velocity in 1..200 {
        for y_velocity in (1..200).rev() {
            let mut highest_y_position= 0;

            probe.reset_with_velocity((x_velocity, -y_velocity));

            while !target_area.is_overshot(&probe) {
                probe.update();

                if probe.position.y < highest_y_position {
                    highest_y_position = probe.position.y;
                }

                if target_area.in_bounds(&probe) {
                    return i16::unsigned_abs(highest_y_position);
                }
            }
        }
    }

    panic!("Should never be reached");
}

pub fn part2(input: &String) -> u16 {
    let input = input.trim().replace("target area: ", "").replace("x=", "").replace("y=", "").replace(",", "").replace("-", "")
                           .split(' ').map(|range| range.split("..").map(|s| s.parse::<i16>().unwrap()).collect::<Vec<i16>>())
                           .collect::<Vec<Vec<i16>>>();

    let target_area = TargetArea::new(&input);
    let mut probe = Probe::new();
   
    let mut total_valid_velocities = 0;

    for x_velocity in 1..200 {
        for y_velocity in (-200..200).rev() {
            probe.reset_with_velocity((x_velocity, -y_velocity));

            while !target_area.is_overshot(&probe) {
                probe.update();

                if target_area.in_bounds(&probe) {
                    total_valid_velocities += 1;
                    break;
                }
            }
        }
    }
    
    total_valid_velocities
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug, Copy, Clone)]
struct Velocity {
    x: i16,
    y: i16,
}

#[derive(Debug, Copy, Clone)]
struct Probe {
    position: Point,
    velocity: Velocity,
}

impl Probe {
    fn new() -> Probe {
        Probe {
            position: Point { x: 0, y: 0 },
            velocity: Velocity { x: 0, y: 0 },
        }
    }

    fn reset_with_velocity(&mut self, velocity :(i16, i16)) {
        self.position.x = 0;
        self.position.y = 0;
        self.velocity.x = velocity.0;
        self.velocity.y = velocity.1;
    }

    fn update(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;

        // drag
        if self.velocity.x > 0 { self.velocity.x -= 1 };
        if self.velocity.x < 0 { self.velocity.x += 1 };

        // gravity
        self.velocity.y += 1;
    }
}

#[derive(Debug)]
struct TargetArea {
    position: Point,
    width: u16,
    height: u16,
}

impl TargetArea {
    fn new(input :&Vec<Vec<i16>>) -> TargetArea {
        TargetArea {
            position: Point { x: input[0][0], y: input[1][1] },
            width: i16::unsigned_abs(input[0][0] - input[0][1]),
            height: i16::unsigned_abs(input[1][0] - input[1][1]),
        }
    }

    fn is_overshot(&self, probe: &Probe) -> bool {
        let o = self.position;
        let p = probe.position;
        p.x > o.x + self.width as i16 || p.y > o.y + self.height as i16
    }

    fn in_bounds(&self, probe: &Probe) -> bool {
        let o = self.position;
        let p = probe.position;
        o.x <= p.x && p.x <= o.x + self.width as i16 &&
        o.y <= p.y && p.y <= o.y + self.height as i16
    }
}