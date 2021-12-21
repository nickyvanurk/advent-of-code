pub fn part1(input: &String) -> u32 {
    let mut input = input.lines();
    let mut players = vec![
        Player::new(input.next().unwrap().chars().last().unwrap().to_digit(10).unwrap()),
        Player::new(input.next().unwrap().chars().last().unwrap().to_digit(10).unwrap())
    ];
    let rolls = 10000;
    let mut die = 1..=rolls;

    for i in 0..(rolls / 3) {
        let player_idx = i % 2;

        for _ in 0..3 {
            let roll = die.next().unwrap() as u32;
            players[player_idx].move_places(roll);
        }

        let score = players[player_idx].add_score();

        if score >= 1000 {
            return players[(i + 1) % 2].score * ((i as u32 * 3) + 3);
        }
    }

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

#[derive(Debug)]
struct Player {
    place: u32,
    score: u32
}

impl Player {
    fn new(start: u32) -> Self {
        Self { place: start, score: 0 }
    }

    fn move_places(&mut self, places: u32) {
        self.place = ((self.place - 1 + places) % 10) + 1;
    }

    fn add_score(&mut self) -> u32 {
        self.score += self.place;
        self.score
    }
}