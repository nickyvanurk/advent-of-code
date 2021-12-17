pub fn part1(input: &String) -> u16 {
    let input = input.trim().replace("target area: ", "").replace("x=", "").replace("y=", "").replace(",", "").replace("-", "")
                           .split(' ').map(|range| range.split("..").map(|s| s.parse::<i16>().unwrap()).collect::<Vec<i16>>())
                           .collect::<Vec<Vec<i16>>>();

    let target_area = TargetArea::new(&input);
    let mut probe = Probe::new();
   
    let velocity = get_highest_arc_velocity(&mut probe, &target_area);
    probe.set_velocity((velocity.x, velocity.y));

    let mut highest_y_position= 0;
    while !target_area.in_bounds(&probe) {
        probe.update();

        // I reversed the y order, negative is up
        if probe.position.y < highest_y_position {
            highest_y_position = probe.position.y;
        }
    }

    i16::unsigned_abs(highest_y_position)
}

fn get_highest_arc_velocity(probe: &mut Probe, target_area: &TargetArea) -> Velocity {
    // let max_x_velocity = get_max_x_velocity(probe, target_area);

    for x_velocity in 1..200 {
        for y_velocity in (1..200).rev() {
            probe.reset();
            probe.set_velocity((x_velocity, -y_velocity));

            while !target_area.is_overshot(&probe) {

                probe.update();

                if target_area.in_bounds(&probe) {
                    probe.reset();
                    return Velocity { x: x_velocity, y: -y_velocity };
                }
            }
        }
    }

    panic!("Should never be reached");
}

// fn get_max_x_velocity(probe: &mut Probe, target_area: &TargetArea) -> i16 {
//     for x_velocity in (1..200).rev() {
//         probe.reset();
//         probe.set_velocity((x_velocity, 0));

//         while !target_area.is_overshot(&probe) {
//             probe.update();

//             if target_area.in_bounds(&probe) {
//                 probe.reset();
//                 return x_velocity;
//             }
//         }
//     }

//     panic!("Should never be reached");
// }

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
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

    fn reset(&mut self) {
        self.position.x = 0;
        self.position.y = 0;
        self.velocity.x = 0;
        self.velocity.y = 0;
    }

    fn set_velocity(&mut self, velocity :(i16, i16)) {
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
        self.in_bounds_x(probe) && self.in_bounds_y(probe)
    }

    fn in_bounds_x(&self, probe: &Probe) -> bool {
        let o = self.position;
        let p = probe.position;
        o.x <= p.x && p.x <= o.x + self.width as i16
    }

    fn in_bounds_y(&self, probe: &Probe) -> bool {
        let o = self.position;
        let p = probe.position;
        o.y <= p.y && p.y <= o.y + self.height as i16
    }
}