pub fn part1(input: &String) -> u32 {
    let random_numbers = parse_random_numbers(input);
    let boards = parse_boards(input);

    println!("{:?}", boards);

    // Sum of all unmarked * last random number drawn

    // score = 0

    // For each random number
    //      For each board
    //          let is_won = board.mark(random_number)
    //           
    //          if is_won
    //              score = board.sum_unmarked() * random_number
    

    // return score
    
    0
}

pub fn part2(input: &String) -> u32 {
    let random_numbers = parse_random_numbers(input);
   
    0
}

fn parse_random_numbers(input: &String) -> Vec<u32> {
    input.trim().split('\n').nth(0).unwrap().split(',').map(|s| s.parse::<u32>().unwrap()).collect()
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
    data: Vec<u8>
}

impl Board {
    fn new(width: u8, height: u8, data: Vec<u8>) -> Board {
        Board {
            width,
            height,
            data,
        }
    }
}