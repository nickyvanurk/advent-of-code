use std::collections::VecDeque;

pub fn part1(input: &String) -> u32 {
    let input = input.lines();

    for line in input {
        let mut l = String::from(line);
        while explode(&mut l) {}

        println!("{:?}", l);
    }

    0
}

pub fn part2(input: &String) -> u32 {
    let input = input.lines();

    0
}

fn explode(line: &mut String) -> bool {
    let mut start_idx = 0;
    let mut depth = 0;

    let mut numbers: VecDeque<u32> = line.matches(char::is_numeric).map(|c| c.parse::<u32>().unwrap()).collect();
    let mut prev_numbers = vec![];

    for (i, c) in line.clone().chars().enumerate() {    
        if c == '[' {
            start_idx = i;
            depth += 1;
        }
        
        if c == ']' {
            depth -= 1;
            
            if (i - start_idx) == 4 && depth >= 4 {
                let n2 = prev_numbers.pop().unwrap();
                let n1 = prev_numbers.pop().unwrap();
                let s = format!("[{},{}]", n1, n2);
                *line = line.replace(&s[0..], "0");

                if let Some(next) = numbers.pop_front() {
                    let mut l = String::from(&line[0..i-2]);
                    l.push_str(&line[i-2..].replacen(char::is_numeric, &format!("{}", next + n2)[0..], 1));
                    *line = l;
                }

                if let Some(prev) = prev_numbers.last() {
                    if let Some(pos) = String::from(&line[0..start_idx]).rfind(&format!("{}", prev)) {
                        let mut l = String::from(&line[..start_idx]);
                        l.replace_range(pos..start_idx, &format!("{}", prev + n1)[0..]);
                        l.push_str(&line[pos+1..]);
                        *line = l;
                    }
                }

                return true
            }
        }

        if c.is_digit(10) {
            let d = numbers.pop_front().unwrap();
            prev_numbers.push(d);
        }
    }

    false
}