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

    let mut status = waiting_area.tick(false);
    while status.state_changed {
        status = waiting_area.tick(false);
    }

    status.occupied_seats
}

pub fn part2(input: &String) -> u32 {
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

    let mut status = waiting_area.tick(true);
    while status.state_changed {
        status = waiting_area.tick(true);
    }

    status.occupied_seats
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
    fn tick(&mut self, look_ahead: bool) -> Status {
        let mut next = self.cells.clone();
        let mut state_changed = false;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let occupied_neighbors = self.occupied_neighbor_count(row, col, look_ahead);
                let seats_before_moving = match look_ahead {
                    true => 5,
                    false => 4,
                };

                let next_cell = match (cell, occupied_neighbors) {
                    (Cell::Empty, 0) => Cell::Occupied,
                    (Cell::Occupied, count) if count >= seats_before_moving => Cell::Empty,
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

    fn occupied_neighbor_count(&self, row: u8, column: u8, look_ahead: bool) -> u8 {
        let mut count = 0;

        if !look_ahead {
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
        } else {
            let mut seen_dirs: Vec<[i8; 2]> = vec![];
            let mut checked_dirs = 0;
            let mut range = 1;
            let mut row_dirs = vec![-1, 0, 1];
            let mut col_dirs = vec![-1, 0, 1];

            while checked_dirs < 8 {
                for &delta_row in &row_dirs {
                    for &delta_col in &col_dirs {
                        if delta_row == 0 && delta_col == 0 {
                            continue;
                        }

                        let neighbor_row = row as i8 + delta_row;
                        let neighbor_col = column as i8 + delta_col;

                        let prev_delta_row = match delta_row {
                            v if v.is_positive() => delta_row - 1,
                            v if v.is_negative() => delta_row + 1,
                            _ => delta_row,
                        };

                        let prev_delta_col = match delta_col {
                            v if v.is_positive() => delta_col - 1,
                            v if v.is_negative() => delta_col + 1,
                            _ => delta_col,
                        };

                        if neighbor_row.is_negative()
                            || neighbor_row as u8 >= self.height
                            || neighbor_col.is_negative()
                            || neighbor_col as u8 >= self.width
                        {
                            if !seen_dirs.contains(&[prev_delta_row, prev_delta_col]) {
                                checked_dirs += 1;
                            }

                            seen_dirs.push([delta_row, delta_col]);

                            continue;
                        }

                        if range > 1 && seen_dirs.contains(&[prev_delta_row, prev_delta_col]) {
                            seen_dirs.push([delta_row, delta_col]);
                            continue;
                        }

                        let idx = self.get_index(neighbor_row as u8, neighbor_col as u8);
                        let cell = self.cells[idx];

                        if cell == Cell::Empty {
                            seen_dirs.push([delta_row, delta_col]);
                            checked_dirs += 1;
                        }

                        if cell == Cell::Occupied {
                            seen_dirs.push([delta_row, delta_col]);
                            checked_dirs += 1;
                            count += 1;
                        }
                    }
                }

                range += 1;

                row_dirs[0] -= 1;
                row_dirs[2] += 1;
                col_dirs[0] -= 1;
                col_dirs[2] += 1;
            }
        }

        count
    }

    fn get_index(&self, row: u8, column: u8) -> usize {
        (row as u16 * self.width as u16 + column as u16) as usize
    }
}
