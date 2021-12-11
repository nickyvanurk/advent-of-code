pub fn part1(input: &String) -> u32 {
    let mut input: Vec<u32> = input.replace('\n', "").chars().map(|s| s.to_digit(10).unwrap()).collect();
    let size = 10;

    let mut total_flashes = 0;

    for _step in 1..=100 {
        for y in 0..size {
            for x in 0..size {
                input[y * size + x] += 1;
            }
        }

        total_flashes += count_flashes(&mut input, size);
    }

    total_flashes
}

pub fn part2(input: &String) -> u32 {
    let mut input: Vec<u32> = input.replace('\n', "").chars().map(|s| s.to_digit(10).unwrap()).collect();
    let size = 10;
    
    let total_octopuses = (size*size) as u32;

    for step in 1..=500 {
        for y in 0..size {
            for x in 0..size {
                input[y * size + x] += 1;
            }
        }

        if count_flashes(&mut input, size) == total_octopuses {
            return step;
        }
    }

    0
}

fn count_flashes(input: &mut Vec<u32>, size: usize) -> u32 {
    let mut flashes = 0;
    let max_idx = size - 1;

    for y in 0..size {
        for x in 0..size {
            let idx = y * size + x;

            if input[idx] >= 10 {
                input[idx] = 0;
                flashes += 1;

                if x > 0 && !flashing(input[idx - 1]) { input[idx - 1] += 1; }                                      // W
                if x < max_idx && !flashing(input[idx + 1]) { input[idx + 1] += 1; }                                // E

                if y > 0 && !flashing(input[idx - size]) { input[idx - size] += 1; }                                // N
                if y < max_idx && !flashing(input[idx + size]) { input[idx + size] += 1; }                          // S

                if x > 0 && y > 0 && !flashing(input[idx - size - 1]) { input[idx - size - 1] += 1; }               // NW
                if x > 0 && y < max_idx && !flashing(input[idx + size - 1]) { input[idx + size - 1] += 1; }         // SW

                if x < max_idx && y > 0 && !flashing(input[idx - size + 1]) { input[idx - size + 1] += 1; }         // NE
                if x < max_idx  && y < max_idx && !flashing(input[idx + size + 1]) { input[idx + size + 1] += 1; }  // SE

                flashes += count_flashes(input, size);
            }
        }
    }

    flashes
}

fn flashing(num: u32) -> bool {
    num == 0
}