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
    let input: Vec<Vec<Vec<char>>> = input.lines().map(|s| { s.split(' ').filter(|s| *s != "|").map(|s| s.chars().collect()).collect() }).collect();

    let mut sum_output = 0;

    for line in input {
        let digit_1: &Vec<char> = line[0..10].iter().filter(|d| d.len() == 2).collect::<Vec<&Vec<char>>>()[0];
        let digit_7: &Vec<char> = line[0..10].iter().filter(|d| d.len() == 3).collect::<Vec<&Vec<char>>>()[0];
        let digit_4: &Vec<char> = line[0..10].iter().filter(|d| d.len() == 4).collect::<Vec<&Vec<char>>>()[0];
        let digits_2_3_5: Vec<&Vec<char>> = line[0..10].iter().filter(|d| d.len() == 5).collect();
        let digits_0_6_9: Vec<&Vec<char>> = line[0..10].iter().filter(|d| d.len() == 6).collect();
        let digit_8: &Vec<char> = line[0..10].iter().filter(|d| d.len() == 7).collect::<Vec<&Vec<char>>>()[0];

        let mut digit_2: &Vec<char> = &vec![];
        for digit in &digits_2_3_5 {
            if digit_8.iter().filter(|c| !digit_4.contains(c)).collect::<Vec<&char>>().iter().all(|c| digit.contains(c)) {
                digit_2 = digit;
                break;
            }
        }

        let mut digit_3: &Vec<char> = &vec![];
        for digit in &digits_2_3_5 {
            if digit_1.iter().all(|c| digit.contains(c)) {
                digit_3 = digit;
                break;
            }
        }

        let mut digit_5: &Vec<char> = &vec![];
        for digit in &digits_2_3_5 {
            if digit_2.iter().all(|c| digit.contains(c)) || digit_3.iter().all(|c| digit.contains(c)){
                continue;
            }

            digit_5 = digit;
        }

        let mut digit_6: &Vec<char> = &vec![];
        for digit in &digits_0_6_9 {
            if !digit_7.iter().all(|c| digit.contains(c)) {
                digit_6 = digit;
                break;
            }
        }

        let mut digit_9: &Vec<char> = &vec![];
        for digit in &digits_0_6_9 {
            if digit_4.iter().all(|c| digit.contains(c)) {
                digit_9 = digit;
                break;
            }
        }

        let mut digit_0: &Vec<char> = &vec![];
        for digit in &digits_0_6_9 {
            if digit_6.iter().all(|c| digit.contains(c)) || digit_9.iter().all(|c| digit.contains(c)){
                continue;
            }

            digit_0 = digit;
        }

        let digits = vec![digit_0, digit_1, digit_2, digit_3, digit_4, digit_5, digit_6, digit_7, digit_8, digit_9];

        for i in 10..line.len() {
            let output = &line[i];

            for j in 0..=9 {
                if output.iter().all(|c| output.len() == digits[j].len() && digits[j].contains(c)) {
                    sum_output += j * usize::pow(10, i32::abs(i as i32 - (line.len() as i32 - 1)) as u32);
                    break;
                }
            }
        }
    }

    sum_output as u32
}