pub fn part1(input: &String) -> u32 {
    let random_numbers = parse_random_numbers(input);
    let mut boards = parse_boards(input);

    for random_number in random_numbers {
        for board in &mut boards {
            if board.mark(random_number) {
                if board.is_bingo() {
                    return board.sum_unmarked() as u32 * random_number as u32;
                }
            }
        }
    }

    0
}

pub fn part2(input: &String) -> u32 {
    let random_numbers = parse_random_numbers(input);
    let mut boards = parse_boards(input);
    let mut score = 0;

    for random_number in random_numbers {
        for board in &mut boards {
            if board.mark(random_number) {
                if !board.has_bingo() && board.is_bingo() {
                    score = board.sum_unmarked() as u32 * random_number as u32;
                }
            }
        }
    }

    score
}

fn parse_random_numbers(input: &String) -> Vec<u8> {
    input.trim().split('\n').nth(0).unwrap().split(',').map(|s| s.parse::<u8>().unwrap()).collect()
}

fn parse_boards(input: &String) -> Vec<Board> {
    let data: Vec<&str> = input.trim().split('\n').filter(|s| !s.is_empty()).skip(1).collect();
    let size = 5;
    let mut boards = vec![];

    for board_data in data.chunks(size as usize) {
        let mut numbers: Vec<u8> = vec![];
        for row in board_data {
            numbers.append(&mut row.split_whitespace().map(|s| s.parse::<u8>().unwrap()).collect());
        }

        boards.push(Board::new(size, size, numbers));
    }

    boards
}

#[derive(Debug)]
struct Board {
    width: u8,
    height: u8,
    data: Vec<u8>,
    marked: u32,
    has_bingo: bool,
}

impl Board {
    fn new(width: u8, height: u8, data: Vec<u8>) -> Board {
        Board {
            width,
            height,
            data,
            marked: 0,
            has_bingo: false
        }
    }

    fn has_bingo(&self) -> bool {
        self.has_bingo
    }

    fn is_bingo(&mut self) -> bool {
        let mut row = vec![];
        let mut col = vec![];

        for y in 0..self.height {
            for x in 0..self.width {
                row.push(if self.is_marked((y * self.width + x) as usize) { 1 } else { 0 });
            }   

            if self.is_all_marked(&row) {
                self.has_bingo = true;
                break;
            }

            row.clear();
        }

        for x in 0..self.width {
            for y in 0..self.height {
                col.push(if self.is_marked((y * self.width + x) as usize) { 1 } else { 0 });
            }   

            if self.is_all_marked(&col) {
                self.has_bingo = true;
                break;
            }

            col.clear();
        }

        self.has_bingo
    }

    fn is_all_marked(&self, numbers: &Vec<u8>) -> bool {
        numbers.iter().fold(0, |acc, &n| acc + n) == self.width
    }
    
    fn mark(&mut self, number: u8) -> bool {
       return match self.find_number(number) {
            Some(indexes) => {
                for i in indexes {
                    self.marked = self.marked | 1 << i;
                }

                true
            },
            None => false,
        };
    }

    fn find_number(&self, number:u8) -> Option<Vec<usize>> {
        let mut indexes = vec![];

        for (i, &n) in self.data.iter().enumerate() {
            if n == number {
                indexes.push(i);
            }
        }

        if indexes.len() > 0 { Some(indexes) } else { None }
    }

    fn sum_unmarked(&self) -> u16 {
        self.sum() - self.sum_marked()
    }

    fn sum_marked(&self) -> u16 {
        self.data.iter().enumerate().fold(0, |acc, (i, &n)| acc + (if self.is_marked(i) { n as u16 } else { 0 })) as u16
    }

    fn sum(&self) -> u16 {
        self.data.iter().fold(0, |acc, &n| acc + n as u16)
    }

    fn is_marked(&self, idx: usize) -> bool {
        self.marked & 1 << idx != 0
    }
}