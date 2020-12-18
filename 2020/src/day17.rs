pub fn part1(input: &String) -> u16 {
    let size = 25;
    let cubes_input = input
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '#' => Cube::Active,
            _ => Cube::Inactive,
        })
        .collect::<Vec<Cube>>();

    let mut grid = Grid::new(&cubes_input, size);

    for _ in 0..6 {
        grid.tick();
        grid.render();
    }

    grid.get_active_cubes()
}

pub fn part2(input: &String) -> u32 {
    0
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cube {
    Inactive = 0,
    Active = 1,
}

struct Grid {
    size: u8,
    cubes: Vec<Cube>,
}

impl Grid {
    fn new(cubes_input: &[Cube], size: u8) -> Grid {
        let mut cubes = Vec::<Cube>::new();
        let cubes_input_size = (cubes_input.len() as f64).sqrt() as u8;

        for _ in 0..size {
            for _ in 0..size {
                for _ in 0..size {
                    cubes.push(Cube::Inactive);
                }
            }
        }

        let mut grid = Grid { size, cubes };

        for y in 0..cubes_input_size {
            for x in 0..cubes_input_size {
                let offset = (size - cubes_input_size) / 2;
                let idx_input = (x as u16 + cubes_input_size as u16 * y as u16) as usize;
                let idx_grid = grid.get_index(offset + x as u8, offset + y as u8, size / 2);

                grid.cubes[idx_grid] = cubes_input[idx_input];
            }
        }

        grid
    }

    fn tick(&mut self) {
        let mut next = self.cubes.clone();

        for z in 0..self.size {
            for y in 0..self.size {
                for x in 0..self.size {
                    let idx = self.get_index(x, y, z);
                    let cube = self.cubes[idx];
                    let occupied_neighbors = self.occupied_neighbor_count(x, y, z);

                    let next_cube = match (cube, occupied_neighbors) {
                        (Cube::Active, count) if count != 2 && count != 3 => Cube::Inactive,
                        (Cube::Inactive, count) if count == 3 => Cube::Active,
                        (otherwise, _) => otherwise,
                    };

                    next[idx] = next_cube;
                }
            }
        }

        self.cubes = next;
    }

    fn render(&self) {
        for z in 0..self.size {
            for y in 0..self.size {
                for x in 0..self.size {
                    let idx = self.get_index(x, y, z);

                    match self.cubes[idx] {
                        Cube::Active => print!("#"),
                        _ => print!("."),
                    }
                }

                println!("");
            }

            println!("");
        }
    }

    fn occupied_neighbor_count(&self, x: u8, y: u8, z: u8) -> u8 {
        let mut count = 0;

        for delta_z in -1..=1 {
            for delta_y in -1..=1 {
                for delta_x in -1..=1 {
                    if delta_x == 0 && delta_y == 0 && delta_z == 0 {
                        continue;
                    }

                    let neighbor_x = x as i8 + delta_x;
                    let neighbor_y = y as i8 + delta_y;
                    let neighbor_z = z as i8 + delta_z;

                    if neighbor_x.is_negative()
                        || neighbor_x as u8 == self.size
                        || neighbor_y.is_negative()
                        || neighbor_y as u8 == self.size
                        || neighbor_z.is_negative()
                        || neighbor_z as u8 == self.size
                    {
                        continue;
                    }

                    let idx = self.get_index(neighbor_x as u8, neighbor_y as u8, neighbor_z as u8);
                    let cube = self.cubes[idx];

                    if cube == Cube::Active {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn get_index(&self, x: u8, y: u8, z: u8) -> usize {
        (x as u16 + self.size as u16 * (y as u16 + self.size as u16 * z as u16)) as usize
    }

    fn get_active_cubes(&self) -> u16 {
        self.cubes
            .iter()
            .filter(|&cube| *cube == Cube::Active)
            .count() as u16
    }
}
