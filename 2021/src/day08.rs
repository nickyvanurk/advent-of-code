pub fn part1(input: &String) -> u32 {
    let input: Vec<Vec<&str>> = input.lines().map(|s| { s.split(' ').filter(|s| *s != "|").collect() }).collect();

    let segments = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
    let mut counter = 0;

    for line in input {
        for i in 10..line.len() {
            let digit = line[i];

            if digit.chars().count() == segments[1] ||
               digit.chars().count() == segments[4] ||
               digit.chars().count() == segments[7] ||
               digit.chars().count() == segments[8] {
                counter += 1;
            }
        }
    }

    counter
}

pub fn part2(input: &String) -> u32 {
    let input: Vec<Vec<&str>> = input.lines().map(|s| { s.split(' ').filter(|s| *s != "|").collect() }).collect();

    0
}
