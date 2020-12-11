pub fn part1(input: &String) -> u32 {
    let grid = input
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '#' => Cell::Occupied,
            'L' => Cell::Empty,
            _ => Cell::Floor,
        })
        .collect::<Vec<Cell>>();

    let height = input.as_bytes().iter().filter(|&&c| c == b'\n').count() as u8;
    let mut waiting_area = WaitingArea {
        width: (grid.len() as u16 / height as u16) as u8,
        height,
        cells: grid,
    };

    let mut status = waiting_area.tick();
    while status.state_changed {
        status = waiting_area.tick();
    }

    status.occupied_seats
}

pub fn part2(input: &String) -> u32 {
    0
}

#[derive(Debug)]
struct Status {
    state_changed: bool,
    occupied_seats: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Floor = 0,
    Empty = 1,
    Occupied = 2,
}

struct WaitingArea {
    width: u8,
    height: u8,
    cells: Vec<Cell>,
}

impl WaitingArea {
    fn tick(&mut self) -> Status {
        let mut next = self.cells.clone();
        let mut state_changed = false;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let occupied_neighbors = self.occupied_neighbor_count(row, col);

                let next_cell = match (cell, occupied_neighbors) {
                    (Cell::Empty, 0) => Cell::Occupied,
                    (Cell::Occupied, count) if count >= 4 => Cell::Empty,
                    (otherwise, _) => otherwise,
                };

                if cell != next_cell {
                    state_changed = true;
                }

                next[idx] = next_cell;
            }
        }

        let occupied_seats = next.iter().filter(|&cell| *cell == Cell::Occupied).count() as u32;

        self.cells = next;

        Status {
            state_changed,
            occupied_seats,
        }
    }

    fn occupied_neighbor_count(&self, row: u8, column: u8) -> u8 {
        let mut count = 0;

        for delta_row in -1..=1 {
            for delta_col in -1..=1 {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = row as i8 + delta_row;
                let neighbor_col = column as i8 + delta_col;

                if neighbor_row.is_negative()
                    || neighbor_row as u8 == self.height
                    || neighbor_col.is_negative()
                    || neighbor_col as u8 == self.width
                {
                    continue;
                }

                let idx = self.get_index(neighbor_row as u8, neighbor_col as u8);
                let cell = self.cells[idx];

                if cell == Cell::Occupied {
                    count += 1;
                }
            }
        }

        count
    }

    fn get_index(&self, row: u8, column: u8) -> usize {
        (row as u16 * self.width as u16 + column as u16) as usize
    }
}
