use itertools::Itertools;

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
            players[player_idx].move_places(roll as i32);
        }

        let score = players[player_idx].add_score();

        if score >= 1000 {
            return players[(i + 1) % 2].score * ((i as u32 * 3) + 3);
        }
    }

    0
}

pub fn part2(input: &String) -> u64 {
    let mut input = input.lines();
    let mut players = vec![
        Player::new(input.next().unwrap().chars().last().unwrap().to_digit(10).unwrap()),
        Player::new(input.next().unwrap().chars().last().unwrap().to_digit(10).unwrap())
    ];
    let rolls = 10000000;
    let mut die = 1..=rolls;

    let player_idx = 0;

    let mut won = false;
    let mut wins = 0 as u128;
    let mut turns = 0;
    let mut old_scores = [[[0 as i64; 3]; 3]; 3];
    let mut result: u128 = 1;
    let mut branches = 27u128;

    loop {
        let mut all_win = true;
        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    if old_scores[i-1][j-1][k-1] != -1 {
                        all_win = false;
                        break;
                    }
                }
            }
        }

        if all_win {
            break;
        }

        turns += 1;
        let old_place = players[player_idx].place;
        let old_score = players[player_idx].score;

        for i in 1..=3 {
            for j in 1..=3 {
                for k in 1..=3 {
                    if old_scores[i-1][j-1][k-1] == -1 {
                        continue;
                    }

                    let roll = i + j + k;
                    players[player_idx].move_places(roll as i32);
        
                    
                    let score = players[player_idx].add_score();
                    old_scores[i-1][j-1][k-1] += score as i64;

                    // println!("{}, {}, {}, {:?}", i, j, k, old_scores[i-1][j-1][k-1]);

                    if old_scores[i-1][j-1][k-1] >= 21 {
                        old_scores[i-1][j-1][k-1] = -1;
                        wins += 1;
                    }

                    players[player_idx].place = old_place;
                    players[player_idx].score = old_score;
                }
            }
        }

        if true {
            println!("turn: {}, branches goal reached: {}", turns, wins);
            // println!("wins.pow(turn): {}", wins.pow(turns));

            // result *= (27u128.pow(turns) as f32 * (wins as f32) / 27 as f32) as u128;
            if wins > 0 {
                result *= (wins as u128).pow(1);
            } else {
                result *= (27 as u128);
            }
            // println!("result: {}", 27u128.pow(turns));
            
            println!("turns: {}, result: {}", turns, (wins as u128).pow(1));
        }

        wins = 0;
    }


    // 21936950640195795
    // 444356092776315
    // 205891132094649
    println!("{:?}", (27u128.pow(3) as f32 * (((wins as i32) as f32)/27f32)) as u128);
    println!("{:?}", 27u128.pow(3));
    println!("{:?}", 27u128.pow(1) * 27u128.pow(1) * 17u128.pow(1));
    println!("{:?}", 27u128.pow(1) + 27u128.pow(2) + 17u128.pow(3));
    println!("{:?}", result);
       

    0
}

fn turn(place: u64, score: u64, turn_num: u64, roll: u64) -> (u64, u64, u64, u64) {
    for i in 1..=3 {
        for j in 1..=3 {
            for k in 1..=3 {
                let new_place = (((place as i32 - 1 + (i + j + k)) % 10) + 1) as u64;
                let new_score = score + place;
        
                if new_score >= 21 {
                    return (new_place, new_score, turn_num, roll);
                }

                let x = turn(new_place, new_score, turn_num + 1, roll + 1);

                if x.1 > 0 {
                    return (new_place, new_score, turn_num, roll);
                }
            }
        }
    }

    (0, 0, 0, 0)
}

#[derive(Debug)]
struct Player {
    place: u32,
    score: u32,
    universe: u128,
}

impl Player {
    fn new(start: u32) -> Self {
        Self { place: start, score: 0, universe: 1 }
    }

    fn move_places(&mut self, places: i32) {
        self.place = (((self.place as i32 - 1 + places) % 10) + 1) as u32;
    }

    fn add_score(&mut self) -> u32 {
        self.score += self.place;
        self.score
    }
}